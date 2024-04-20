use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use tokio::{fs, net::TcpListener};
use tower_http::services::ServeDir;
use tracing::info;

#[derive(Debug)]
pub struct AppState {
    pub dir: PathBuf,
}

pub async fn process_server(dir: impl AsRef<std::path::Path>, port: u16) -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Listening on {}", addr);

    let state = Arc::new(AppState {
        dir: dir.as_ref().to_path_buf(),
    });

    let serve_dir = ServeDir::new(dir.as_ref())
        .append_index_html_on_directories(true)
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_gzip()
        .precompressed_zstd();

    let app = Router::new()
        .route("/*path", get(file_handler))
        .nest_service("/tower", serve_dir)
        .with_state(state);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let path = std::path::Path::new(&state.dir).join(path.trim_start_matches('/'));

    if !path.exists() {
        (StatusCode::NOT_FOUND, "Not Found".into())
    } else {
        match fs::read_to_string(path).await {
            Ok(content) => (StatusCode::OK, content),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(AppState {
            dir: PathBuf::from("."),
        });

        let (status, content) =
            file_handler(State(state.clone()), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.starts_with("[package]"));

        let (status, content) =
            file_handler(State(state.clone()), Path("not_found.txt".to_string())).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(content, "Not Found");
    }
}

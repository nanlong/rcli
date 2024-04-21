use std::{
    net::{Ipv4Addr, SocketAddr},
    path::PathBuf,
    sync::Arc,
};

use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
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
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
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
    Path(path_arg): Path<String>,
) -> (StatusCode, HeaderMap, String) {
    let path = std::path::Path::new(&state.dir).join(path_arg.trim_start_matches('/'));
    let mut headers = HeaderMap::new();

    if path.is_dir() {
        match path.read_dir() {
            Ok(entries) => {
                let entries = entries
                    .filter_map(|entry| {
                        entry.ok().map(|entry| {
                            let path = entry.path();
                            let name = path.file_name().unwrap().to_string_lossy().to_string();
                            let is_dir = path.is_dir();
                            (name, is_dir)
                        })
                    })
                    .collect::<Vec<_>>();

                let html = entries
                    .iter()
                    .map(|(name, is_dir)| {
                        let src = std::path::Path::new(&path_arg).join(name);
                        let link = format!("<a href=\"/{}\">{}</a>", src.to_string_lossy(), name);
                        format!("<li>{} {}</li>", if *is_dir { "(dir)" } else { "" }, link)
                    })
                    .collect::<Vec<_>>()
                    .join("");

                let html = format!(
                    r#"
                    <!DOCTYPE html>
                    <html>
                        <head>
                            <title>Index of {path}</title>
                        </head>
                        <body>
                            <h1>Index of {path}</h1>
                            <ul>
                                {html}
                            </ul>
                        </body>
                    </html>
                    "#,
                    path = path.display(),
                    html = html
                );
                headers.insert("Content-Type", "text/html".parse().unwrap());
                (StatusCode::OK, headers, html)
            }
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, headers, e.to_string()),
        }
    } else if !path.exists() {
        (StatusCode::NOT_FOUND, headers, "Not Found".into())
    } else {
        match fs::read_to_string(path).await {
            Ok(content) => (StatusCode::OK, headers, content),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, headers, e.to_string()),
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

        let (status, _, content) =
            file_handler(State(state.clone()), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.starts_with("[package]"));

        let (status, _, content) =
            file_handler(State(state.clone()), Path("not_found.txt".to_string())).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(content, "Not Found");
    }
}

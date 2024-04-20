use clap::Parser;

#[derive(Debug, Parser)]
pub enum HttpSubcommand {
    #[command(name = "server", about = "Start an HTTP static file server")]
    Server(ServerOpts),
}

#[derive(Debug, Parser)]
pub struct ServerOpts {
    #[arg(short, long, help = "directory to serve", default_value = ".")]
    pub dir: String,
    #[arg(short, long, help = "port to listen on", default_value = "8080")]
    pub port: u16,
}

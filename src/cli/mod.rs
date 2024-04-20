mod base64;
mod csv;
mod genpass;
mod http;
mod jwt;
mod text;

pub use base64::*;
use clap::Parser;
pub use csv::*;
pub use genpass::*;
pub use http::*;
pub use jwt::*;
pub use text::*;

#[derive(Debug, Parser)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    Genpass(GenpassOpts),
    #[command(subcommand, about = "Encode or decode base64")]
    Base64(Base64Subcommand),
    #[command(subcommand, about = "Text sign/verify")]
    Text(TextSubcommand),
    #[command(subcommand, about = "Http server")]
    Http(HttpSubcommand),
    #[command(subcommand, about = "JWT sign/verify")]
    JWT(JwtSubcommand),
}

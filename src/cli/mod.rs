mod base64;
mod csv;
mod genpass;
mod http;
mod jwt;
mod text;

pub use base64::*;
pub use csv::*;
pub use genpass::*;
pub use http::*;
pub use jwt::*;
pub use text::*;

use anyhow::Result;
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
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

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExector {
    async fn execute(self) -> Result<()>;
}

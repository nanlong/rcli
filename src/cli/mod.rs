mod base64;
mod csv;
mod genpass;
mod text;

pub use base64::*;
use clap::Parser;
pub use csv::*;
pub use genpass::*;
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
}

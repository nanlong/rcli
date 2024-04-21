use crate::{verify_input, CmdExector};

use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum Base64Subcommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(EncodeOpts),
    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct EncodeOpts {
    #[arg(short, long, value_parser = verify_input, default_value = "-", help = "file path or enter it manually")]
    pub input: String,
    #[arg(
        short,
        long,
        default_value = "standard",
        help = "base64 format: standard or urlsafe"
    )]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct DecodeOpts {
    #[arg(short, long, value_parser = verify_input, default_value = "-", help = "file path or enter it manually")]
    pub input: String,
    #[arg(
        short,
        long,
        default_value = "standard",
        help = "base64 format: standard or urlsafe"
    )]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl std::str::FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid base64 format")),
        }
    }
}

impl CmdExector for EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = crate::get_reader(&self.input)?;
        let encoded =
            crate::process_base64(&mut reader, &self.format, crate::Base64Action::Encode)?;
        println!("{}", encoded);
        Ok(())
    }
}

impl CmdExector for DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = crate::get_reader(&self.input)?;
        let decoded =
            crate::process_base64(&mut reader, &self.format, crate::Base64Action::Decode)?;
        println!("{}", decoded);
        Ok(())
    }
}

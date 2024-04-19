use crate::verify_input;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum TextSubcommand {
    #[command(name = "sign", about = "Sign a message")]
    Sign(SignOpts),
    #[command(name = "verify", about = "Verify a signed message")]
    Verify(VerifyOpts),
    #[command(name = "generate", about = "Generate a key")]
    Generate(GenerateOpts),
}

#[derive(Debug, Parser)]
pub struct SignOpts {
    #[arg(short, long, value_parser = verify_input, default_value = "-")]
    pub input: String,
    #[arg(short, long, help = "key to sign with")]
    pub key: String,
    #[arg(short, long, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct VerifyOpts {
    #[arg(short, long, value_parser = verify_input, default_value = "-")]
    pub input: String,
    #[arg(short, long, help = "key to verify with")]
    pub key: String,
    #[arg(short, long, help = "signature to verify")]
    pub sig: String,
    #[arg(short, long, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct GenerateOpts {
    #[arg(short, long, help = "output file", default_value = "-")]
    pub output: String,
    #[arg(short, long, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

impl std::str::FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid text sign format")),
        }
    }
}

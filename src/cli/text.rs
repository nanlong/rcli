use crate::{verify_input, CmdExector};

use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum TextSubcommand {
    #[command(name = "sign", about = "Sign a message")]
    Sign(SignOpts),
    #[command(name = "verify", about = "Verify a signed message")]
    Verify(VerifyOpts),
    #[command(name = "generate", about = "Generate a key")]
    Generate(GenerateOpts),
    #[command(name = "encrypt", about = "Encrypt a message")]
    Encrypt(EncryptOpts),
    #[command(name = "decrypt", about = "Decrypt a message")]
    Decrypt(DecryptOpts),
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

#[derive(Debug, Parser)]
pub struct EncryptOpts {
    #[arg(short, long, value_parser = verify_input, default_value = "-")]
    pub input: String,
    #[arg(short, long, help = "key to encrypt with")]
    pub key: String,
    #[arg(short, long, help = "output file", default_value = "-")]
    pub output: String,
}

#[derive(Debug, Parser)]
pub struct DecryptOpts {
    #[arg(short, long, value_parser = verify_input, default_value = "-")]
    pub input: String,
    #[arg(short, long, help = "key to decrypt with")]
    pub key: String,
    #[arg(short, long, help = "output file", default_value = "-")]
    pub output: String,
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

impl CmdExector for SignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = crate::get_reader(&self.input)?;
        let key = crate::get_content(&self.key)?;
        let signature = crate::process_sign(&mut reader, &key, self.format)?;
        println!("{}", signature);
        Ok(())
    }
}

impl CmdExector for VerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = crate::get_reader(&self.input)?;
        let key = crate::get_content(&self.key)?;
        let sig = crate::get_content(&self.sig)?;
        let verified = crate::process_verify(&mut reader, &key, &sig, self.format)?;
        println!("{}", verified);
        Ok(())
    }
}

impl CmdExector for GenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = crate::process_generate(self.format);
        crate::output_contents(&self.output, &key);
        Ok(())
    }
}

impl CmdExector for EncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = crate::get_reader(&self.input)?;
        let key = crate::get_content(&self.key)?;
        let encrypted = crate::process_encrypt(&mut reader, &key)?;
        crate::output_contents(&self.output, &encrypted);
        Ok(())
    }
}

impl CmdExector for DecryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = crate::get_reader(&self.input)?;
        let key = crate::get_content(&self.key)?;
        let decrypted = crate::process_decrypt(&mut reader, &key)?;
        crate::output_contents(&self.output, &decrypted);
        Ok(())
    }
}

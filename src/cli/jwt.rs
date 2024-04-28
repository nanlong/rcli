use crate::CmdExector;

use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum JwtSubcommand {
    #[command(name = "sign", about = "Sign a JWT")]
    Sign(JWTEncodeOpts),
    #[command(name = "verify", about = "Verify a JWT")]
    Verify(JWTDecodeOpts),
}

#[derive(Debug, Parser)]
pub struct JWTEncodeOpts {
    #[arg(short, long, help = "key to sign with", default_value = "secret")]
    pub key: String,
    #[arg(short, long, help = "audience")]
    pub aud: Option<String>,
    #[arg(short, long, help = "subject", default_value = "1d")]
    pub exp: String,
    #[arg(short, long, help = "issuer")]
    pub iss: Option<String>,
    #[arg(short, long, help = "subject")]
    pub sub: Option<String>,
}

#[derive(Debug, Parser)]
pub struct JWTDecodeOpts {
    #[arg(short, long, help = "key to verify with", default_value = "secret")]
    pub key: String,
    #[arg(short, long, help = "token to verify")]
    pub token: String,
    #[arg(short, long, help = "audience")]
    pub aud: Option<String>,
}

impl CmdExector for JWTEncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let token = crate::process_jwt_sign(&self.key, &self.exp, self.aud, self.iss, self.sub)?;

        println!("Sign JWT: \n{}", token);

        Ok(())
    }
}

impl CmdExector for JWTDecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let verified = crate::process_jwt_verify(&self.key, &self.token, self.aud);
        println!("Verify JWT: {}", verified.is_ok());

        Ok(())
    }
}

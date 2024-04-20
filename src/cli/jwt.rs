use clap::Parser;

#[derive(Debug, Parser)]
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
    #[arg(short, long, help = "audience", default_value = "-")]
    pub aud: String,
    #[arg(short, long, help = "subject", default_value = "1d")]
    pub exp: String,
    #[arg(short, long, help = "issuer", default_value = "-")]
    pub iss: String,
    #[arg(short, long, help = "subject", default_value = "-")]
    pub sub: String,
}

#[derive(Debug, Parser)]
pub struct JWTDecodeOpts {
    #[arg(short, long, help = "key to verify with", default_value = "secret")]
    pub key: String,
    #[arg(short, long, help = "token to verify")]
    pub token: String,
    #[arg(short, long, help = "audience", default_value = "-")]
    pub aud: String,
}

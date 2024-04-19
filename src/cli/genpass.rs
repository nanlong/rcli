use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenpassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,
    #[arg(long, default_value_t = false, action = clap::ArgAction::SetTrue)]
    pub no_upper: bool,
    #[arg(long, default_value_t = false, action = clap::ArgAction::SetTrue)]
    pub no_lower: bool,
    #[arg(long, default_value_t = false, action = clap::ArgAction::SetTrue)]
    pub no_num: bool,
    #[arg(long, default_value_t = false, action = clap::ArgAction::SetTrue)]
    pub no_symbol: bool,
}

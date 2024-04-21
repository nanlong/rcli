use crate::CmdExector;

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

impl CmdExector for GenpassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let password = crate::process_genpass(
            self.length,
            self.no_upper,
            self.no_lower,
            self.no_num,
            self.no_symbol,
        )?;

        println!("{}", password);
        let estimate = zxcvbn::zxcvbn(&password, &[])?;
        eprintln!("Estimated strength: {}", estimate.score());

        Ok(())
    }
}

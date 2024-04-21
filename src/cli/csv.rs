use crate::{verify_input, CmdExector};
use clap::Parser;

#[derive(Debug, Parser, Clone)]
pub enum OutputFormat {
    Raw,
    Json,
    Yaml,
    Toml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input)]
    pub input: String,
    #[arg(short, long, default_value = "-")]
    pub output: String,
    #[arg(short, long, default_value = "raw")]
    pub format: OutputFormat,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = false, action = clap::ArgAction::SetTrue)]
    pub no_header: bool,
}

impl std::str::FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "raw" => Ok(OutputFormat::Raw),
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            _ => Err(anyhow::anyhow!("Invalid CSV output format")),
        }
    }
}

impl CmdExector for CsvOpts {
    async fn execute(self) -> anyhow::Result<()> {
        crate::process_csv(
            &self.input,
            &self.output,
            self.format,
            self.delimiter,
            self.no_header,
        )?;
        Ok(())
    }
}

use clap::Parser;
use crate::models::LogLevel;
use std::path::PathBuf;
use anyhow::{ Result };

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    pub file_path: PathBuf,

    #[arg(long, short = 'l', value_enum, ignore_case = true)]
    pub level: Option<LogLevel>,

    #[arg(long)]
    pub sciter: bool,
}

impl Cli {
    pub fn parse() -> Result<Self> {
        let args = <Self as clap::Parser>::parse();
        args.validate()?;
        Ok(args)
    }

    fn validate(&self) -> Result<()> {
        if self.sciter && self.level.is_some() {
            anyhow::bail!("--sciter cannot be used with --level");
        }

        if !self.file_path.is_file() {
            anyhow::bail!("Expected a file path, got '{}'", self.file_path.display());
        }

        Ok(())
    }
}

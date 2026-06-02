use clap::Parser;
use crate::logger::LogLevel;
use std::path::PathBuf;
use anyhow::{Result};

fn parse_positive(s: &str) -> Result<usize> {
    let n: usize = s
        .parse()
        .map_err(|_| anyhow::anyhow!("`{s}` is not a valid positive integer"))?;

    if n == 0 {
        anyhow::bail!("`{s}` must be greater than 0");
    }

    Ok(n)
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    pub file_path: PathBuf,

    #[arg(long, short = 'l', value_enum, ignore_case = true)]
    pub level: Option<LogLevel>,

    #[arg(long)]
    pub save: bool,

    #[arg(long, value_parser = parse_positive)]
    pub first: Option<usize>,

    #[arg(long, value_parser = parse_positive)]
    pub last: Option<usize>,
}

impl Cli {
    pub fn parse() -> Result<Self> {
        let args = <Self as clap::Parser>::parse();
        args.validate()?;
        Ok(args)
    }

    fn validate(&self) -> Result<()> {
        if self.first.is_some() && self.last.is_some() {
            anyhow::bail!("--first and --last cannot be used together");
        }

        if !self.file_path.is_file() {
            anyhow::bail!("Expected a file path, got '{}'", self.file_path.display());
        }

        Ok(())
    }
}
use clap::Parser;
use crate::parser::LogLevel;
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

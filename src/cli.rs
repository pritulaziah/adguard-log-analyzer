use clap::Parser;
use crate::parser::LogLevel;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    pub file_path: PathBuf,

    #[arg(long, short = 'l', value_enum, ignore_case = true)]
    pub level: Option<LogLevel>,

    #[arg(long)]
    pub save: bool,
}

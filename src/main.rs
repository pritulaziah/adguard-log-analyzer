use clap::Parser;
use anyhow::Result;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

mod parser;
use parser::{LogLevel, LogParser};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    file_path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let file = File::open(&args.file_path)?;
    let reader = BufReader::new(file);

    let mut counts: HashMap<LogLevel, u32> = HashMap::new();

    for (i, line) in reader.lines().enumerate() {
        let mut line = line?;

        if i == 0 {
            line = line.trim_start_matches('\u{feff}').to_string();
        }

        if let Some(entry) = LogParser::parse_line(&line) {
            *counts.entry(entry.level).or_insert(0) += 1;
        }
    }

    for (level, count) in &counts {
        println!("{:?} {}", level, count);
    }

    Ok(())
}

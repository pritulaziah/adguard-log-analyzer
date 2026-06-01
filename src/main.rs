use clap::Parser;
use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

mod parser;
use parser::{LogLevel, LogParser, LogEntry};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    file_path: PathBuf,
    #[arg(long, ignore_case = true)]
    level: Option<LogLevel>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let file = File::open(&args.file_path)?;
    let reader = BufReader::new(file);

    let mut logs: Vec<LogEntry> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let mut line = line?;

        if i == 0 {
            line = line.trim_start_matches('\u{feff}').to_string();
        }

        if let Some(entry) = LogParser::parse_line(&line) {
            logs.push(entry);
        }
    }

    let iter_logs = logs.iter();

    if args.level.is_some() {
        let current_level = args.level.clone().unwrap();

        let results: Vec<&LogEntry> = iter_logs
            .filter(|log_entry| log_entry.level == current_level)
            .collect();

        for entry in &results {
            println!("{:?}, {}", entry.timestamp, entry.message);
        }
    } else {
        for entry in iter_logs {
            println!("{:?}, {:?}, {}", entry.level, entry.timestamp, entry.message);
        }
    }

    Ok(())
}

use clap::Parser;
use anyhow::{Result};
use std::{
    fs::{File, write, create_dir_all},
    io::{BufRead, BufReader},
};
mod parser;
use parser::{LogParser, LogEntry};
mod cli;
use cli::Cli;

fn main() -> Result<()> {
    let args = Cli::parse();
    let level = args.level.as_ref();
    let file_path = &args.file_path;

    if !file_path.is_file() {
        anyhow::bail!("Expected a file path, got '{}'", file_path.display());
    }

    let file = File::open(file_path)?;
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

    let mut output: Vec<String> = Vec::new();

    for entry in logs.iter() {
        if let Some(filter) = level {
            if filter != &entry.level {
                continue;
            }
        }

        output.push(format!("{:?}, {:?}, {}", entry.level, entry.timestamp, entry.message));
    }

    if args.save {
        create_dir_all("result_logs")?;
        let level_logs = match level {
            Some(level) => level.to_string(),
            None => "all".to_string(),
        };
        let output_path = file_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let to_save = format!("result_logs/{}_{}", level_logs, output_path);
        write(&to_save, output.join("\n"))?;
    } else {
        println!("{}", output.join("\n"));
    }

    Ok(())
}

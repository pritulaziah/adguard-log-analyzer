use clap::Parser;
use anyhow::{Result};
use std::{fs};
mod parser;
mod cli;
use cli::Cli;
mod reader;
use reader::{LogReader};

fn main() -> Result<()> {
    let args = Cli::parse();
    let level = args.level.as_ref();
    let file_path = &args.file_path;

    if !file_path.is_file() {
        anyhow::bail!("Expected a file path, got '{}'", file_path.display());
    }

    let logs = LogReader::read_logs(file_path)?;

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
        fs::create_dir_all("result_logs")?;
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
        fs::write(&to_save, output.join("\n"))?;
    } else {
        println!("{}", output.join("\n"));
    }

    Ok(())
}

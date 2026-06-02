use anyhow::{Result};
use std::{fs};
mod cli;
use cli::Cli;
mod logger;
use logger::{Logger};
mod models;

fn main() -> Result<()> {
    let args = Cli::parse()?;
    let level = args.level.as_ref();
    let file_path = &args.file_path;
    let save = args.save;
    let first = args.first;
    let last = args.last;

    let logs = Logger::read_logs(file_path)?;

    let mut output: Vec<String> = Vec::new();

    for entry in logs.iter() {
        if let Some(filter) = level {
            if filter != &entry.level {
                continue;
            }
        }

        output.push(format!("{:?}, {:?}, {}", entry.level, entry.timestamp, entry.message));
    }

    if let Some(n) = first {
        output.truncate(n);
    } else if let Some(n) = last {
        let len = output.len();
        if n < len {
            output.drain(0..len - n);
        }
    }

    if save {
        let log_level = match level {
            Some(level) => level.to_string(),
            None => "all".to_string(),
        };
        let output_path = file_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let folder = format!("result_logs/{}", output_path);
        fs::create_dir_all(&folder)?;

        let mut parts = vec![format!("{}/{}", folder, log_level)];

        if let Some(n) = first {
            parts.push(format!("first{}", n));
        } else if let Some(n) = last {
            parts.push(format!("last{}", n));
        }

        fs::write(format!("{}.log", parts.join("_")), output.join("\n"))?;
    } else {
        println!("{}", output.join("\n"));
    }

    Ok(())
}

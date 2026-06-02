use crate::{ cli::Cli, models::{ LogEntry } };
use std::fs;
use anyhow::Result;

pub struct Formatter;

impl Formatter {
    pub fn output_logs(logs: Vec<LogEntry>, opts: &Cli) -> Result<()> {
        let output = logs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        if opts.save {
            let log_level = match &opts.level {
                Some(level) => level.to_string(),
                None => "all".to_string(),
            };
            let output_path = opts.file_path.file_name().unwrap().to_string_lossy().to_string();

            let folder = format!("result_logs/{}", output_path);
            fs::create_dir_all(&folder)?;

            let mut parts = vec![format!("{}/{}", folder, log_level)];

            if let Some(n) = opts.first {
                parts.push(format!("first{}", n));
            } else if let Some(n) = opts.last {
                parts.push(format!("last{}", n));
            }

            fs::write(format!("{}.log", parts.join("_")), output)?;
        } else {
            println!("{}", output);
        }

        Ok(())
    }
}

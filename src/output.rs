use crate::{ cli::Cli, models::{ LogEntry, LogLevel } };
use std::{ collections::HashMap, fs };
use anyhow::{ Ok, Result };

fn format_duration(duration: chrono::Duration) -> String {
    let total_seconds = duration.num_seconds();

    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!("{hours}h {minutes}m {seconds}s")
}

pub fn output_logs(logs: Vec<LogEntry>, opts: &Cli) -> Result<()> {
    if opts.stats {
        let len_logs = logs.len();
        let mut counts = HashMap::new();

        for entry in &logs {
            *counts.entry(entry.level.clone()).or_insert(0) += 1;
        }

        let first_log = logs.first().map(|entry| entry.timestamp);
        let last_log = logs.last().map(|entry| entry.timestamp);

        let duration = match (first_log, last_log) {
            (Some(first), Some(last)) => Some(last - first),
            _ => None,
        };

        println!("Total logs: {}", len_logs);
        println!();
        for level in &[LogLevel::Info, LogLevel::Error, LogLevel::Warning, LogLevel::Verbose] {
            println!("{} {}", level.colored_label(), counts.get(level).unwrap_or(&0));
        }

        if
            let (Some(first), Some(last), Some(duration)) = (first_log, last_log, duration) &&
            len_logs > 1
        {
            println!();
            println!("First log:  {}", first.format("%d.%m.%Y %H:%M:%S%.3f"));
            println!("Last log:   {}", last.format("%d.%m.%Y %H:%M:%S%.3f"));
            println!("Duration:   {}", format_duration(duration));
        }

        return Ok(());
    }

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

        let mut parts = vec![format!("{}/{}", folder, log_level.to_lowercase())];

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

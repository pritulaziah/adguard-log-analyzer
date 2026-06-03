use crate::{ cli::Cli, models::{ LogEntry, LogLevel } };
use std::{ collections::HashMap, fs };
use anyhow::Result;

fn format_duration(duration: chrono::Duration) -> String {
    let total_secs = duration.num_seconds();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    format!("{hours}h {minutes}m {seconds}s")
}

pub fn output_logs(entries: Vec<LogEntry>, opts: &Cli) -> Result<()> {
    if opts.stats {
        let total = entries.len();
        println!("TOTAL LOGS: {}", total);

        let mut level_counts = HashMap::new();

        for entry in &entries {
            *level_counts.entry(entry.level.clone()).or_insert(0) += 1;

            if
                let Some(module_info) = entry.message.strip_prefix(
                    "Application version and installed modules info:\n"
                )
            {
                for line in module_info.split('\n') {
                    let head = line.split(',').next();

                    if let Some(head) = head {
                        let cleaned = head.trim().replacen(": version is", ":", 1);
                        let parts: Vec<&str> = cleaned.split(": ").collect();

                        if parts.len() >= 2 {
                            println!("  {}: {}", parts[0], parts[1]);
                        }
                    }
                }
            }
        }

        let first_ts = entries.first().map(|e| e.timestamp);
        let last_ts = entries.last().map(|e| e.timestamp);

        let span = match (first_ts, last_ts) {
            (Some(first), Some(last)) => Some(last - first),
            _ => None,
        };

        println!();
        for level in &[LogLevel::Info, LogLevel::Error, LogLevel::Warning, LogLevel::Verbose] {
            println!("{} {}", level.colored_label(), level_counts.get(level).unwrap_or(&0));
        }

        if let (Some(first), Some(last), Some(span)) = (first_ts, last_ts, span) {
            if total > 1 {
                println!();
                println!("First log:  {}", first.format("%d.%m.%Y %H:%M:%S%.3f"));
                println!("Last log:   {}", last.format("%d.%m.%Y %H:%M:%S%.3f"));
                println!("Duration:   {}", format_duration(span));
            }
        }

        return Ok(());
    }

    let output = entries
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    if opts.save {
        let level_name = match &opts.level {
            Some(lvl) => lvl.to_string(),
            None => "all".to_string(),
        };
        let filename = opts.file_path.file_name().unwrap().to_string_lossy().to_string();

        let out_dir = format!("result_logs/{}", filename);
        fs::create_dir_all(&out_dir)?;

        let mut segments = vec![format!("{}/{}", out_dir, level_name.to_lowercase())];

        if let Some(n) = opts.first {
            segments.push(format!("first{}", n));
        } else if let Some(n) = opts.last {
            segments.push(format!("last{}", n));
        }

        fs::write(format!("{}.log", segments.join("_")), output)?;
    } else {
        println!("{}", output);
    }

    Ok(())
}

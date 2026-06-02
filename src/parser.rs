use anyhow::Result;
use std::{ fs::File, io::{ BufRead, BufReader }, path::Path };
use chrono::NaiveDateTime;
use crate::models::{ LogEntry, LogLevel };

pub fn parse_file<P: AsRef<Path>>(file_path: P) -> Result<Vec<LogEntry>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut logs: Vec<LogEntry> = Vec::new();
    let mut current: Option<String> = None;

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let line = if i == 0 { line.trim_start_matches('\u{feff}') } else { &line };

        if is_log_start(line) {
            if let Some(prev) = current.take() {
                if let Some(entry) = parse_line(&prev) {
                    logs.push(entry);
                }
            }
            current = Some(line.to_string());
        } else if let Some(ref mut buf) = current {
            buf.push('\n');
            buf.push_str(line);
        }
    }

    if let Some(prev) = current {
        if let Some(entry) = parse_line(&prev) {
            logs.push(entry);
        }
    }

    Ok(logs)
}

fn is_log_start(line: &str) -> bool {
    line.split(", ")
        .next()
        .and_then(|word| word.parse::<LogLevel>().ok())
        .is_some()
}

pub fn parse_line(line: &str) -> Option<LogEntry> {
    let parts: Vec<&str> = line.splitn(6, ", ").collect();

    if parts.len() != 6 {
        return None;
    }

    let level = match parts[0] {
        "INFO" => LogLevel::Info,
        "VERBOSE" => LogLevel::Verbose,
        "WARNING" => LogLevel::Warning,
        "ERROR" => LogLevel::Error,
        _ => {
            return None;
        }
    };

    let process = parts[1].to_string();
    let logger = parts[2].to_string();
    let thread_id = parts[3].parse().ok()?;
    let timestamp = NaiveDateTime::parse_from_str(parts[4], "%d.%m.%Y %H:%M:%S%.3f").ok()?;
    let message = parts[5].to_string();

    Some(LogEntry {
        level,
        process,
        logger,
        thread_id,
        timestamp,
        message,
    })
}

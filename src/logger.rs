use anyhow::Result;
use std::{
    fs::File,
    io::{ BufRead, BufReader },
    path::Path,
    fmt::{ Display, Formatter, Result as FmtResult },
    str::FromStr,
};
use chrono::NaiveDateTime;

#[derive(clap::ValueEnum, Debug, Hash, PartialEq, Eq, Clone)]
pub enum LogLevel {
    Verbose,
    Info,
    Warning,
    Error,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            LogLevel::Verbose => write!(f, "verbose"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warning => write!(f, "warning"),
            LogLevel::Error => write!(f, "error"),
        }
    }
}

impl FromStr for LogLevel {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "INFO" => Ok(LogLevel::Info),
            "VERBOSE" => Ok(LogLevel::Verbose),
            "WARNING" => Ok(LogLevel::Warning),
            "ERROR" => Ok(LogLevel::Error),
            _ => Err(format!("unknown level: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub level: LogLevel,
    #[allow(dead_code)]
    pub process: String,
    #[allow(dead_code)]
    pub logger: String,
    #[allow(dead_code)]
    pub thread_id: u32,
    pub timestamp: NaiveDateTime,
    pub message: String,
}

pub struct Logger;

impl Logger {
    pub fn read_logs<P: AsRef<Path>>(file_path: P) -> Result<Vec<LogEntry>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut logs: Vec<LogEntry> = Vec::new();
        let mut current: Option<String> = None;

        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            let line = if i == 0 { line.trim_start_matches('\u{feff}') } else { &line };

            if Self::is_log_start(line) {
                if let Some(prev) = current.take() {
                    if let Some(entry) = Self::parse_line(&prev) {
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
            if let Some(entry) = Self::parse_line(&prev) {
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
}

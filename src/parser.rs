use chrono::NaiveDateTime;
use std::fmt::{Display, Formatter, Result as FmtResult};

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

pub struct LogParser;

impl LogParser {
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
            _ => return None,
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
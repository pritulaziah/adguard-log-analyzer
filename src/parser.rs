use chrono::NaiveDateTime;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum LogLevel {
    Verbose,
    Info,
    Warning,
    Error,
    Unknown(String),
}

#[derive(Debug)]
pub struct LogEntry {
    pub level: LogLevel,
    pub process: String,
    pub logger: String,
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
            _ => LogLevel::Unknown(parts[0].to_string()),
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
use std::fmt::{ Display, Formatter, Result as FmtResult };
use std::str::FromStr;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(clap::ValueEnum, Debug, Hash, PartialEq, Eq, Clone, Serialize)]
pub enum LogLevel {
    Verbose,
    Info,
    Warning,
    Error,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            LogLevel::Verbose => write!(f, "VERBOSE"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARNING"),
            LogLevel::Error => write!(f, "ERROR"),
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

#[derive(Debug, Clone, Serialize)]
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
    pub is_sciter_message: bool,
}

impl Display for LogEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}, {}",
            self.level,
            format!("{:?}, {}", self.timestamp, self.message)
        )
    }
}

#[derive(Serialize)]
pub struct JsonLogEntry {
    pub timestamp: NaiveDateTime,
    pub level: LogLevel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
}

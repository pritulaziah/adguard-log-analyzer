use std::fmt::{ Display, Formatter, Result as FmtResult };
use std::str::FromStr;
use chrono::NaiveDateTime;
use colored::*;

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

impl LogLevel {
    pub fn colored_label(&self) -> ColoredString {
        let label = format!("{:<12}", self.to_string());
        match self {
            LogLevel::Info => label.green(),
            LogLevel::Error => label.red(),
            LogLevel::Warning => label.yellow(),
            LogLevel::Verbose => label.blue(),
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
    pub is_sciter_message: bool,
}

impl Display for LogEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{} {}",
            self.level,
            format!("{:?}, {}", self.timestamp, self.message)
        )
    }
}

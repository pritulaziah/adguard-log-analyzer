use std::fmt::{ Display, Formatter, Result as FmtResult };
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct LogEntry {
    pub level: String,
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
    pub level: String,
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

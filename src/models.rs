use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct LogEntry {
    pub timestamp: NaiveDateTime,
    pub message: String,
    pub service: Option<String>,
    pub method: Option<String>,
    pub event_type: Option<String>,
    pub payload: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct JsonCall {
    pub method: String,
    pub event_type: String,
    pub timestamp: NaiveDateTime,
    pub raw: String,
    pub payload: serde_json::Value,
}

#[derive(Serialize)]
pub struct JsonServiceGroup {
    pub service: String,
    pub calls: Vec<JsonCall>,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum JsonOutputEntry {
    Service(JsonServiceGroup),
    Raw(JsonRawLogEntry),
}

#[derive(Serialize)]
pub struct JsonRawLogEntry {
    pub timestamp: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

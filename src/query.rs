use crate::{ models::{ LogEntry } };

pub fn query_logs(logs: Vec<LogEntry>) -> Vec<LogEntry> {
    logs
        .iter()
        .filter(|e| e.is_sciter_message)
        .cloned()
        .collect()
}

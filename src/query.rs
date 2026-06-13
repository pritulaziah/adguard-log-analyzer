use crate::{ models::{ LogEntry } };

pub fn query_logs(logs: Vec<LogEntry>) -> Vec<LogEntry> {
    logs
        .iter()
        .cloned()
        .collect()
}

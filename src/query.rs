use crate::{cli::Cli, models::LogEntry};

pub fn query_logs(logs: Vec<LogEntry>, opts: &Cli) -> Vec<LogEntry> {
    logs.iter()
        .cloned()
        .filter(|entry| {
            opts.service
                .as_ref()
                .is_none_or(|service| entry.service.as_ref() == Some(service))
        })
        .filter(|entry| opts.from.is_none_or(|from| entry.timestamp >= from))
        .filter(|entry| opts.to.is_none_or(|to| entry.timestamp <= to))
        .collect()
}

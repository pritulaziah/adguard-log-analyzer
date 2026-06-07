use crate::{ cli::Cli, models::{ LogEntry } };

pub fn query_logs(logs: Vec<LogEntry>, opts: &Cli) -> Vec<LogEntry> {
    let mut selected: Vec<LogEntry> = logs
        .iter()
        .filter(|entry| {
            if let Some(filter) = &opts.level { filter == &entry.level } else { true }
        })
        .cloned()
        .collect();

    if opts.sciter {
        selected.retain(|e| e.is_sciter_message);
    }

    selected
}

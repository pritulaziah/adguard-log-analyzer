use crate::{ cli::Cli, models::{ LogEntry } };

pub fn query_logs(logs: Vec<LogEntry>, opts: &Cli) -> Vec<LogEntry> {
    let mut selected: Vec<LogEntry> = logs
        .iter()
        .filter(|entry| {
            if let Some(filter) = &opts.level { filter == &entry.level } else { true }
        })
        .cloned()
        .collect();

    if let Some(n) = opts.first {
        selected.truncate(n);
    } else if let Some(n) = opts.last {
        let len = selected.len();
        selected.drain(0..len.saturating_sub(n));
    }

    selected
}

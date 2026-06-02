pub struct LogQuery;
use crate::models::{LogEntry, LogLevel};

pub struct QueryOptions<'a> {
    pub level: Option<&'a LogLevel>,
    pub first: Option<usize>,
    pub last: Option<usize>,
}

impl LogQuery {
    pub fn query_logs(
        logs: Vec<LogEntry>,
        opts: QueryOptions
    ) -> Vec<LogEntry> {
        let mut output: Vec<LogEntry> = logs
            .iter()
            .filter(|entry| {
                if let Some(filter) = opts.level {

                    filter == &entry.level
                } else {
                    true
                }
            })
            .cloned()
            .collect();

        if let Some(n) = opts.first {
            output.truncate(n);
        } else if let Some(n) = opts.last {
            let len = output.len();
            output.drain(0..len.saturating_sub(n));
        }

        output
    }
}
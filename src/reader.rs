use anyhow::Result;
use std::{ fs::File, io::{ BufRead, BufReader }, path::Path };

use crate::parser::{ LogEntry, LogParser, LogLevel };

pub struct LogReader;

impl LogReader {
    pub fn read_logs<P: AsRef<Path>>(file_path: P) -> Result<Vec<LogEntry>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut logs: Vec<LogEntry> = Vec::new();
        let mut current: Option<String> = None;

        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            let line = if i == 0 {
                line.trim_start_matches('\u{feff}')
            } else {
                &line
            };

            if Self::is_log_start(line) {
                if let Some(prev) = current.take() {
                    if let Some(entry) = LogParser::parse_line(&prev) {
                        logs.push(entry);
                    }
                }
                current = Some(line.to_string());
            } else if let Some(ref mut buf) = current {
                buf.push('\n');
                buf.push_str(line);
            }
        }

        Ok(logs)
    }

    fn is_log_start(line: &str) -> bool {
        line.split(", ")
            .next()
            .and_then(|word| word.parse::<LogLevel>().ok())
            .is_some()
    }
}

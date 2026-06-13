use crate::models::LogEntry;
use anyhow::Result;
use chrono::NaiveDateTime;
use regex::Regex;
use std::sync::LazyLock;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn parse_file<P: AsRef<Path>>(file_path: P) -> Result<Vec<LogEntry>> {
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

        if is_log_start(line) {
            if let Some(prev) = current.take() {
                if let Some(entry) = parse_line(&prev) {
                    logs.push(entry);
                }
            }
            current = Some(line.to_string());
        } else if let Some(ref mut buf) = current {
            buf.push('\n');
            buf.push_str(line);
        }
    }

    if let Some(prev) = current {
        if let Some(entry) = parse_line(&prev) {
            logs.push(entry);
        }
    }

    Ok(logs)
}

static LOG_START_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\w+, Adguard\.exe,").unwrap());

static SCITER_PREFIX_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"SciterMessage, OS_INFO\(OT_TIS\): \d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z DBG: ",
    )
    .unwrap()
});

static SCITER_EVENT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?s)^\[([^\]]+)\]\s+(Callback data|Request data|Response data)\s+\[(.*)\]$")
        .unwrap()
});

fn is_log_start(line: &str) -> bool {
    LOG_START_RE.is_match(line)
}

pub fn parse_line(line: &str) -> Option<LogEntry> {
    let parts: Vec<&str> = line.splitn(6, ", ").collect();

    if parts.len() != 6 {
        return None;
    }

    let timestamp = NaiveDateTime::parse_from_str(parts[4], "%d.%m.%Y %H:%M:%S%.3f").ok()?;
    let mut message = parts[5].trim().to_string();

    if SCITER_PREFIX_RE.is_match(&message) {
        message = SCITER_PREFIX_RE
            .replace(&message, "")
            .replace(":Value has been hidden,", ":\"\",")
            .to_string();

        let (service, method, event_type, payload) =
            if let Some(caps) = SCITER_EVENT_RE.captures(&message) {
                let (service, method) = caps[1]
                    .rsplit_once('.')
                    .map(|(s, m)| (Some(s.to_string()), Some(m.to_string())))
                    .unwrap_or((None, None));
                let payload = serde_json::from_str(&caps[3]).ok();

                (service, method, Some(caps[2].to_string()), payload)
            } else {
                (None, None, None, None)
            };

        return Some(LogEntry {
            timestamp,
            message,
            service,
            method,
            event_type,
            payload,
        });
    }

    return None;
}

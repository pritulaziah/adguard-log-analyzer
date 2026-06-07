use crate::{ cli::Cli, models::{ LogEntry, JsonLogEntry } };
use std::{ fs, sync::LazyLock };
use anyhow::Result;
use regex::Regex;

pub fn output_logs(entries: Vec<LogEntry>, opts: &Cli) -> Result<()> {
    let filename = opts.file_path.file_name().unwrap().to_string_lossy().to_string();
    let out_dir = format!("result_logs/{}", filename);
    fs::create_dir_all(&out_dir)?;

    let json_entries: Vec<JsonLogEntry> = entries
        .iter()
        .map(|e| convert_to_json_entry(e))
        .collect();
    let output_content = serde_json::to_string_pretty(&json_entries)?;

    fs::write(format!("{}/{}.sciter.json", out_dir, filename), output_content)?;

    Ok(())
}

fn convert_to_json_entry(entry: &LogEntry) -> JsonLogEntry {
    if entry.is_sciter_message {
        static SCITER_RE: LazyLock<Regex> = LazyLock::new(||
            Regex::new(r"^\[([^\]]+)\]\s+(\S+\s\S+)\s+\[(.+)\]$").unwrap()
        );

        if let Some(caps) = SCITER_RE.captures(&entry.message) {
            let service_method = &caps[1];
            let (service, method) = service_method
                .rsplit_once('.')
                .map(|(s, m)| (Some(s.to_string()), Some(m.to_string())))
                .unwrap_or((None, None));

            let event_type = caps[2].to_string();
            let payload_str = &caps[3];

            let payload = serde_json::from_str(payload_str).ok();

            JsonLogEntry {
                timestamp: entry.timestamp,
                level: entry.level.clone(),
                message: None,
                service,
                method,
                event_type: Some(event_type),
                payload,
            }
        } else {
            // Fallback: doesn't match expected format
            JsonLogEntry {
                timestamp: entry.timestamp,
                level: entry.level.clone(),
                message: Some(entry.message.clone()),
                service: None,
                method: None,
                event_type: None,
                payload: None,
            }
        }
    } else {
        JsonLogEntry {
            timestamp: entry.timestamp,
            level: entry.level.clone(),
            message: Some(entry.message.clone()),
            service: None,
            method: None,
            event_type: None,
            payload: None,
        }
    }
}

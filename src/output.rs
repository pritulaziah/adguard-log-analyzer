use crate::{
    cli::Cli,
    models::{JsonCall, JsonOutputEntry, JsonRawLogEntry, JsonServiceGroup, LogEntry},
};
use anyhow::Result;
use std::collections::HashMap;
use std::fs;

pub fn output_logs(entries: Vec<LogEntry>, opts: &Cli) -> Result<()> {
    let filename = opts
        .file_path
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let out_dir = format!("result_logs/{}", filename);
    fs::create_dir_all(&out_dir)?;

    let json_entries = group_by_service(&entries);
    let output_content = serde_json::to_string_pretty(&json_entries)?;

    fs::write(
        format!("{}/{}.sciter.json", out_dir, filename),
        output_content,
    )?;

    Ok(())
}

fn group_by_service(entries: &[LogEntry]) -> Vec<JsonOutputEntry> {
    let mut groups = HashMap::new();
    let mut raw_entries = Vec::new();

    for entry in entries {
        let (Some(service), Some(method), Some(event_type)) = (
            entry.service.as_ref(),
            entry.method.as_ref(),
            entry.event_type.as_ref(),
        ) else {
            raw_entries.push(JsonOutputEntry::Raw(raw_log_entry(entry)));
            continue;
        };

        let call = JsonCall {
            method: method.clone(),
            event_type: event_type.clone(),
            timestamp: entry.timestamp,
            raw: entry.message.clone(),
            payload: entry.payload.clone().unwrap_or(serde_json::Value::Null),
        };

        groups
            .entry(service.clone())
            .or_insert_with(|| JsonServiceGroup {
                service: service.clone(),
                calls: Vec::new(),
            })
            .calls
            .push(call);
    }

    groups
        .into_values()
        .map(JsonOutputEntry::Service)
        .chain(raw_entries)
        .collect()
}

fn raw_log_entry(entry: &LogEntry) -> JsonRawLogEntry {
    JsonRawLogEntry {
        timestamp: entry.timestamp,
        message: Some(entry.message.clone()),
    }
}

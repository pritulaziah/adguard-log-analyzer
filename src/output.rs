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
    let mut output = Vec::new();
    let mut service_indexes: HashMap<String, usize> = HashMap::new();

    for entry in entries {
        let (Some(service), Some(method), Some(event_type)) = (
            entry.service.as_ref(),
            entry.method.as_ref(),
            entry.event_type.as_ref(),
        ) else {
            output.push(JsonOutputEntry::Raw(JsonRawLogEntry {
                timestamp: entry.timestamp,
                message: Some(entry.message.clone()),
            }));
            continue;
        };

        let service_index = ensure_service_group(&mut output, &mut service_indexes, service);
        if let Some(JsonOutputEntry::Service(group)) = output.get_mut(service_index) {
            group.calls.push(JsonCall {
                method: method.clone(),
                event_type: event_type.clone(),
                timestamp: entry.timestamp,
                raw: entry.message.clone(),
                payload: entry.payload.clone().unwrap_or(serde_json::Value::Null),
            });
        }
    }

    output
}

fn ensure_service_group(
    output: &mut Vec<JsonOutputEntry>,
    service_indexes: &mut HashMap<String, usize>,
    service: &str,
) -> usize {
    if let Some(index) = service_indexes.get(service) {
        return *index;
    }

    let index = output.len();
    output.push(JsonOutputEntry::Service(JsonServiceGroup {
        service: service.to_string(),
        calls: Vec::new(),
    }));
    service_indexes.insert(service.to_string(), index);
    index
}

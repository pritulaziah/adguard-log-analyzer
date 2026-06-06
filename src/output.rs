use crate::{ cli::Cli, models::{ LogEntry, JsonLogEntry, Stats } };
use std::{ collections::HashMap, fs, sync::LazyLock };
use anyhow::Result;
use regex::Regex;

fn format_duration(duration: chrono::Duration) -> String {
    let total_secs = duration.num_seconds();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    format!("{hours}h {minutes}m {seconds}s")
}

pub fn output_logs(entries: Vec<LogEntry>, opts: &Cli) -> Result<()> {
    let filename = opts.file_path.file_name().unwrap().to_string_lossy().to_string();
    let out_dir = format!("result_logs/{}", filename);
    fs::create_dir_all(&out_dir)?;

    if opts.stats {
        let total = entries.len();

        let mut levels: HashMap<String, usize> = HashMap::new();
        let mut modules: HashMap<String, String> = HashMap::new();

        static DLL_RE: LazyLock<Regex> = LazyLock::new(||
            Regex::new(r"^(.+\.dll): version is ([^,]+)").unwrap()
        );
        static OK_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(.+?): (OK):").unwrap());
        static OS_RE: LazyLock<Regex> = LazyLock::new(||
            Regex::new(r"^OS version is (.+)").unwrap()
        );

        for entry in &entries {
            *levels.entry(entry.level.to_string()).or_insert(0) += 1;

            if
                let Some(module_info) = entry.message.strip_prefix(
                    "Application version and installed modules info:\n"
                )
            {
                for line in module_info.trim().split('\n') {
                    let line = line.trim_matches(|c: char| (c == ',' || c.is_whitespace()));

                    if line.is_empty() {
                        continue;
                    }

                    if let Some(rest) = line.strip_prefix("Application version is ") {
                        modules.insert("application_version".into(), rest.to_string());
                    } else if let Some(rest) = line.strip_prefix("Application shown version is ") {
                        modules.insert("application_shown_version".into(), rest.to_string());
                    } else if let Some(caps) = DLL_RE.captures(line) {
                        modules.insert(caps[1].to_string(), caps[2].to_string());
                    } else if let Some(caps) = OK_RE.captures(line) {
                        modules.insert(caps[1].to_string(), caps[2].to_string());
                    } else if let Some(caps) = OS_RE.captures(line) {
                        modules.insert("os_version".into(), caps[1].to_string());
                    }
                }
            }
        }

        let first_ts = entries.first().map(|e| e.timestamp);
        let last_ts = entries.last().map(|e| e.timestamp);

        let span = match (first_ts, last_ts) {
            (Some(first), Some(last)) => Some(last - first),
            _ => None,
        };

        let stats = Stats {
            total,
            levels,
            modules,
            first_timestamp: first_ts.map(|ts| ts.format("%d.%m.%Y %H:%M:%S%.3f").to_string()),
            last_timestamp: last_ts.map(|ts| ts.format("%d.%m.%Y %H:%M:%S%.3f").to_string()),
            duration: span.map(|d| format_duration(d)),
        };

        let json = serde_json::to_string_pretty(&stats)?;
        fs::write(format!("{}/stats.log.json", out_dir), json)?;
    }

    let level_name = match &opts.level {
        Some(lvl) => lvl.to_string(),
        None => "all".to_string(),
    };

    let mut segments = vec![];

    if opts.sciter {
        segments.push("sciter".to_string());
    } else {
        segments.push(level_name.to_lowercase());
    }

    if let Some(n) = opts.first {
        segments.push(format!("first{}", n));
    } else if let Some(n) = opts.last {
        segments.push(format!("last{}", n));
    }

    let json_entries: Vec<JsonLogEntry> = entries
        .iter()
        .map(|e| convert_to_json_entry(e))
        .collect();
    let output_content = serde_json::to_string_pretty(&json_entries)?;

    fs::write(format!("{}/{}.json", out_dir, segments.join("_")), output_content)?;

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

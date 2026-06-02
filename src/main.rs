use anyhow::{ Result };
use std::{ fs };
mod models;
mod cli;
use cli::Cli;
mod log_parser;
use log_parser::{ LogParser };
mod log_query;
use log_query::{ LogQuery, QueryOptions };

fn main() -> Result<()> {
    let args = Cli::parse()?;
    let level = args.level.as_ref();
    let file_path = &args.file_path;
    let save = args.save;
    let first = args.first;
    let last = args.last;

    let logs = LogParser::parse_file(file_path)?;
    let opts = QueryOptions {
        level,
        first,
        last,
    };
    let selected_logs = LogQuery::query_logs(logs, opts);
    let output = selected_logs
        .iter()
        .map(|entry| entry.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    if save {
        let log_level = match level {
            Some(level) => level.to_string(),
            None => "all".to_string(),
        };
        let output_path = file_path.file_name().unwrap().to_string_lossy().to_string();

        let folder = format!("result_logs/{}", output_path);
        fs::create_dir_all(&folder)?;

        let mut parts = vec![format!("{}/{}", folder, log_level)];

        if let Some(n) = first {
            parts.push(format!("first{}", n));
        } else if let Some(n) = last {
            parts.push(format!("last{}", n));
        }

        fs::write(format!("{}.log", parts.join("_")), output)?;
    } else {
        println!("{}", output);
    }

    Ok(())
}

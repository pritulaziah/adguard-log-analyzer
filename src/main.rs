use anyhow::{ Result };
mod models;
mod cli;
use cli::Cli;
mod log_parser;
use log_parser::{ LogParser };
mod log_query;
use log_query::{ LogQuery };
mod formatter;
use formatter::Formatter;

fn main() -> Result<()> {
    let args = Cli::parse()?;
    let file_path = &args.file_path;

    let logs = LogParser::parse_file(file_path)?;
    let selected_logs = LogQuery::query_logs(logs, &args);
    Formatter::output_logs(selected_logs, &args)?;

    Ok(())
}

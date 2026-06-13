use anyhow::Result;
mod cli;
mod models;
use cli::Cli;
mod parser;
use parser::parse_file;
mod query;
use query::query_logs;
mod output;
use output::output_logs;

fn main() -> Result<()> {
    let args = Cli::parse()?;

    let logs = parse_file(&args.file_path)?;
    let selected_logs = query_logs(logs, &args);
    output_logs(selected_logs, &args)?;

    Ok(())
}

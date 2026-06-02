use anyhow::{ Result };
mod models;
mod cli;
use cli::Cli;
mod parser;
use parser::{ parse_file };
mod query;
use query::{ query_logs };
mod output;
use output::{ output_logs };

fn main() -> Result<()> {
    let args = Cli::parse()?;
    let file_path = &args.file_path;

    let logs = parse_file(file_path)?;
    let selected_logs = query_logs(logs, &args);
    output_logs(selected_logs, &args)?;

    Ok(())
}

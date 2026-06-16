use anyhow::Result;
use chrono::NaiveDateTime;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    pub file_path: PathBuf,

    #[arg(long)]
    pub service: Option<String>,

    #[arg(long, value_parser = parse_datetime)]
    pub from: Option<NaiveDateTime>,

    #[arg(long, value_parser = parse_datetime)]
    pub to: Option<NaiveDateTime>,
}

impl Cli {
    pub fn parse() -> Result<Self> {
        let args = <Self as clap::Parser>::parse();
        args.validate()?;
        Ok(args)
    }

    fn validate(&self) -> Result<()> {
        if !self.file_path.is_file() {
            anyhow::bail!("Expected a file path, got '{}'", self.file_path.display());
        }

        if let (Some(from), Some(to)) = (self.from, self.to)
            && from > to
        {
            anyhow::bail!("Expected --from to be earlier than or equal to --to");
        }

        Ok(())
    }
}

fn parse_datetime(value: &str) -> Result<NaiveDateTime, String> {
    NaiveDateTime::parse_from_str(value, "%d.%m.%Y %H:%M:%S%.3f")
        .map_err(|_| "expected date format dd.mm.yyyy HH:MM:SS.mmm".to_string())
}

use clap::Parser;
use std::path::PathBuf;
use anyhow::{ Result };

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    pub file_path: PathBuf,
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

        Ok(())
    }
}

use clap::Parser;
use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    file_path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let file = File::open(&args.file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}

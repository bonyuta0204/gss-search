// src/cli.rs

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// URL for spreadsheet
    #[arg(short, long)]
    pub url: String,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}

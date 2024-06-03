// src/cli.rs

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version = "1.0",
    about = "A CLI tool for fuzzy searching Google Spreadsheets",
    long_about = "gss-search allows users to fetch, cache, and fuzzy search records from Google Spreadsheets using a user-friendly command-line interface."
)]
pub struct Cli {
    /// URL for spreadsheet
    #[arg(short, long)]
    pub url: String,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}

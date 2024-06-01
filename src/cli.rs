// src/cli.rs

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// does testing things
    Fetch {
        /// lists test values
        #[arg(short, long)]
        spreadsheet_id: String,
    },
}

pub fn parse_args() -> Cli {
    Cli::parse()
}

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
    /// Fetch data and save it for search
    Save {
        /// URL for spreadsheet
        #[arg(short, long)]
        url: String,
    },

    Search {
        /// URL for spreadsheet
        #[arg(short, long)]
        url: String,
    },
}

pub fn parse_args() -> Cli {
    Cli::parse()
}

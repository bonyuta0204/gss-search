mod auth;
mod cache;
mod cli;
mod fetch;
mod sheet_client;
mod spreadsheet;
mod url_helper;

use cli::{parse_args, Commands};
use fetch::run_fetch;

#[tokio::main]
async fn main() {
    let args = parse_args();

    match &args.command {
        Some(Commands::Save { url }) => run_fetch(url).await,
        None => {}
    }
}

mod auth;
mod cli;
mod fetch;
mod path_builder;
mod search;
mod select;
mod sheet_client;
mod spreadsheet;
mod storage;
mod table;
mod url_helper;

use cli::{parse_args, Commands};
use fetch::run_fetch;
use search::run_search;

#[tokio::main]
async fn main() {
    let args = parse_args();

    match &args.command {
        Some(Commands::Save { url }) => run_fetch(url).await,
        Some(Commands::Search { url }) => run_search(url).await,
        None => {}
    }
}

mod auth;
mod cache;
mod cli;
mod log;
mod path_builder;
mod run;
mod select;
mod sheet_client;
mod spreadsheet;
mod storage;
mod table;
mod url_helper;

use std::error::Error;

use cli::parse_args;
use log::build_subscriber;
use run::run;
use tracing::info;
use tracing_subscriber::util::SubscriberInitExt;
// ensure that the directory exists

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args();

    build_subscriber().init();

    info!("started");

    run(&args.url).await;
    Ok(())
}

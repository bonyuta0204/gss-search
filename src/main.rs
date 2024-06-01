mod auth;
mod cache;
mod cli;
mod sheet;

use auth::create_auth;
use cli::{parse_args, Commands};
use sheet::SheetClient;

#[tokio::main]
async fn main() {
    let args = parse_args();

    match &args.command {
        Some(Commands::Fetch { spreadsheet_id }) => run_fetch(spreadsheet_id).await,
        None => {}
    }
}

async fn run_fetch(spreadsheet_id: &str) {
    // Create the authenticator
    // Create the HTTP client
    let auth = create_auth().await.expect("Failed to Authenticate");

    let sheet_client = SheetClient::new(auth);

    let range = "ListUp!A1:E100";

    // Fetch the values from the spreadsheet
    let result = sheet_client.fetch_data(spreadsheet_id, range).await;

    match result {
        Ok(values) => {
            for row in values {
                println!("{:?}", row);
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }
}

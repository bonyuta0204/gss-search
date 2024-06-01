mod auth;
mod sheet;

use auth::create_auth;
use sheet::SheetClient;

#[tokio::main]
async fn main() {
    // Create the authenticator
    // Create the HTTP client
    let auth = create_auth().await.expect("Failed to Authenticate");

    let sheet_client = SheetClient::new(auth);

    // Replace with your spreadsheet ID and range
    let spreadsheet_id = "1O3zlFl2IslSwFOi-rYOX6hdlAIS2IhaTmGqzpgIgbbU";
    let range = "ListUp!A1:B2";

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

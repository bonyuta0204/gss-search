mod auth;

use auth::create_auth;
use google_sheets4::{hyper::Client, hyper_rustls, Sheets};

#[tokio::main]
async fn main() {
    // Create the authenticator
    // Create the HTTP client
    let auth = create_auth().await.expect("Failed to Authenticate");

    // Create the Sheets API hub
    let hub = Sheets::new(
        Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .build(),
        ),
        auth,
    );

    // Replace with your spreadsheet ID and range
    let spreadsheet_id = "1O3zlFl2IslSwFOi-rYOX6hdlAIS2IhaTmGqzpgIgbbU";
    let range = "ListUp!A1:B2";

    // Fetch the values from the spreadsheet
    let result = hub
        .spreadsheets()
        .values_get(spreadsheet_id, range)
        .doit()
        .await;

    match result {
        Ok((_, value_range)) => {
            if let Some(values) = value_range.values {
                for row in values {
                    println!("{:?}", row);
                }
            } else {
                println!("No data found.");
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }
}

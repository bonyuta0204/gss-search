use google_sheets4::{
    hyper::Client,
    hyper_rustls,
    oauth2::{
        read_application_secret, AuthorizedUserAuthenticator, InstalledFlowAuthenticator,
        InstalledFlowReturnMethod,
    },
    Sheets,
};

#[tokio::main]
async fn main() {
    let secret = read_application_secret("clientsecret.json")
        .await
        .expect("clientsecret.json");

    // Create an authenticator that uses an InstalledFlow to authenticate. The
    // authentication tokens are persisted to a file named tokencache.json. The
    // authenticator takes care of caching tokens to disk and refreshing tokens once
    // they've expired.
    let mut auth =
        InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
            .persist_tokens_to_disk("tokencache.json")
            .build()
            .await
            .unwrap();

    // Create the authenticator
    // Create the HTTP client

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

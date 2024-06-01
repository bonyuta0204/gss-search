use google_sheets4::{
    client::GetToken,
    hyper::{client::HttpConnector, Client},
    hyper_rustls::{HttpsConnector, HttpsConnectorBuilder},
    Sheets,
};
use serde_json::Value;

pub struct SheetClient {
    hub: Sheets<HttpsConnector<HttpConnector>>,
}

impl SheetClient {
    pub fn new<A: GetToken + 'static>(auth: A) -> Self {
        let hub = Sheets::new(
            Client::builder().build(
                HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_or_http()
                    .enable_http1()
                    .build(),
            ),
            auth,
        );

        Self { hub }
    }

    pub async fn fetch_data(
        &self,
        spreadsheet_id: &str,
        range: &str,
    ) -> Result<Vec<Vec<Value>>, google_sheets4::Error> {
        let response = self
            .hub
            .spreadsheets()
            .values_get(spreadsheet_id, range)
            .doit()
            .await?;
        let values = response.1.values.unwrap_or_default();
        Ok(values)
    }
}

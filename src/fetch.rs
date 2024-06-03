use crate::{
    auth::create_auth,
    path_builder::PathBuilder,
    sheet_client::SheetClient,
    spreadsheet::{self, get_sheet_title},
    storage::save_to_storage,
    table::Table,
    url_helper::extract_id_from_url,
};
use spreadsheet::find_sheet_by_id;
use tracing::info;

#[tracing::instrument]
pub async fn run_fetch(url: &str) {
    let spreadsheet_info = match extract_id_from_url(url) {
        Some(info) => info,
        None => {
            eprintln!("Error: Invalid URL or unable to extract spreadsheet information.");
            return;
        }
    };

    // Create the authenticator
    // Create the HTTP client
    let auth = create_auth().await.expect("Failed to Authenticate");

    let sheet_client = SheetClient::new(auth);

    let spreadsheet_detail = match sheet_client
        .get_spreadsheet(&spreadsheet_info.spreadsheet_id)
        .await
    {
        Ok(sheet) => sheet,
        Err(_) => {
            eprintln!("Error: Invalid URL or unable to extract spreadsheet information.");
            return;
        }
    };

    let target_sheet = find_sheet_by_id(&spreadsheet_detail, spreadsheet_info.sheet_id);

    let range = match target_sheet {
        Some(sheet) => get_sheet_title(sheet).unwrap(),
        None => {
            eprintln!("Failed to fetch sheet title.",);
            return;
        }
    };

    let result = sheet_client
        .fetch_data(&spreadsheet_info.spreadsheet_id, &range)
        .await;

    let path_builder = PathBuilder::new();

    match result {
        Ok(values) => {
            let table = Table::from(values);
            info!("Table created");
            let data_path = path_builder.sheet_data(&spreadsheet_info);

            save_to_storage(&data_path, &table).expect("failed to write to disk");
        }
        Err(e) => println!("Error: {:?}.", e),
    }
}

use crate::{
    auth::create_auth,
    path::path_for_sheet,
    sheet_client::SheetClient,
    spreadsheet::{self, get_sheet_title},
    storage::save_to_storage,
    url_helper::extract_id_from_url,
};
use spreadsheet::find_sheet_by_id;
use std::fs::File;
use std::io::{self, Write};

use std::fs;
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

    let target_sheet =
        find_sheet_by_id(&spreadsheet_detail, spreadsheet_info.sheet_id.unwrap_or(0));

    let range = match target_sheet {
        Some(sheet) => get_sheet_title(sheet).unwrap(),
        None => {
            eprint!("failed");
            return;
        }
    };

    // Fetch the values from the spreadsheet
    let result = sheet_client
        .fetch_data(&spreadsheet_info.spreadsheet_id, &range)
        .await;

    match result {
        Ok(values) => {
            let data_path = path_for_sheet(
                &spreadsheet_info.spreadsheet_id,
                spreadsheet_info.sheet_id.unwrap(),
            );

            if let Some(parent) = data_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent).expect("Failed to create directories");
                }
            }

            save_to_storage(data_path, &values);
        }
        Err(e) => println!("Error: {:?}", e),
    }
}
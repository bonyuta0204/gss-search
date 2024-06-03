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
use std::time::Instant;

fn log_progress(message: &str, start: &Instant) {
    println!("[{:>6} ms] {}", start.elapsed().as_millis(), message);
}

pub async fn run_fetch(url: &str) {
    let start = Instant::now();
    let spreadsheet_info = match extract_id_from_url(url) {
        Some(info) => {
            log_progress("Spreadsheet information extracted successfully.", &start);
            info
        }
        None => {
            eprintln!("Error: Invalid URL or unable to extract spreadsheet information. Time elapsed: {:?}", start.elapsed());
            return;
        }
    };

    // Create the authenticator
    // Create the HTTP client
    let auth = create_auth().await.expect("Failed to Authenticate");
    log_progress("Authentication successful.", &start);

    let sheet_client = SheetClient::new(auth);
    log_progress("Sheet client created successfully.", &start);

    let spreadsheet_detail = match sheet_client
        .get_spreadsheet(&spreadsheet_info.spreadsheet_id)
        .await
    {
        Ok(sheet) => {
            log_progress("Spreadsheet details fetched successfully.", &start);
            sheet
        }
        Err(_) => {
            eprintln!("Error: Invalid URL or unable to extract spreadsheet information. Time elapsed: {:?}", start.elapsed());
            return;
        }
    };

    let target_sheet = find_sheet_by_id(&spreadsheet_detail, spreadsheet_info.sheet_id);
    log_progress("Target sheet identified successfully.", &start);

    let range = match target_sheet {
        Some(sheet) => {
            log_progress("Sheet title fetched successfully.", &start);
            get_sheet_title(sheet).unwrap()
        }
        None => {
            eprintln!(
                "Failed to fetch sheet title. Time elapsed: {:?}",
                start.elapsed()
            );
            return;
        }
    };

    let result = sheet_client
        .fetch_data(&spreadsheet_info.spreadsheet_id, &range)
        .await;
    log_progress("Data fetched successfully.", &start);

    let path_builder = PathBuilder::new();
    log_progress("Path builder created successfully.", &start);

    match result {
        Ok(values) => {
            let table = Table::from(values);
            log_progress("Table creating", &start);
            let data_path = path_builder.sheet_data(&spreadsheet_info);
            log_progress("Table created", &start);

            save_to_storage(&data_path, &table).expect("failed to write to disk");
            log_progress("Data saved to storage successfully.", &start);
        }
        Err(e) => println!("Error: {:?}. Time elapsed: {:?}", e, start.elapsed()),
    }
}

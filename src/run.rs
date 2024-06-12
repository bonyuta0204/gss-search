use tokio::task::JoinHandle;

use crate::select::skim_select;
use crate::table::TableRow;
use crate::{
    cache::build_cache, path_builder::PathBuilder, storage::load_from_storage, table::Table,
    url_helper::extract_id_from_url,
};

/// Fetches data from a Google Spreadsheet and saves it locally.
///
/// # Arguments
///
/// * `url` - A string slice that holds the URL of the Google Spreadsheet.
///
/// # Example
///
/// ```
/// let url = "https://docs.google.com/spreadsheets/d/1O3zlFl2IslSwFOi-rYOX6hdlAIS2IhaTmGqzpgIgbbU/edit#gid=1234";
/// run(url).await;
/// ```
pub async fn run(url: &str) {
    let spreadsheet_info = match extract_id_from_url(url) {
        Some(info) => info,
        None => {
            eprintln!("Error: Invalid URL or unable to extract spreadsheet information.");
            return;
        }
    };

    let path_builder = PathBuilder::new();
    let store_path = path_builder.sheet_data(&spreadsheet_info);

    if let Ok(table) = load_from_storage::<Table<String>>(&store_path) {
        let handle = refresh_cache_in_background(url);
        process_table(table);
        handle.await.expect("Failed to refresh cache");
    } else {
        build_cache(url).await.expect("Failed to create cache");
        let table = load_from_storage::<Table<String>>(&store_path).expect("Failed to read");
        process_table(table);
    }
}

fn refresh_cache_in_background(url: &str) -> JoinHandle<()> {
    let url = url.to_string();
    let handle = tokio::spawn(async move {
        let _ = build_cache(&url).await;
    });
    handle
}

fn process_table(table: Table<String>) {
    let rows: Vec<TableRow<String>> = table.body_rows().clone();
    let selected = skim_select(rows).expect("Failed to select");
    selected.pretty_print();
}

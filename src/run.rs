use crate::{
    cache::build_cache, path_builder::PathBuilder, select::interactive_select,
    storage::load_from_storage, table::Table, url_helper::extract_id_from_url,
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
        // Spawn a background task to refresh the data
        let url = url.to_string();
        let handle = tokio::spawn(async move {
            build_cache(&url).await;
        });

        // Proceed with the search using cached data
        let selected = interactive_select(table.body_rows()).expect("Failed to select");
        selected.pretty_print();
        handle.await.expect("Failed to refresh the cache")
    } else {
        // Fetch data first if no cached data exists
        build_cache(url).await;
        let table = load_from_storage::<Table<String>>(&store_path).expect("failed to read");
        let selected = interactive_select(table.body_rows()).expect("Failed to select");
        selected.pretty_print();
    }
}

use crate::{
    fetch::run_fetch, path_builder::PathBuilder, select::interactive_select,
    storage::load_from_storage, table::Table, url_helper::extract_id_from_url,
};

pub async fn run_search(url: &str) {
    let spreadsheet_info = match extract_id_from_url(url) {
        Some(info) => info,
        None => {
            eprintln!("Error: Invalid URL or unable to extract spreadsheet information.");
            return;
        }
    };

    let path_builder = PathBuilder::new();
    let store_path = path_builder.sheet_data(&spreadsheet_info);

    if let Ok(table) = load_from_storage::<Table>(&store_path) {
        // Spawn a background task to refresh the data
        let url = url.to_string();
        tokio::spawn(async move {
            run_fetch(&url).await;
        });

        // Proceed with the search using cached data
        let selected = interactive_select(table.body_rows()).expect("Failed to select");
        println!("{:#?}", selected);
    } else {
        // Fetch data first if no cached data exists
        run_fetch(url).await;
        let table = load_from_storage::<Table>(&store_path).expect("failed to read");
        let selected = interactive_select(table.body_rows()).expect("Failed to select");
        println!("{:#?}", selected);
    }
}

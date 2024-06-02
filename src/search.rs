use crate::{
    path_builder::PathBuilder, select::interactive_select, sheet_data::SheetData,
    storage::load_from_storage, url_helper::extract_id_from_url,
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

    let data = load_from_storage::<SheetData>(&store_path).expect("failed to read");

    let search_data: Vec<_> = data.iter().map(|row| row).collect();

    let selected = interactive_select(search_data).expect("Failed to select");

    selected.pretty_print();
}

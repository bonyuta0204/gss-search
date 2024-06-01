use std::path::PathBuf;

/// Generates a file path based on the given spreadsheet_id and sheet_id.
/// The path format is: ~/.gss-search/data/<spreadsheet_id>/<sheet_id>
pub fn path_for_sheet(spreadsheet_id: &str, sheet_id: i32) -> PathBuf {
    let mut path = dirs::home_dir().expect("Unable to get home directory");
    path.push(".gss-search");
    path.push("data");
    path.push(spreadsheet_id);
    path.push(sheet_id.to_string());
    path
}

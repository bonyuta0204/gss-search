use std::path::PathBuf;

use crate::url_helper::SpreadsheetInfo;

const BASE_DIR: &str = ".gss-search";

pub struct PathBuilder {
    base_dir: PathBuf,
}

impl PathBuilder {
    pub fn new() -> Self {
        let mut path = dirs::home_dir().expect("Unable to get home directory");
        path.push(BASE_DIR);
        Self { base_dir: path }
    }

    pub fn sheet_data(&self, spreadsheet_info: &SpreadsheetInfo) -> PathBuf {
        let mut path = self.base_dir.clone();
        path.push("data");
        path.push(spreadsheet_info.spreadsheet_id.clone());
        path.push(spreadsheet_info.sheet_id.to_string());
        path
    }

    pub fn tokencache(&self) -> PathBuf {
        let mut path = self.base_dir.clone();
        path.push("tokencache.json");
        path
    }
}

use std::path::PathBuf;

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

    pub fn sheet_data(&self, spreadsheet_id: &str, sheet_id: i32) -> PathBuf {
        let mut path = self.base_dir.clone();
        path.push("data");
        path.push(spreadsheet_id);
        path.push(sheet_id.to_string());
        path
    }

    pub fn tokencache(&self) -> PathBuf {
        let mut path = self.base_dir.clone();
        path.push("tokencache.json");
        path
    }
}

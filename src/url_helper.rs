use regex::Regex;

pub struct SpreadsheetInfo {
    pub spreadsheet_id: String,
    pub sheet_id: i32,
}

pub fn extract_id_from_url(url: &str) -> Option<SpreadsheetInfo> {
    // extract spreadsheet_id and sheet_id from URL
    // URL format is as follows https://docs.google.com/spreadsheets/d/1O3zlFl2IslSwFOi-rYOX6hdlAIS2IhaTmGqzpgIgbbU/edit#gid=1234

    let re = Regex::new(r"https://docs\.google\.com/spreadsheets/d/([^/]+)/edit(?:#gid=(\d+))?")
        .unwrap();

    if let Some(captures) = re.captures(url) {
        let spreadsheet_id = captures.get(1).map(|m| m.as_str())?;
        let sheet_id = captures
            .get(2)
            .map_or(Ok(0), |m| m.as_str().parse::<i32>())
            .expect("Invalid sheet id");

        return Some(SpreadsheetInfo {
            spreadsheet_id: spreadsheet_id.to_string(),
            sheet_id,
        });
    }

    None
}

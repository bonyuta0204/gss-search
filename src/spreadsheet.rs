use google_sheets4::api::{Sheet, Spreadsheet};

pub fn find_sheet_by_id(spreadsheet: &Spreadsheet, sheet_id: i32) -> Option<&Sheet> {
    match &spreadsheet.sheets {
        Some(ref sheets) => sheets.iter().find(|&sheet| {
            sheet
                .properties
                .as_ref()
                .map_or(false, |props| props.sheet_id == Some(sheet_id))
        }),
        None => None,
    }
}

pub fn get_sheet_title(sheet: &Sheet) -> Option<&str> {
    sheet.properties.as_ref()?.title.as_deref()
}

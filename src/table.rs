use core::fmt;

use clap::builder::Str;
use google_sheets4::hyper::header;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub header: TableRow,
    pub body: Vec<TableRow>,
    pub columns: Vec<TableColumn>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableRow {
    data: Vec<Value>,
}

impl TableRow {
    pub fn new(data: Vec<Value>) -> Self {
        Self { data }
    }
}

impl fmt::Display for TableRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for cell in &self.data {
            match cell {
                Value::String(s) => write!(f, "{:<20} |", s.to_string())?,
                Value::Number(n) => write!(f, "{:<20} |", n.to_string())?,
                _ => {}
            }
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableColumn {
    title: String,
}

impl Table {
    pub fn new(header: TableRow, body: Vec<TableRow>, columns: Vec<TableColumn>) -> Self {
        Self {
            header,
            body,
            columns,
        }
    }
}

impl From<Vec<Vec<Value>>> for Table {
    fn from(value: Vec<Vec<Value>>) -> Self {
        // calculate row size and column size
        let column_size: usize = value.iter().map(|row| row.len()).max().unwrap_or(0);
        // initialize columns based on the size
        let mut columns: Vec<_> = (0..column_size)
            .map(|i| TableColumn {
                title: String::new(),
            })
            .collect();

        let mut rows: Vec<_> = value.into_iter().filter(|row| !row.is_empty()).collect();

        if rows.len() < 1 {
            return Self {
                header: TableRow::new(Vec::new()),
                body: Vec::new(),
                columns: Vec::new(),
            };
        }

        let header = TableRow::new(rows.remove(0));

        for (i, cell) in header.data.iter().enumerate() {
            if let Some(title) = cell.as_str() {
                columns[i].title = title.to_owned()
            }
        }

        let body: Vec<_> = rows
            .into_iter()
            .map(|row| TableRow::new(row.to_owned()))
            .collect();

        // create table body
        Table {
            header,
            body,
            columns,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_table_from_empty() {
        let data: Vec<Vec<Value>> = vec![];
        let table = Table::from(data);
        assert!(table.header.data.is_empty());
        assert!(table.body.is_empty());
        assert!(table.columns.is_empty());
    }

    #[test]
    fn test_table_from_single_row() {
        let data = vec![vec![json!("header1"), json!("header2")]];
        let table = Table::from(data);
        assert_eq!(table.header.data.len(), 2);
        assert!(table.body.is_empty());
        assert_eq!(table.columns.len(), 2);
    }

    #[test]
    fn test_table_from_multiple_rows() {
        let data = vec![
            vec![json!("header1"), json!("header2")],
            vec![],
            vec![json!("row1col1"), json!("row1col2")],
            vec![json!("row2col1"), json!("row2col2")],
        ];
        let table = Table::from(data);
        assert_eq!(table.header.data.len(), 2);
        assert_eq!(table.body.len(), 2);
        assert_eq!(table.columns.len(), 2);
        assert_eq!(table.columns[0].title, "header1")
    }

    #[test]
    fn test_table_from_uneven_rows() {
        let data = vec![
            vec![json!("header1"), json!("header2")],
            vec![json!("row1col1")],
            vec![json!("row2col1"), json!("row2col2"), json!("row2col3")],
        ];
        let table = Table::from(data);

        dbg!(&table);

        assert_eq!(table.header.data.len(), 2);
        assert_eq!(table.body.len(), 2);
        assert_eq!(table.columns.len(), 3);
    }
}

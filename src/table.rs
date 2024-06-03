use core::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub header: TableRowInternal,
    pub body: Vec<TableRowInternal>,
    pub columns: Vec<TableColumn>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableRowInternal {
    data: Vec<Value>,
}

impl TableRowInternal {
    pub fn new(data: Vec<Value>) -> Self {
        Self { data }
    }
}

#[derive(Debug)]
pub struct TableRow<'a> {
    internal: &'a TableRowInternal,
    table: &'a Table,
}

impl<'a> fmt::Display for TableRow<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, cell) in self.internal.data.iter().enumerate() {
            let col_size = self.table.columns[i].max_content_lengh;
            match cell {
                Value::String(s) => write!(f, "{:<width$} |", s.to_string(), width = col_size)?,
                Value::Number(n) => write!(f, "{:<width$} |", n.to_string(), width = col_size)?,
                _ => {}
            }
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableColumn {
    title: String,
    max_content_lengh: usize,
}

impl Table {
    pub fn new(
        header: TableRowInternal,
        body: Vec<TableRowInternal>,
        columns: Vec<TableColumn>,
    ) -> Self {
        Self {
            header,
            body,
            columns,
        }
    }

    pub fn body_rows(&self) -> Vec<TableRow> {
        self.body
            .iter()
            .map(|row| TableRow {
                internal: row,
                table: self,
            })
            .collect()
    }
}

impl From<Vec<Vec<Value>>> for Table {
    fn from(value: Vec<Vec<Value>>) -> Self {
        // calculate row size and column size
        let column_size: usize = value.iter().map(|row| row.len()).max().unwrap_or(0);
        // initialize columns based on the size
        let mut columns: Vec<_> = (0..column_size)
            .map(|_i| TableColumn {
                title: String::new(),
                max_content_lengh: 0,
            })
            .collect();

        let mut rows: Vec<_> = value.into_iter().filter(|row| !row.is_empty()).collect();

        if rows.len() < 1 {
            return Self {
                header: TableRowInternal::new(Vec::new()),
                body: Vec::new(),
                columns: Vec::new(),
            };
        }

        let header = TableRowInternal::new(rows.remove(0));

        for (i, cell) in header.data.iter().enumerate() {
            if let Some(title) = cell.as_str() {
                columns[i].title = title.to_owned()
            }
        }

        let body: Vec<_> = rows
            .into_iter()
            .map(|row| {
                for (i, cell) in row.iter().enumerate() {
                    let cell_length = match cell {
                        Value::String(s) => s.len(),
                        Value::Number(n) => n.to_string().len(),
                        _ => 0,
                    };
                    if cell_length > columns[i].max_content_lengh {
                        columns[i].max_content_lengh = cell_length;
                    }
                }
                TableRowInternal::new(row.to_owned())
            })
            .collect();

        // create table body
        Table::new(header, body, columns)
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

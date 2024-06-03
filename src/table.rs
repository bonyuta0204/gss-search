use core::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct Table<C: Display> {
    pub header: TableRowInternal<C>,
    pub body: Vec<TableRowInternal<C>>,
    pub columns: Vec<TableColumn>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableRowInternal<C: Display> {
    data: Vec<C>,
}

impl<C: Display> TableRowInternal<C> {
    pub fn new(data: Vec<C>) -> Self {
        Self { data }
    }
}

#[derive(Debug)]
pub struct TableRow<'a, C: Display> {
    internal: &'a TableRowInternal<C>,
    table: &'a Table<C>,
}

impl<'a, C: Display> TableRow<'a, C> {
    pub fn pretty_print(&self) {
        for (i, cell) in self.internal.data.iter().enumerate() {
            let column = &self.table.columns[i];
            println!("{}:{}", column.title, cell)
        }
    }
}

impl<'a, C: Display> fmt::Display for TableRow<'a, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, cell) in self.internal.data.iter().enumerate() {
            let col_size = self.table.columns[i].max_content_lengh.min(20);
            write!(f, "{:<width$} |", cell, width = col_size)?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableColumn {
    title: String,
    max_content_lengh: usize,
}

impl<C: Display> Table<C> {
    pub fn new(
        header: TableRowInternal<C>,
        body: Vec<TableRowInternal<C>>,
        columns: Vec<TableColumn>,
    ) -> Self {
        Self {
            header,
            body,
            columns,
        }
    }

    pub fn body_rows(&self) -> Vec<TableRow<C>> {
        self.body
            .iter()
            .map(|row| TableRow {
                internal: row,
                table: self,
            })
            .collect()
    }
}

impl From<Vec<Vec<Value>>> for Table<String> {
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

        let rows: Vec<_> = value.into_iter().filter(|row| !row.is_empty()).collect();

        if rows.len() < 1 {
            return Self {
                header: TableRowInternal::new(Vec::new()),
                body: Vec::new(),
                columns: Vec::new(),
            };
        }

        let mut rows: Vec<_> = rows
            .into_iter()
            .map(|row| row.into_iter().map(|cell| value_to_string(cell)).collect())
            .collect();

        let header = TableRowInternal::new(rows.remove(0));

        for (i, cell) in header.data.iter().enumerate() {
            columns[i].title = cell.to_owned()
        }

        let body: Vec<_> = rows
            .into_iter()
            .map(|row| {
                for (i, cell) in row.iter().enumerate() {
                    let cell_length = cell.len();
                    if cell_length > columns[i].max_content_lengh {
                        columns[i].max_content_lengh = cell_length;
                    }
                }
                TableRowInternal::new(row.to_owned())
            })
            .collect();

        info!(
            "Table created. Colmns: {}, Rows: {}",
            columns.len(),
            body.len()
        );
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

fn value_to_string(value: Value) -> String {
    match value {
        Value::String(s) => s,
        Value::Number(n) => n.to_string(),
        _ => String::new(),
    }
}

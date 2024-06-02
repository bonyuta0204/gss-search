use core::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct SheetData {
    data: Vec<Vec<Value>>,
}

pub struct SheetRow<'a> {
    data: &'a Vec<Value>,
}

impl SheetData {
    pub fn new(data: Vec<Vec<Value>>) -> Self {
        Self { data }
    }

    pub fn iter(&self) -> impl Iterator<Item = SheetRow> {
        self.data.iter().map(|row| SheetRow::new(row))
    }
}

impl<'a> SheetRow<'a> {
    pub fn new(data: &'a Vec<Value>) -> Self {
        Self { data }
    }

    pub fn pretty_print(&self) {
        for cell in self.data.iter() {
            match cell {
                Value::String(s) => println!("{}", s),
                Value::Number(n) => println!("{}", n),
                _ => {}
            }
        }
        println!();
    }
}

impl fmt::Display for SheetData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.iter() {
            write!(f, "{}\n", row)?
        }
        Ok(())
    }
}

impl<'a> fmt::Display for SheetRow<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for cell in self.data.iter() {
            match cell {
                Value::String(s) => write!(f, "{:<20} |", s.to_string())?,
                Value::Number(n) => write!(f, "{:<20} |", n.to_string())?,
                _ => {}
            }
        }
        Ok(())
    }
}

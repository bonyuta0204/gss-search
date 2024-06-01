// src/cache.rs

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Serialize, Deserialize)]
pub struct Cache {
    pub data: Vec<Vec<String>>,
}

impl Cache {
    pub fn load(path: &str) -> Result<Self, io::Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let cache: Cache = serde_json::from_str(&contents)?;
        Ok(cache)
    }

    pub fn save(&self, path: &str) -> Result<(), io::Error> {
        let contents = serde_json::to_string(&self)?;
        let mut file = File::create(path)?;
        file.write_all(contents.as_bytes())
    }
}

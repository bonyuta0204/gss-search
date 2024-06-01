// src/cache.rs

use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;

pub fn load_from_storage<D: DeserializeOwned>(path: &str) -> Result<D, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data: D = serde_json::from_str(&contents)?;
    Ok(data)
}

pub fn save_to_storage<D: Serialize>(path: PathBuf, data: &D) -> Result<(), io::Error> {
    let contents = serde_json::to_string(data)?;
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())
}

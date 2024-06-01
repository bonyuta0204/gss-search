// src/cache.rs

use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

pub fn load_from_storage<D: DeserializeOwned>(path: &Path) -> Result<D, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data: D = serde_json::from_str(&contents)?;
    Ok(data)
}

pub fn save_to_storage<D: Serialize>(path: &Path, data: &D) -> Result<(), io::Error> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("Failed to create directories");
        }
    }
    let contents = serde_json::to_string(data)?;
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())
}

use std::fs::{self, File, OpenOptions};

use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    fmt::{
        format::{Format, Json, JsonFields},
        time::{uptime, Uptime},
    },
    FmtSubscriber,
};

use crate::path_builder::PathBuilder;

pub fn build_subscriber(
) -> FmtSubscriber<JsonFields, Format<Json, Uptime>, LevelFilter, std::fs::File> {
    let log_file_path = PathBuilder::new().log_file();

    if let Some(parent) = log_file_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create log directory");
    }

    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
        .expect("Failed to open or create log file");

    tracing_subscriber::fmt()
        .with_timer(uptime())
        .json()
        .with_writer(log_file)
        .finish()
}

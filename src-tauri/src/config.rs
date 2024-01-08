use std::fs::{self};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub midi_output_port: Option<String>,
    pub auto_load_file: Option<String>,
}

pub fn load_config() -> Config {
    let file = fs::read_to_string(r".\target\debug\config.toml").unwrap();
    let decoded: Config = toml::from_str(&file).unwrap();

    decoded
}

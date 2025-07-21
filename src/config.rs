use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub claude_api_key: String,
    pub openai_api_key: String,
}

impl Config {
    pub fn load_from_file(filename: &str) -> Self {
        let config_str = fs::read_to_string(filename).expect("Failed to read config file.");
        serde_json::from_str(&config_str).expect("Failed to parse config file.")
    }
}


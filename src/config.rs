use crate::models::AppSettings;
use std::fs;

pub type Config = AppSettings;

impl Config {
    pub fn load_from_file(filename: &str) -> Self {
        match fs::read_to_string(filename) {
            Ok(config_str) => {
                serde_json::from_str(&config_str).unwrap_or_else(|_| {
                    println!("Warning: Invalid config file, using defaults");
                    Self::default()
                })
            }
            Err(_) => {
                println!("Warning: Config file not found, using defaults");
                Self::default()
            }
        }
    }
    
    pub fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config_str = serde_json::to_string_pretty(self)?;
        fs::write(filename, config_str)?;
        Ok(())
    }
}


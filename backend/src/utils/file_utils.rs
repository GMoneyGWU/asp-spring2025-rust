use std::fs;
use std::path::Path;
use log::error;

// Load data from JSON files
pub fn load_data(file_path: &str) -> Result<String, std::io::Error> {
    if Path::new(file_path).exists() {
        fs::read_to_string(file_path)
    } else {
        Ok("[]".to_string()) // Return empty array if file doesn't exist
    }
}

// Load data from JSON files with generic parsing
pub fn load_json_file<T: serde::de::DeserializeOwned>(file_path: &str) -> Option<T> {
    if !Path::new(file_path).exists() {
        error!("File does not exist: {}", file_path);
        return None;
    }

    match fs::read_to_string(file_path) {
        Ok(data) => {
            match serde_json::from_str(&data) {
                Ok(parsed) => Some(parsed),
                Err(e) => {
                    error!("Error parsing JSON from {}: {}", file_path, e);
                    None
                }
            }
        },
        Err(e) => {
            error!("Error reading file {}: {}", file_path, e);
            None
        }
    }
}

// Generate a simple unique ID
pub fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
    
    format!("{}", now)
}
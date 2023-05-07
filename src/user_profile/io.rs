use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use dirs::home_dir;
use serde_json;

use crate::user_profile::config::Config;

const USER_PROFILE: &str = ".aita";
const USER_CONFIG: &str = "configuration";
const USER_FILE: &str = "user_profile.json";

pub fn user_profile_exists() -> bool {
    let mut json_file_path = match home_dir() {
        Some(path) => PathBuf::from(path),
        None => panic!("Could not find the user's home directory"),
    };
    json_file_path = json_file_path
        .join(USER_PROFILE)
        .join(USER_CONFIG)
        .join(USER_FILE);

    return json_file_path.exists() && json_file_path.is_file()
}

pub fn write_user_settings(config: Config) -> Result<(), Box<dyn Error>> {
    // Construct the JSON file path
    let mut json_file_path = match home_dir() {
        Some(path) => PathBuf::from(path),
        None => panic!("Could not find the user's home directory"),
    };
    json_file_path = json_file_path
        .join(USER_PROFILE)
        .join(USER_CONFIG)
        .join(USER_FILE);

    // Create parent directories if they don't exist
    if let Some(parent_dir) = json_file_path.parent() {
        fs::create_dir_all(parent_dir)?;
    }

    let json_contents = serde_json::to_string_pretty(&config)?;
    let mut file = fs::File::create(json_file_path)?;
    file.write_all(json_contents.as_bytes())?;
    Ok(())
}
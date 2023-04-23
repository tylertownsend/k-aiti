use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use dirs::home_dir;
use serde_json;

use crate::user_profile::config::Config;

const USER_PROFILE: & str = ".aita/configuration/user_profile.json";

pub fn user_profile_exists() -> bool {
    let mut profile_file_path = match home_dir() {
        Some(path) => PathBuf::from(path),
        None => panic!("Could not find the user's home directory"),
    };
    profile_file_path.push(USER_PROFILE);

    return profile_file_path.exists() && profile_file_path.is_file();
}

// pub fn read_user_settings() -> Result<Config, Box<dyn Error>> {
//     // Construct the JSON file path
//     let mut json_file_path = match home_dir() {
//         Some(path) => PathBuf::from(path),
//         None => panic!("Could not find the user's home directory"),
//     };
//     json_file_path.push(USER_PROFILE);

//     // Read the JSON file's contents
//     let json_contents = fs::read_to_string(json_file_path).expect("Could not read the JSON file");

//     // Deserialize the JSON contents into a `Config` struct
//     let config: Config = serde_json::from_str(&json_contents)?;
//     Ok(config)
// }

pub fn write_user_settings(config: Config) -> Result<(), Box<dyn Error>> {
    // Construct the JSON file path
    let mut json_file_path = match home_dir() {
        Some(path) => PathBuf::from(path),
        None => panic!("Could not find the user's home directory"),
    };
    json_file_path.push(USER_PROFILE);

    let json_contents = serde_json::to_string_pretty(&config)?;
    let mut file = fs::File::create(json_file_path)?;
    file.write(json_contents.as_bytes())?;
    Ok(())
}
// src/config_manager.rs
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::error::Error;

pub trait ConfigTrait: serde::Serialize + serde::de::DeserializeOwned {
    fn config_directory() -> &'static str;
    fn config_filename() -> &'static str;

    fn config_file_path() -> PathBuf {
        let mut path = dirs::home_dir().expect("Could not find the user's home directory");
        path.push(Self::config_directory());
        path.push(Self::config_filename());
        path
    }

    fn config_exists() -> bool {
        let path = Self::config_file_path();
        path.exists() && path.is_file()
    }

    fn read() -> Result<Self, Box<dyn Error>> {
        let path = Self::config_file_path();
        let contents = fs::read_to_string(&path)?;
        let config: Self = serde_json::from_str(&contents)?;
        Ok(config)
    }

    fn write(&self) -> Result<(), Box<dyn Error>> {
        let path = Self::config_file_path();
        if let Some(parent_dir) = path.parent() {
            fs::create_dir_all(parent_dir)?;
        }

        let contents = serde_json::to_string_pretty(self)?;
        let mut file = fs::File::create(path)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }
}
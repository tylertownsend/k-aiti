use super::super::config_trait::ConfigTrait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Application {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelConfig {
    pub id: String,
    pub name: String,
    pub config: serde_json::Value,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InteractionModes {
    pub completion: Mode,
    pub chat: Mode,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mode {
    pub id: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SettingsConfig {
    pub application: Application,
    pub models: Vec<ModelConfig>,
    pub modes: InteractionModes,
}

impl ConfigTrait for SettingsConfig {
    fn config_directory() -> &'static str {
        ".k-aiti/configuration"
    }

    fn config_filename() -> &'static str {
        "settings.json"
    }
}
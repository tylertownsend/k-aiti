use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Application {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    pub id: String,
    pub name: String,
    pub config: serde_json::Value,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Modes {
    pub completion: Mode,
    pub chat: Mode,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mode {
    pub id: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub application: Application,
    pub models: Vec<Model>,
    pub modes: Modes,
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    pub name: String,
    pub version: String,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModelConfig {
    OpenAIGPT { max_tokens: u32, temperature: f64, top_p: f64 },
    // VideoGen { frame_rate: u32, resolution: String, duration: u32 },
    // ImageGen { width: u32, height: u32, num_samples: u32 },
    // AgentGPT { max_tokens: u32, temperature: f64, top_p: f64 },
    // CustomPythonModel { custom_script: String, input_size: u32, output_size: u32, num_layers: u32 },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub name: String,
    pub config: ModelConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mode {
    pub id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub application: Application,
    pub models: Vec<Model>,
    pub modes: HashMap<String, Mode>,
}
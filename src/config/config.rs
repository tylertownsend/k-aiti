use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Application {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ModelConfig {
    OpenAIGPT { max_tokens: u16, temperature: f32, top_p: f64, n: u8, model: String, },
    // VideoGen { frame_rate: u32, resolution: String, duration: u32 },
    // ImageGen { width: u32, height: u32, num_samples: u32 },
    // AgentGPT { max_tokens: u32, temperature: f64, top_p: f64 },
    // CustomPythonModel { custom_script: String, input_size: u32, output_size: u32, num_layers: u32 },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    pub id: String,
    pub name: String,
    pub config: ModelConfig,
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
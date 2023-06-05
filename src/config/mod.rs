mod config;
mod model_selectors;
pub mod config_manager;

use std::{error::Error, vec};

pub use config::{ Application, Config, Mode, Model };
pub use model_selectors::{get_model_by_mode, ModeSelection};
use serde::{Deserialize, Serialize};

use self::config_manager::ConfigTrait;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GptConfig {
    max_tokens: u16,
    n: u8,
    temperature: f32,
    model: String,
}

pub fn settings_setup() -> Result<(), Box<dyn Error>> {
    let gpt = GptConfig {
        model: String::from("gpt-3.5-turbo"),
        max_tokens: 100,
        n: 1,
        temperature: 0.9
    };
    let config = Config {
        application: {
            Application { 
                name: String::from("aita"), 
                version: String::from("0.0.1")
            }
        },
        models: vec![
            {
                Model {
                    id: String::from("chatgpt"),
                    name: String::from("ChatGPT"),
                    config: serde_json::to_value(gpt)?
                }
            }
        ],
        modes: config::Modes {
            completion: Mode {
                id: String::from("chatgpt")
            },
            chat: Mode {
                id: String::from("chatgpt")
            }
        }
    };
    config.write()?;
    Ok(())
}
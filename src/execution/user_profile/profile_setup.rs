use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::config::{
    ConfigTrait, 
    user::settings::{Application, ModelConfig, SettingsConfig, Mode, InteractionModes }
};
use super::config::Config as ProfileConfig;

pub fn validate() -> Result<bool, Box<dyn Error>> {
    if ProfileConfig::config_exists() {
        return Ok(false);
    }
    Ok(true)
}

pub struct SetupResult {
    pub abort: bool
}

pub fn setup() -> Result<SetupResult, Box<dyn Error>> {
    let result = profile_setup()?;
    settings_setup()?;
    Ok(result)
}

fn profile_setup() -> Result<SetupResult, Box<dyn Error>> {
    let mut result = SetupResult { abort: false };

    let env_var_handler = super::environment_variables::get_environment_variable_handler()?;
    let created_profile = super::profile_setup_menu::run(&env_var_handler)?;
    if created_profile.abort {
        result.abort = true;
        return Ok(result)
    }

    let api_keys_to_add = created_profile.clone()
        .accounts.clone().into_iter()
        .filter(|account| account.create_env_var == true)
        .map(|account| {
            super::environment_variables::EnvVar { 
                name: account.env_var_name.clone(),
                value: account.env_var_value.clone()
            }
        }).collect::<Vec<_>>();

    env_var_handler.update(&api_keys_to_add)?;

    let config = ProfileConfig::new(created_profile);
    config.write()?;
    Ok(result)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GptConfig {
    max_tokens: u16,
    n: u8,
    temperature: f32,
    model: String,
}

fn settings_setup() -> Result<(), Box<dyn std::error::Error>> {
    let gpt = GptConfig {
        model: String::from("gpt-3.5-turbo"),
        max_tokens: 100,
        n: 1,
        temperature: 0.9
    };
    let config = SettingsConfig {
        application: {
            Application { 
                name: String::from("k-aiti"), 
                version: String::from("0.0.1")
            }
        },
        models: vec![
            {
                ModelConfig {
                    id: String::from("chatgpt"),
                    name: String::from("ChatGPT"),
                    config: serde_json::to_value(gpt)?
                }
            }
        ],
        modes: InteractionModes {
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

pub fn welcome_message() {
    println!("k-aiti installed!");
    println!("");
    println!("To get started, try one of the following commands:");
    println!("- kaiti chat: Use the chat interface.");
    println!("- kaiti config config : Configure the interface.");
    println!("");
    println!("For more information on how to use My CLI, try running:");
    println!("kaiti --help");
}

pub fn abort_message() {
    println!("");
    println!("Operation aborted. Exiting...");
}
use std::error::Error;


use crate::config::{config_manager::ConfigTrait, settings_setup};

use self::config::{ Config };

mod config;
mod setup;
mod environment_variables;

pub fn validate() -> Result<bool, Box<dyn Error>> {
    if Config::config_exists() {
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

    let env_var_handler = environment_variables::get_environment_variable_handler()?;
    let created_profile = setup::run(&env_var_handler)?;
    if created_profile.abort {
        result.abort = true;
        return Ok(result)
    }

    let api_keys_to_add = created_profile.clone()
        .accounts.clone().into_iter()
        .filter(|account| account.create_env_var == true)
        .map(|account| {
            environment_variables::EnvVar { 
                name: account.env_var_name.clone(),
                value: account.env_var_value.clone()
            }
        }).collect::<Vec<_>>();

    env_var_handler.update(&api_keys_to_add)?;

    let config = Config::new(created_profile);
    config.write()?;
    Ok(result)
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
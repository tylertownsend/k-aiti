use std::error::Error;


use crate::config::{config_manager::ConfigTrait, settings_setup};

use self::config::{ Config };

mod io;
mod config;
mod setup;
mod environment_variables;

pub fn validate() -> Result<bool, Box<dyn Error>> {
    if Config::config_exists() {
        return Ok(false);
    }
    Ok(true)
}

pub fn setup() -> Result<(), Box<dyn Error>> {
    profile_setup()?;
    settings_setup()?;
    Ok(())
}

fn profile_setup() -> Result<(), Box<dyn Error>> {
    let env_var_handler = environment_variables::get_environment_variable_handler()?;
    let created_profile = setup::run(&env_var_handler)?;

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
    Ok(())
}

pub fn welcome() {
    println!("");
    println!("Welcome to My CLI!");
    println!("");
    println!("To get started, try one of the following commands:");
    println!("- my_cli chat   : Use the chat interface.");
    println!("- my_cli config : Configure the interface.");
    println!("");
    println!("For more information on how to use My CLI, try running:");
    println!("my_cli --help");
}
use std::error::Error;


use self::config::{ Config };

mod io;
mod config;
mod setup;
mod environment_variables;

pub fn validation() -> Result<bool, Box<dyn Error>> {
    if io::user_profile_exists() {
        return Ok(false);
    }

    let created_profile = setup::run()?;

    let api_keys_to_add = created_profile.clone()
        .accounts.clone().into_iter()
        .filter(|account| account.create_env_var == true)
        .map(|account| {
            environment_variables::EnvVar { 
                name: account.env_var_name.clone(),
                value: account.env_var_value.clone()
            }
        }).collect::<Vec<_>>();

    environment_variables::EnvironmentVariables::update(&api_keys_to_add)?;

    io::write_user_settings(Config::new(created_profile))?;
    Ok(true)
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
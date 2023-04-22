use std::error::Error;

mod io;
mod config;
mod setup;

pub fn validation() -> Result<bool, Box<dyn Error>> {
    if io::user_profile_exists() {
        return Ok(false);
    }

    setup::run()
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
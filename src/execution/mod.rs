use clap::ArgMatches;
use crate::config::ConfigTrait;

pub mod chat_mode;
pub mod debug_mode;
pub mod input_provider;
pub mod config_menu;
pub mod user_profile;

pub async fn process_command(matches: ArgMatches) {
    if let Some(_) = matches.subcommand_matches("search") {
    } else if let Some(_) = matches.subcommand_matches("debug") {
    } else if let Some(_) = matches.subcommand_matches("chat") { 
        start_chat().await;
    } else if let Some(_) = matches.subcommand_matches("config") {
        start_config_menu().await;
    }else {
    }
}

async fn start_chat() {
    // let chat_history_path = "chat_history.txt";

    // let chat_history = if Path::new(chat_history_path).exists() {
    //     let content = std::fs::read_to_string(chat_history_path).expect("Failed to read chat history");
    //     content.lines().map(|line| line.to_string()).collect()
    // } else {
    //     Vec::new()
    // };
    let config = match crate::config::user::settings::SettingsConfig::read() {
        Ok(path) => path,
        Err(error) => {
            panic!("{}", error);
        }
    };
    let c_model = match crate::config::get_model_by_mode(&config, crate::config::ModeSelection::Chat) {
        Some(model) => model,
        _ => {
            println!("Corrupted settings file found.");
            return;
        }
    };
    match chat_mode::run_chat_mode(c_model).await {
        Ok(_) => println!("Chat ended."),
        Err(e) => eprintln!("Error: {}", e),
    };

    // File::create(chat_history_path)
    //     .and_then(|mut file| file.write_all(chat_history.join(STOP_PHRASE).as_bytes()))
    //     .expect("Failed to save chat history");
}

async fn start_config_menu() {
    let mut config= match crate::config::user::settings::SettingsConfig::read() {
        Ok(instance) => instance,
        Err(_) => panic!("Error reading configuration!"),
    };
    match config_menu::run_config_menu(&mut config).await {
        Ok(()) => println!("\nConfiguration menu closed successfully"),
        Err(e) => eprintln!("Error running configuration menu: {}", e),
    }
    match config.write() {
        Ok(()) => println!("Configuration updates written successfully"),
        Err(_) => eprintln!("Error updating configuration settings")
    }
}
use clap::ArgMatches;
use crate::config::config_manager::ConfigTrait;

pub mod chat_mode;
pub mod debug_mode;
pub mod input_provider;
pub mod config_menu;

pub async fn process_command(matches: ArgMatches) {
    if let Some(_) = matches.subcommand_matches("search") {
    } else if let Some(_) = matches.subcommand_matches("debug") {
        
        // if let Some(_) = debug_matches.value_of("error") {
        // } else {
        //     // Terminal capture code here
        //     match debug_mode::run_debug_mode(gpt_client).await {
        //         Ok(_) => println!("Debug Mode Run."),
        //         Err(e) => eprintln!("Error: {}", e),
        //     };
        // }
    } else if let Some(_) = matches.subcommand_matches("chat") {
        // let chat_history_path = "chat_history.txt";

        // let chat_history = if Path::new(chat_history_path).exists() {
        //     let content = std::fs::read_to_string(chat_history_path).expect("Failed to read chat history");
        //     content.lines().map(|line| line.to_string()).collect()
        // } else {
        //     Vec::new()
        // };
        let config = match crate::config::Config::read() {
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
    } else if let Some(_) = matches.subcommand_matches("config") {
        let mut config= match crate::config::Config::read() {
            Ok(instance) => instance,
            Err(error) => panic!("Error reading configuration!"),
        };
        match config_menu::run_config_menu(&mut config).await {
            Ok(()) => println!("Configuration menu closed successfully"),
            Err(e) => eprintln!("Error running configuration menu: {}", e),
        }
        match config.write() {
            Ok(()) => println!("Configuration updates written successfully"),
            Err(_) => eprintln!("Error updating configuration settings")
        }
    }else {
    }
}
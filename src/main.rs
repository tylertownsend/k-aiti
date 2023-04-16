use aita::ai::open_ai_gpt::{GptClient, STOP_PHRASE};
use aita::render::chat_mode::{self, run_chat_mode};
use clap::{App, Arg, SubCommand};
use std::env;
use std::io::{self, Write};
use std::fs::File;
use std::path::Path;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let matches = App::new("aita")
        .version("0.1.0")
        .author("Tyler Townsend")
        .about("A smart terminal-based assistant to help engineers resolve errors and find relevant information")
        .subcommand(
            SubCommand::with_name("search")
                .about("Searches for information using the provided input string")
                .arg(
                    Arg::new("query")
                        .required(true)
                        // .about("The input string to search")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("debug")
                .about("Analyzes an error message")
                .arg(
                    Arg::new("error")
                        .short('e')
                        .long("error")
                        .value_name("ERROR_MESSAGE")
                        // .about("Provides an error message to analyze")
                        .takes_value(true),
                ),
        )
        .subcommand(SubCommand::with_name("chat").about("Enter chat mode with the GPT API"))
        .get_matches();

    let mut gpt_client = GptClient::new(api_key, 1000, 1, 0.8, String::from("gpt-3.5-turbo"), None);

    if let Some(search_matches) = matches.subcommand_matches("search") {
        if let Some(query) = search_matches.value_of("query") {
            // let request = ClientRequest {
            //     prompt: query.to_string(),
            //     chat_log: None, // No chat log needed for search
            // };
    
            // match gpt_client.generate_response(request).await {
            //     Ok(response) => println!("GPT response: {}", response),
            //     Err(e) => eprintln!("Error: {}", e),
            // }
        }
    } else if let Some(debug_matches) = matches.subcommand_matches("debug") {
        if let Some(error_message) = debug_matches.value_of("error") {
            // let request = ClientRequest {
            //     prompt: format!("How to fix the following error in code: {}", error_message),
            //     chat_log: None, // No chat log needed for debug
            // };
    
            // match gpt_client.generate_response(request).await {
            //     Ok(response) => println!("GPT response: {}", response),
            //     Err(e) => eprintln!("Error: {}", e),
            // }
        } else {
            // Terminal capture code here
            println!("Terminal capture not implemented yet");
        }
    } else if let Some(_) = matches.subcommand_matches("chat") {
        let chat_history_path = "chat_history.txt";

        let mut chat_history = if Path::new(chat_history_path).exists() {
            let content = std::fs::read_to_string(chat_history_path).expect("Failed to read chat history");
            content.lines().map(|line| line.to_string()).collect()
        } else {
            Vec::new()
        };

        match run_chat_mode(gpt_client).await {
            Ok(_) => println!("Chat ended."),
            Err(e) => eprintln!("Error: {}", e),
        }

        // File::create(chat_history_path)
        //     .and_then(|mut file| file.write_all(chat_history.join(STOP_PHRASE).as_bytes()))
        //     .expect("Failed to save chat history");
    }
}
use aita::ai::open_ai_gpt::{GptClient, GptRequest};
use clap::{App, Arg, SubCommand};
use std::env;

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
        .get_matches();

    let mut gpt_client = GptClient::new(api_key);

    if let Some(search_matches) = matches.subcommand_matches("search") {
        if let Some(query) = search_matches.value_of("query") {
            let request = GptRequest {
                prompt: query.to_string(),
                max_tokens: 100,
                n: 1,
                temperature: 0.8,
                model: String::from("text-davinci-003"),
                chat_log: None, // No chat log needed for search
                stop: None, // No stop phrase needed for search
            };
    
            match gpt_client.generate_response(request).await {
                Ok(response) => println!("GPT response: {}", response),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    } else if let Some(debug_matches) = matches.subcommand_matches("debug") {
        if let Some(error_message) = debug_matches.value_of("error") {
            let request = GptRequest {
                prompt: format!("How to fix the following error in code: {}", error_message),
                max_tokens: 100,
                n: 1,
                temperature: 0.8,
                model: String::from("text-davinci-003"),
                chat_log: None, // No chat log needed for debug
                stop: Some(String::from("import")), // Stop on "import" keyword
            };
    
            match gpt_client.generate_response(request).await {
                Ok(response) => println!("GPT response: {}", response),
                Err(e) => eprintln!("Error: {}", e),
            }
        } else {
            // Terminal capture code here
            println!("Terminal capture not implemented yet");
        }
    }
}
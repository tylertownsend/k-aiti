use aita::ai::open_ai_gpt::{GptClient, ClientRequest, STOP_PHRASE};
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

    let mut gpt_client = GptClient::new(api_key);

    if let Some(search_matches) = matches.subcommand_matches("search") {
        if let Some(query) = search_matches.value_of("query") {
            let request = ClientRequest {
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
            let request = ClientRequest {
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
    } else if let Some(_) = matches.subcommand_matches("chat") {
        let chat_history_path = "chat_history.txt";

        let mut chat_history = if Path::new(chat_history_path).exists() {
            let content = std::fs::read_to_string(chat_history_path).expect("Failed to read chat history");
            content.lines().map(|line| line.to_string()).collect()
        } else {
            Vec::new()
        };

        loop {
            print!("You: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();

            if input.to_lowercase() == "chat::quit" {
                break;
            }

            let request = ClientRequest {
                prompt: input.clone(),
                max_tokens: 1000,
                n: 1,
                temperature: 0.8,
                model: String::from("text-davinci-003"),
                chat_log: Some(chat_history.clone()), // Update this line
                stop: Some(String::from(STOP_PHRASE)),
            };
            match gpt_client.generate_response(request).await {
                Ok(response) => {
                    let output = format!("GPT: {}", response);
                    println!("{}", output);
                    chat_history.push(format!("You: {}", input));
                    chat_history.push(output);
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }

        File::create(chat_history_path)
            .and_then(|mut file| file.write_all(chat_history.join(STOP_PHRASE).as_bytes()))
            .expect("Failed to save chat history");
    }
}
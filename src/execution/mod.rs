use clap::ArgMatches;

use crate::ai::open_ai_gpt::GptClient;

pub mod chat_mode;
pub mod input_provider;


pub async fn process_command(matches: ArgMatches, gpt_client: GptClient) {
    if let Some(search_matches) = matches.subcommand_matches("search") {
        // if let Some(query) = search_matches.value_of("query") {
        //     let request = ClientRequest {
        //         prompt: query.to_string(),
        //         chat_log: None, // No chat log needed for search
        //     };
    
        //     match gpt_client.generate_response(request).await {
        //         Ok(response) => println!("GPT response: {}", response),
        //         Err(e) => eprintln!("Error: {}", e),
        //     }
        // }
    } else if let Some(debug_matches) = matches.subcommand_matches("debug") {
        // if let Some(error_message) = debug_matches.value_of("error") {
        //     let request = ClientRequest {
        //         prompt: format!("How to fix the following error in code: {}", error_message),
        //         chat_log: None, // No chat log needed for debug
        //     };
    
        //     match gpt_client.generate_response(request).await {
        //         Ok(response) => println!("GPT response: {}", response),
        //         Err(e) => eprintln!("Error: {}", e),
        //     }
        // } else {
        //     // Terminal capture code here
        //     println!("Terminal capture not implemented yet");
        // }
    } else if let Some(_) = matches.subcommand_matches("chat") {
        // let chat_history_path = "chat_history.txt";

        // let chat_history = if Path::new(chat_history_path).exists() {
        //     let content = std::fs::read_to_string(chat_history_path).expect("Failed to read chat history");
        //     content.lines().map(|line| line.to_string()).collect()
        // } else {
        //     Vec::new()
        // };

        match chat_mode::run_chat_mode(gpt_client).await {
            Ok(_) => println!("Chat ended."),
            Err(e) => eprintln!("Error: {}", e),
        };

        // File::create(chat_history_path)
        //     .and_then(|mut file| file.write_all(chat_history.join(STOP_PHRASE).as_bytes()))
        //     .expect("Failed to save chat history");
    } else {
    }
}
use std::{io::{self, Write, }, error::Error};
use async_openai::types::{Role, ChatCompletionRequestMessageArgs, ChatCompletionRequestMessage };

use crate::ai::{open_ai_gpt::{GptClient, GptClientRequest}, ChatClient};

pub async fn run_chat_mode(gpt_client: GptClient) -> Result<(), Box<dyn Error>> {
    let mut chat_client = ChatClient::new(gpt_client);
    let mut chat_log: Vec<ChatCompletionRequestMessage> = Vec::new();

    // let mut prompt = String::new();
    // let stdin = io::stdin();
    // let mut stdout = io::stdout();

    loop {
        print!("You: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();

        if input.trim() == "stop" {
            break;
        }


        let response = chat_client.render_response(input).await?;


        // User should be added via render_response
        // let user_message = ChatCompletionRequestMessageArgs::default()
        //     .content(input.to_string())
        //     .role(Role::User)
        //     .build()?;
        // chat_log.push(user_message);

        // prompt.clear();
    }
    Ok(())
}
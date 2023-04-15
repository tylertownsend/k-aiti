use std::{io::{self, Write, }, error::Error};
use async_openai::types::{Role, ChatCompletionRequestMessageArgs, ChatCompletionRequestMessage };

use crate::ai::{open_ai_gpt::{GptClient, ClientRequest}, ChatClient};

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

        let client_request = ClientRequest {
            prompt: input.trim().to_string(),
            chat_log: Some(chat_log.clone()),
        };

        let response = chat_client.render_response(&client_request).await?;

        let response_message = ChatCompletionRequestMessageArgs::default()
            .content(response)
            .role(Role::Assistant)
            .build()?;
        chat_log.push(response_message);

        let user_message = ChatCompletionRequestMessageArgs::default()
            .content(input.to_string())
            .role(Role::User)
            .build()?;
        chat_log.push(user_message);

        // prompt.clear();
    }
    Ok(())
}
use std::{io::{self, Write, }, error::Error};
use async_openai::types::{Role, ChatCompletionRequestMessageArgs, ChatCompletionRequestMessage };

use crate::ai::{open_ai_gpt::{GptClient, GptClientRequest}, ChatClient};

pub async fn run_chat_mode(gpt_client: GptClient) -> Result<(), Box<dyn Error>> {
    let mut chat_client = ChatClient::new(gpt_client);

    loop {
        let input = chat_client.render_input().await?;

        if input.trim() == "stop" {
            break;
        }

        let response = chat_client.render_response(input).await?;
    }
    Ok(())
}
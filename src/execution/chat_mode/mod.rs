use std::error::Error;

use crate::ai::{open_ai_gpt::GptClient, ChatClient};
use crate::render::terminal_renderer::TerminalRenderer;

pub async fn run_chat_mode(gpt_client: GptClient) -> Result<(), Box<dyn Error>> {
    let renderer = TerminalRenderer::new();
    let mut chat_client = ChatClient::new(gpt_client, renderer);

    loop {
        let input = chat_client.render_input().await?;

        if input.trim() == "stop" {
            break;
        }

        chat_client.render_response(input).await?;
    }
    Ok(())
}
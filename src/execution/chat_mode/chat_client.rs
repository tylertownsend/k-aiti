use std::error::Error;

use crate::ai::{
    chat_model::{ChatModel, ChatModelRequest},
    chat_types::{ChatCompletionRequestMessage, Role}
};
use super::terminal_renderer::TerminalRenderer;
use crate::execution::input_provider::get_user_input;

pub struct ChatClient {
    chat_model: Box<dyn ChatModel>,
    chat_log: Vec<crate::ai::chat_types::ChatCompletionRequestMessage>,
}

impl ChatClient {
    pub fn new(chat_model: Box<dyn ChatModel>,) -> Self {
        Self { chat_model, chat_log: Vec::new() }
    }

    pub async fn run(&mut self, renderer: &mut TerminalRenderer) -> Result<(), Box<dyn Error>> {
        loop {
            let input = self.get_input(renderer).await?;
            if input.trim() == "stop" {
                break;
            }
            self.handle_response(input, renderer).await?;
        }
        Ok(())
    }

    pub async fn get_input(&mut self, renderer: &mut TerminalRenderer) -> Result<String, Box<dyn Error>> {
        renderer.render_user("You");
        get_user_input().await
    }

    pub async fn handle_response(&mut self, user_input: String, renderer: &mut TerminalRenderer) -> Result<String, Box<dyn Error>> {
        let user_message = ChatCompletionRequestMessage {
            content: user_input,
            role: Role::User,
            name: None
        };
        self.chat_log.push(user_message);
        let client_request = ChatModelRequest { messages: self.chat_log.clone() };

        let stream = self.chat_model.create_response_stream(&client_request).await?;

        // Delegate to renderer to process the stream
        let response_string = renderer.render_stream(stream).await?;

        let response_message = ChatCompletionRequestMessage {
            content: response_string.clone(),
            role: Role::Assistant,
            name: None
        };
        self.chat_log.push(response_message);

        Ok(response_string)
    }
}
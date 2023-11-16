use std::error::Error;

use async_openai::types::ChatCompletionRequestMessage;

use crate::ai::chat_model::{ChatModel, ChatModelRequest};
use crate::render::terminal_renderer::TerminalRenderer;
use crate::execution::input_provider::get_user_input;

pub struct ChatClient {
    chat_model: Box<dyn ChatModel>,
    chat_log: Vec<ChatCompletionRequestMessage>,
}

impl ChatClient {
    pub fn new(chat_model: Box<dyn ChatModel>,) -> Self {
        Self { chat_model, chat_log: Vec::new() }
    }

    pub async fn get_input(&mut self, renderer: &mut TerminalRenderer) -> Result<String, Box<dyn Error>> {
        renderer.render_user("You");
        get_user_input().await
    }

    pub async fn handle_response(&mut self, user_input: String, renderer: &mut TerminalRenderer) -> Result<String, Box<dyn Error>> {
        let user_message = self.chat_model.create_user_message(user_input)?;
        self.chat_log.push(user_message);
        let client_request = ChatModelRequest { messages: self.chat_log.clone() };

        let stream = self.chat_model.create_response_stream(&client_request).await?;

        // Delegate to renderer to process the stream
        let response_string = renderer.render_stream(stream).await?;

        let response_message = self.chat_model.create_assistant_message(response_string.clone())?;
        self.chat_log.push(response_message);

        Ok(response_string)
    }
}
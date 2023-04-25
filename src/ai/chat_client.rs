use crate::ai::chat_model::{ChatModel, ChatModelRequest};
use crate::render::terminal_renderer::{TerminalRenderer, TextState};
use crate::execution::input_provider::get_user_input;
use std::error::Error;
use std::io::{stdout, Write};
use async_openai::types::{ChatCompletionRequestMessage};
use crossterm::style::Color;
use futures::StreamExt;

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

        let mut response_string = String::new();
        let mut lock = stdout().lock();

        let user_message = self.chat_model.create_user_message(user_input)?;
        self.chat_log.push(user_message);
        let client_request = ChatModelRequest{ messages: self.chat_log.clone() };

        let mut stream = self.chat_model.create_response_stream(&client_request).await?;

        renderer.print_entity("AI ", Color::Green);
        // TODO: move TextState to renderer completely
        let mut state = TextState::new();
        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    response.choices.iter().for_each(|chat_choice| {
                        if let Some(ref content) = chat_choice.delta.content {
                            renderer.print_content(& mut lock, content, & mut state).unwrap();
                            response_string.push_str(content);
                        }
                    });
                }
                Err(err) => {
                    renderer.print_error(&mut lock, err.to_string())
                }
            }
            stdout().flush()?;
        }
        println!("\n");
        let response_message = self.chat_model
            .create_assistant_message(response_string.clone())?;
        self.chat_log.push(response_message);
        Ok(response_string)
    }
}
pub mod open_ai_gpt;

use crate::render::terminal_renderer::{TerminalRenderer, TextState};
use crate::execution::input_provider::get_user_input;
use std::error::Error;
use std::io::{stdout, Write};
use async_openai::types::ChatCompletionRequestMessage;
use futures::stream::StreamExt;
use crossterm::style::Color;


use self::open_ai_gpt::{GptClientRequest, GptClient};

pub struct ChatClient {
    gpt_client: GptClient,
    chat_log: Vec<ChatCompletionRequestMessage>,
    renderer: TerminalRenderer
}

impl ChatClient {
    pub fn new(gpt_client: GptClient, renderer: TerminalRenderer) -> Self {
        Self { gpt_client, chat_log: Vec::new(), renderer }
    }

    pub async fn render_input(&mut self) -> Result<String, Box<dyn Error>> {
        self.renderer.print_entity("You", Color::Cyan);
        get_user_input().await
    }

    pub async fn render_response(&mut self, user_input: String) -> Result<String, Box<dyn Error>> {
        let mut response_string = String::new();
        let mut lock = stdout().lock();

        let user_message = self.gpt_client.clone().create_user_message(user_input)?;
        self.chat_log.push(user_message);
        let client_request = GptClientRequest { messages: self.chat_log.clone() };

        let mut stream = self.gpt_client.generate_response(&client_request).await?;

        self.renderer.print_entity("AI ", Color::Green);
        // TODO: move TextState to renderer completely
        let mut state = TextState::new();
        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    response.choices.iter().for_each(|chat_choice| {
                        if let Some(ref content) = chat_choice.delta.content {
                            self.renderer.print_content(& mut lock, content, & mut state).unwrap();
                            response_string.push_str(content);
                        }
                    });
                }
                Err(err) => {
                    self.renderer.print_error(&mut lock, err.to_string())
                }
            }
            stdout().flush()?;
        }
        println!("\n");
        let response_message = self.gpt_client
            .clone()
            .create_assistant_message(response_string.clone())?;
        self.chat_log.push(response_message);
        Ok(response_string)
    }
}
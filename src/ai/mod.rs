pub mod open_ai_gpt;

use std::error::Error;
use std::io::{stdout, Write};
use async_openai::types::ChatCompletionRequestMessage;
use futures::stream::StreamExt;
use termimad::{MadSkin};
// use ansi_term::{Style, Color};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor, SetAttribute, Attribute},
};


use self::open_ai_gpt::{GptClientRequest, GptClient};

pub struct ChatClient {
    gpt_client: GptClient,
    chat_log: Vec<ChatCompletionRequestMessage>
}


struct TextState {
    in_code_block: bool,
    buffer: String,
    backticks_count: i32
}

impl TextState {
    pub fn new() -> Self {
        Self { in_code_block: false, buffer: String::new(), backticks_count: 0 }
    }
}

impl ChatClient {
    pub fn new(gpt_client: GptClient) -> Self {
        Self { gpt_client, chat_log: Vec::new() }
    }

    pub async fn render_input(&mut self) -> Result<String, Box<dyn Error>> {
        // let mut lock = stdout().lock();
        execute!(std::io::stdout(), SetForegroundColor(Color::Cyan), SetAttribute(Attribute::Bold), Print("You: "), ResetColor).unwrap();
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        Ok(input.trim().to_string())
    }

    pub async fn render_response(&mut self, user_input: String) -> Result<String, Box<dyn Error>> {
        let mut response_string = String::new();
        let mut lock = stdout().lock();

        let user_message = self.gpt_client.clone().create_user_message(user_input)?;
        self.chat_log.push(user_message);
        let client_request = GptClientRequest { messages: self.chat_log.clone() };

        let mut stream = self.gpt_client.generate_response(&client_request).await?;

        let mut state = TextState::new();
        execute!(lock, SetForegroundColor(Color::Green), SetAttribute(Attribute::Bold), Print("AI : "), ResetColor).unwrap();
        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    response.choices.iter().for_each(|chat_choice| {
                        if let Some(ref content) = chat_choice.delta.content {
                            self.print_content(& mut lock, content, & mut state).unwrap();
                            response_string.push_str(content);
                        }
                    });
                }
                Err(err) => {
                    execute!(
                        lock,
                        SetForegroundColor(Color::Red),
                        Print(format!("error: {}", err)),
                        ResetColor
                    )
                    .unwrap();
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

    fn print_content(&mut self, lock: &mut std::io::StdoutLock, content: &str, state: &mut TextState) -> Result<(), Box<dyn Error>> {
        for c in content.chars() {
            if c == '`' {
                state.backticks_count += 1;
            } else {
                state.backticks_count = 0;
            }

            if state.backticks_count == 3 {
                state.backticks_count = 0;
                state.in_code_block = !state.in_code_block;
                if !state.in_code_block {
                    execute!(lock, ResetColor).unwrap();
                }
                continue;
            }

            if !state.in_code_block || state.backticks_count > 0 {
                write!(lock, "{}", c).unwrap();
            } else {
                execute!(lock, SetForegroundColor(Color::DarkGreen), SetBackgroundColor(Color::Black), Print(c), ResetColor).unwrap();
            }
        }
        Ok(())
    }
}
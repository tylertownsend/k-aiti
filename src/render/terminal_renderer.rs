use std::pin::Pin;
use std::{error::Error, io::stdout};
use std::io::Write;
use async_openai::error::OpenAIError;
use async_openai::types::{CreateChatCompletionStreamResponse};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor, SetAttribute, Attribute},
};
use futures::Stream;
use futures::StreamExt;

pub struct TerminalRenderer {
}

pub struct TextState {
    in_code_block: bool,
    backticks_count: i32
}

impl TextState {
    pub fn new() -> Self {
        Self { in_code_block: false, backticks_count: 0 }
    }
}

impl TerminalRenderer {

    pub fn new() -> Self {
        TerminalRenderer {  }
    }

    pub fn render_user(&mut self, _: &str) {
        self.print_entity("You", Color::Cyan);
    }

    pub async fn render_content(&mut self,
        mut stream: Pin<Box<dyn Stream<Item = Result<CreateChatCompletionStreamResponse, OpenAIError>> + std::marker::Send>>
    ) -> Result<String, Box<dyn Error>> {
        let mut response_string = String::new();
        let mut state = TextState::new();

        let mut lock = stdout().lock();
        self.print_entity("AI ", Color::Green);
        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    response.choices.iter().for_each(|chat_choice| {
                        if let Some(ref content) = chat_choice.delta.content {
                            
                            print_content(&mut lock, content, & mut state).unwrap();
                            response_string.push_str(content);
                        }
                    });
                }
                Err(err) => {
                    self.print_error(&mut lock, err.to_string())
                }
            }
            stdout().flush()?;
        }
        println!("\n");
        Ok(response_string)
    }

    pub fn print_content(
        &mut self,
        lock: &mut std::io::StdoutLock,
        content: &str,
        state: &mut TextState
    ) -> Result<(), Box<dyn Error>> {
        print_content(lock, content, state)
    }

    pub fn print_entity(&mut self, entity: &str, color: Color) {
        execute!(std::io::stdout(), SetForegroundColor(color), SetAttribute(Attribute::Bold), Print(format!("{}: ", entity)), ResetColor).unwrap();
    }

    pub fn print_error(
        &mut self,
        lock: &mut std::io::StdoutLock,
        err: String
    ) {
        execute!(lock,
                SetForegroundColor(Color::Red),
                Print(format!("error: {}", err)),
                ResetColor)
                .unwrap();
}
}

fn print_content(
    lock: &mut std::io::StdoutLock,
    content: &str,
    state: &mut TextState
) -> Result<(), Box<dyn Error>> {
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

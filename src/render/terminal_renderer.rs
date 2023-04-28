use std::pin::Pin;
use std::{error::Error, io::stdout};
use std::io::Write;
use async_openai::error::OpenAIError;
use async_openai::types::{CreateChatCompletionStreamResponse};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, SetAttribute, Attribute},
};
use futures::Stream;
use futures::StreamExt;

pub struct TerminalRenderer {

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

        let mut lock = stdout().lock();
        self.print_entity("AI ", Color::Green);
        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    response.choices.iter().for_each(|chat_choice| {
                        if let Some(ref content) = chat_choice.delta.content {
                            self.render(&mut lock, content).unwrap();
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

    pub fn render(
        &mut self,
        lock: &mut std::io::StdoutLock,
        content: &str,
    ) -> Result<(), Box<dyn Error>> {
        write!(lock, "{}", content).unwrap();
        Ok(())
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
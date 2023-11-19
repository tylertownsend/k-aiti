use std::error::Error;
use std::io::stdout;
use std::io::Write;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, SetAttribute, Attribute},
};
use futures::StreamExt;

use crate::ai::chat_types::ChatCompletionStream;

pub struct TerminalRenderer {

}

impl TerminalRenderer {

    pub fn new() -> Self {
        TerminalRenderer {  }
    }

    pub fn render_user(&mut self, _: &str) {
        self.print_entity("You", Color::Cyan);
    }

    pub async fn render_stream(&mut self, mut stream: ChatCompletionStream) -> Result<String, Box<dyn Error>> {
        let mut response_string = String::new();
        let mut lock = stdout().lock();

        self.print_entity("AI ", Color::Green);

        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    for chat_choice in &response.choices {
                        if let Some(content) = &chat_choice.delta.content {
                            write!(lock, "{}", content)?;
                            response_string.push_str(content);
                        }
                    }
                },
                Err(err) => {
                    self.print_error(&mut lock, err.to_string())
                }
            }
            stdout().flush()?;
        }

        println!("\n");
        Ok(response_string)
    }
    pub fn print_entity(&mut self, entity: &str, color: Color) {
        execute!(std::io::stdout(), SetForegroundColor(color), SetAttribute(Attribute::Bold), Print(format!("{}: ", entity)), ResetColor).unwrap();
    }

    pub fn print_error(&mut self, lock: &mut std::io::StdoutLock, err: String) {
        execute!(lock,
                SetForegroundColor(Color::Red),
                Print(format!("error: {}", err)),
                ResetColor)
                .unwrap();
    }
}
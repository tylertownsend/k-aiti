use std::error::Error;
use std::io::Write;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor, SetAttribute, Attribute},
};

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
        TerminalRenderer {}
    }

    pub fn print_entity(&mut self, entity: &str, _: Color) {
        execute!(std::io::stdout(), SetForegroundColor(Color::Cyan), SetAttribute(Attribute::Bold), Print(format!("{}: ", entity)), ResetColor).unwrap();
    }

    pub fn print_content(&mut self, lock: &mut std::io::StdoutLock, content: &str, state: &mut TextState) -> Result<(), Box<dyn Error>> {
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

    pub fn print_error(&mut self, lock: &mut std::io::StdoutLock, err: String) {
        execute!(lock,
                 SetForegroundColor(Color::Red),
                 Print(format!("error: {}", err)),
                 ResetColor)
                 .unwrap();
    }
}
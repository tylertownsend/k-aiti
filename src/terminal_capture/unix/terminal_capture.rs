use std::io::{stdout, Read, Write};
use termion::get_tty;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use crate::terminal_capture::traits::TerminalCapture;

pub struct UnixTerminalCapture;

impl TerminalCapture for UnixTerminalCapture {
    fn capture_output(&self) -> Result<String, std::io::Error> {
        // TODO: Read from history
        Ok("hello".to_string())
    }
}
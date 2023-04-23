use crate::terminal_capture::traits::TerminalCapture;

pub struct UnixTerminalCapture;

impl TerminalCapture for UnixTerminalCapture {
    fn capture_output(&self) -> Result<String, std::io::Error> {
        // TODO: Read from history
        Ok("hello".to_string())
    }
}
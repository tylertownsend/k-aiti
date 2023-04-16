use std::process::Command;
use crate::terminal_capture::traits::TerminalCapture;

pub struct UnixTerminalCapture;

impl TerminalCapture for UnixTerminalCapture {
    fn capture_output(&self) -> Result<String, std::io::Error> {
        let output = Command::new("tmux")
            .arg("capture-pane")
            .arg("-p")
            .output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to capture terminal output",
            ))
        }
    }
}
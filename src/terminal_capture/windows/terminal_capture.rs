use std::process::Command;
use crate::terminal_capture::traits::TerminalCapture;

pub struct WindowsTerminalCapture;

impl TerminalCapture for WindowsTerminalCapture {
    fn capture_output(&self) -> Result<String, std::io::Error> {
        let output = Command::new("powershell")
            .arg("-NoProfile")
            .arg("-Command")
            .arg("Get-Host | Select-Object -ExpandProperty Buffer | ForEach-Object { [System.Text.Encoding]::Default.GetString($_.Character) }")
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
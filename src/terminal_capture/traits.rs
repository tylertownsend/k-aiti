pub trait TerminalCapture {
    fn capture_output(&self) -> Result<String, std::io::Error>;
}
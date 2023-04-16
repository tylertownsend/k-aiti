use std::error::Error;

use crate::ai::{open_ai_gpt::GptClient, ChatClient};
use crate::terminal_capture::capture_instance;

pub async fn run_debug_mode(gpt_client: GptClient) -> Result<(), Box<dyn Error>> {
    let renderer = TerminalRenderer::new();
    let terminal = capture_instance();
}
use std::error::Error;

use crate::{
    ai,
    render::terminal_renderer::TerminalRenderer,
    config::Model
};

pub async fn run_chat_mode(c_model: &Model) -> Result<(), Box<dyn Error>> {
    let mut renderer = TerminalRenderer::new();
    let chat_model = ai::create_chat_model(c_model)?;
    let mut chat_client = ai::ChatClient::new(chat_model);
    loop {
        let input = chat_client.get_input(&mut renderer).await?;

        if input.trim() == "stop" {
            break;
        }

        chat_client.handle_response(input, &mut renderer).await?;
    }
    Ok(())
}
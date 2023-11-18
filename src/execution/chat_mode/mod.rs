use std::error::Error;

use crate::ai::ChatModel;
use crate::config::Model;
use crate::open_ai_gpt::GptClient;
use crate::{
    ai,
    render::terminal_renderer::TerminalRenderer,
};

pub async fn run_chat_mode(c_model: &Model) -> Result<(), Box<dyn Error>> {
    let mut renderer = TerminalRenderer::new();
    let chat_model = create_chat_model(c_model)?;
    let mut chat_client = ai::ChatClient::new(chat_model);
    chat_client.run(&mut renderer).await?;
    Ok(())
}

// TODO: Implement a model path to determine the correct selection
fn create_chat_model(c_model: &Model) -> Result<Box<dyn ChatModel>, Box<dyn Error>> {
    let model = match c_model.name.as_str() {
        "ChatGPT" => {
            let config = serde_json::to_value(c_model.config.clone())?;
            let gpt_client = GptClient::new(config);
            Box::new(gpt_client) as Box<dyn ChatModel>
        }
        _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unsupported model name"))),
    };
    Ok(model)
}
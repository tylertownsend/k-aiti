use std::error::Error;

use crate::config::Model;
use crate::open_ai_gpt::GptClient;

pub mod chat;
mod chat_client;
mod chat_model;
mod stream;

pub use chat_client::ChatClient;
pub use chat_model::{ChatModel, ChatModelRequest};
pub use stream::{ChatCompletionChoice, ChatCompletionChunk, ChatCompletionDelta, CompletionStream, Role, ModelUsage};

// TODO: Implement a model path to determine the correct selection
pub fn create_chat_model(c_model: &Model) -> Result<Box<dyn ChatModel>, Box<dyn Error>> {
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
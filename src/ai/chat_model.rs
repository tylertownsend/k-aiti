use std::error::Error;

use async_trait::async_trait;

use super::{chat_types::ChatCompletionStream, chat_types::ChatCompletionRequestMessage};

#[derive(Clone)]
pub struct ChatModelRequest {
    pub messages: Vec<ChatCompletionRequestMessage>,
}

#[async_trait]
pub trait ChatModel {
    fn new(config: serde_json::Value) -> Self where Self: Sized;
    async fn create_response_stream(&mut self, client_request: &ChatModelRequest) -> Result<ChatCompletionStream, Box<dyn Error>>;
}
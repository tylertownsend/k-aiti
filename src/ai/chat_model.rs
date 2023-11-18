use std::error::Error;

use async_trait::async_trait;

use super::{stream::CompletionStream, stream::ChatCompletionRequestMessage};

#[derive(Clone)]
pub struct ChatModelRequest {
    pub messages: Vec<ChatCompletionRequestMessage>,
}

#[async_trait]
pub trait ChatModel {
    fn new(config: serde_json::Value) -> Self where Self: Sized;
    async fn create_response_stream(&mut self, client_request: &ChatModelRequest) -> Result<CompletionStream, Box<dyn Error>>;
}
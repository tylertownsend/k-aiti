use std::error::Error;

use async_openai::{types::{ChatCompletionRequestMessage, ChatCompletionResponseMessage}, error::OpenAIError};
use async_trait::async_trait;

use super::stream::CompletionStream;

#[derive(Clone)]
pub struct ChatModelRequest {
    pub messages: Vec<ChatCompletionRequestMessage>,
}

#[async_trait]
pub trait ChatModel {
    fn new(config: serde_json::Value) -> Self where Self: Sized;

    fn create_user_message(&mut self, prompt: String) -> Result<ChatCompletionRequestMessage, Box<dyn Error>>;

    fn create_assistant_message(&mut self, prompt: String) -> Result<ChatCompletionRequestMessage, Box<dyn Error>>;

    async fn create_response_message(&mut self, client_request: &ChatModelRequest) -> Result<ChatCompletionResponseMessage, OpenAIError>;

    async fn create_response_stream(&mut self, client_request: &ChatModelRequest) -> Result<CompletionStream, Box<dyn Error>>;
}
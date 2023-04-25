use std::error::Error;

use async_openai::error::OpenAIError;
use async_openai::types::{
    ChatCompletionRequestMessageArgs, 
    CreateChatCompletionRequestArgs,
    ChatCompletionResponseStream,
    Role,
    ChatCompletionRequestMessage,
    ChatCompletionResponseMessage
};
use async_openai::Client;
use async_trait::async_trait;

use crate::ai::chat_model::{ChatModel, ChatModelRequest};

#[derive(Clone)]
pub struct GptClient {
    client: Client,
    max_tokens: u16,
    n: u8,
    temperature: f32,
    model: String,
}

impl GptClient  {
    pub fn new(config: serde_json::Value) -> GptClient {
        
        let max_tokens = match config.get("max_tokens") {
            Some(value) => value.as_u64().unwrap_or(0) as u16,
            None => 1000,
        };
        
        let n = match config.get("n") {
            Some(value) => value.as_u64().unwrap_or(0) as u8,
            None => 1,
        };
        
        let temperature = match config.get("temperature") {
            Some(value) => value.as_f64().unwrap_or(0.0) as f32,
            None => 0.8,
        };
        
        let model = match config.get("model") {
            Some(value) => value.as_str().unwrap_or("").to_string(),
            None => String::from("gpt-3.5-turbo"),
        };
        let client = Client::new();
        GptClient {
            client,
            max_tokens,
            n,
            temperature,
            model,
        }
    }
}

#[async_trait]
impl ChatModel for GptClient  {

    fn new(config: serde_json::Value) -> GptClient {
        
        let max_tokens = match config.get("max_tokens") {
            Some(value) => value.as_u64().unwrap_or(0) as u16,
            None => 1000,
        };
        
        let n = match config.get("n") {
            Some(value) => value.as_u64().unwrap_or(0) as u8,
            None => 1,
        };
        
        let temperature = match config.get("temperature") {
            Some(value) => value.as_f64().unwrap_or(0.0) as f32,
            None => 0.8,
        };
        
        let model = match config.get("model") {
            Some(value) => value.as_str().unwrap_or("").to_string(),
            None => String::from("gpt-3.5-turbo"),
        };
        let client = Client::new();
        GptClient {
            client,
            max_tokens,
            n,
            temperature,
            model,
        }
    }

    fn create_user_message(&mut self, prompt: String) -> Result<ChatCompletionRequestMessage, Box<dyn Error>> {
        Ok(ChatCompletionRequestMessageArgs::default()
            .content(prompt.to_string())
            .role(Role::User)
            .build()?)
    }

    fn create_assistant_message(&mut self, prompt: String) -> Result<ChatCompletionRequestMessage, Box<dyn Error>> {
        Ok(ChatCompletionRequestMessageArgs::default()
            .content(prompt.to_string())
            .role(Role::User)
            .build()?)
    }

    async fn create_response_message(&mut self, client_request: &ChatModelRequest) -> Result<ChatCompletionResponseMessage, OpenAIError> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(self.model.to_string())
            .n(self.n)
            .max_tokens(self.max_tokens)
            .temperature(self.temperature)
            // .n(value)
            .messages(client_request.messages.clone())
            .build()?;
        Ok(self.client.chat().create(request).await?.choices[0].clone().message)
    }

    async fn create_response_stream(
        &mut self,
        client_request: &ChatModelRequest,
    ) -> Result<ChatCompletionResponseStream, OpenAIError> {
        // Update the generate_response method in the GptClient implementation

        let request = CreateChatCompletionRequestArgs::default()
            .model(self.model.to_string())
            .n(self.n)
            .max_tokens(self.max_tokens)
            .temperature(self.temperature)
            .messages(client_request.messages.clone())
            .build()?;

       Ok(self.client.chat().create_stream(request).await?)
    }
}
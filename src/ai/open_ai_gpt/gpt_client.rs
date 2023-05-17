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
    config: GptConfig,
}


#[derive(Clone)]
pub struct GptConfig {
    max_tokens: u16,
    n: u8,
    temperature: f32,
    model: String,
}

impl GptClient  {
    pub fn new(config: serde_json::Value) -> GptClient {
        
        let max_tokens = match config.get("max_tokens") {
            Some(value) => if value.is_u64() {
                value.as_u64().unwrap_or(1000) as u16
            } else if value.is_string() {
                match value.as_str().unwrap().parse::<u16>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Failed to parse max_tokens as u16");
                        1000
                    },
                }
            } else {
                1000
            },
            None => 1000,
        };
        
        let n = match config.get("n") {
            Some(value) => if value.is_u64() {
                value.as_u64().unwrap_or(1) as u8
            } else if value.is_string() {
                match value.as_str().unwrap().parse::<u8>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Failed to parse n as u8");
                        1
                    },
                }
            } else {
                1
            },
            None => 1,
        };
        
        let temperature = match config.get("temperature") {
            Some(value) => if value.is_f64() {
                value.as_f64().unwrap_or(0.8) as f32
            } else if value.is_string() {
                match value.as_str().unwrap().parse::<f32>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Failed to parse temperature as f32");
                        0.8
                    },
                }
            } else {
                0.8
            },
            None => 0.8,
        };
        
        let model = match config.get("model") {
            Some(value) => value.as_str().unwrap_or("gpt-3.5-turbo").to_string(),
            None => String::from("gpt-3.5-turbo"),
        };
        let client = Client::new();
        GptClient {
            client,
            config: GptConfig {
                max_tokens,
                n,
                temperature,
                model,
            }
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
            config: GptConfig {
                max_tokens,
                n,
                temperature,
                model,
            }
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
            .model(self.config.model.to_string())
            .n(self.config.n)
            .max_tokens(self.config.max_tokens)
            .temperature(self.config.temperature)
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
            .model(self.config.model.to_string())
            .n(self.config.n)
            .max_tokens(self.config.max_tokens)
            .temperature(self.config.temperature)
            .messages(client_request.messages.clone())
            .build()?;

       Ok(self.client.chat().create_stream(request).await?)
    }
}
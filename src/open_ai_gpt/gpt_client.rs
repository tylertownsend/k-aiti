use std::error::Error;

use async_openai::types::{
    CreateChatCompletionRequestArgs,
    Role,
};
use async_openai::Client;
use async_trait::async_trait;
use futures::{TryStreamExt, StreamExt};

use crate::ai::chat_types::{ChatCompletionStream, ChatCompletionDelta, ChatCompletionChoice, ChatCompletionChunk, ModelUsage};
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

#[async_trait]
impl ChatModel for GptClient  {

    fn new(config: serde_json::Value) -> GptClient {
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

    async fn create_response_stream(&mut self, client_request: &ChatModelRequest) -> Result<ChatCompletionStream, Box<dyn Error>> {
        // convert messages to that expected by async_ openai
        let messages = client_request.messages.iter()
            .map(|msg| {
                async_openai::types::ChatCompletionRequestMessage {
                    content: msg.content.clone(),
                    name: msg.name.clone(),
                    role: match msg.role {
                        crate::ai::chat_types::Role::User      => Role::User,
                        crate::ai::chat_types::Role::Assistant => Role::Assistant,
                        crate::ai::chat_types::Role::System    => Role::Assistant,
                    }
                }
        }).collect::<Vec<async_openai::types::ChatCompletionRequestMessage>>();

        // Update the generate_response method in the GptClient implementation
        let request = CreateChatCompletionRequestArgs::default()
            .model(self.config.model.to_string())
            .n(self.config.n)
            .max_tokens(self.config.max_tokens)
            .temperature(self.config.temperature)
            .messages(messages)
            .build()?;

        let response = self.client.chat()
            .create_stream(request)
            .await
            .expect("Failed to create stream");

        // Transform the stream
        let stream = response
            .map_err(|e| Box::new(e) as Box<dyn Error> )
            .map(|result| {
                result.and_then(|chat_completion_response| {
                    // Process the chat_completion_response to transform it into the new type
                    // Assuming you want to convert CreateChatCompletionStreamResponse to ChatCompletionChunck
                    let choices = chat_completion_response.choices.into_iter().map(|choice_delta| {
                        ChatCompletionChoice {
                            index: choice_delta.index,
                            delta: ChatCompletionDelta {
                                content: choice_delta.delta.content,
                                role: Some(crate::ai::chat_types::Role::Assistant),
                            },
                            finish_reason: choice_delta.finish_reason,
                        }
                    }).collect();

                    Ok(ChatCompletionChunk {
                        usage: Some(ModelUsage {
                            completion_tokens: 0, // todo
                            prompt_tokens:     0, // todo
                            total_tokens:      0, // todo
                        }),
                        id: chat_completion_response.id,
                        object: chat_completion_response.object,
                        created: chat_completion_response.created,
                        model: chat_completion_response.model,
                        choices,
                    })
                })
            })
            .boxed();
        // Box the stream and pin it
        Ok(Box::pin(stream))
    }
}
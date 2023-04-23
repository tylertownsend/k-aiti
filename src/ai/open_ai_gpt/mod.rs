use std::error::Error;

use async_openai::error::OpenAIError;
use async_openai::types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, ChatCompletionResponseStream, Role, ChatCompletionRequestMessage, ChatCompletionResponseMessage};
// use serde::{Deserialize, Serialize};
use async_openai::Client;
// use futures::{StreamExt, Stream};


pub const STOP_PHRASE: &str = "##End chat##";


// #[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
// pub struct ChatCompletionRequestMessage {
//     /// The role of the author of this message.
//     pub role: Role,
//     /// The contents of the message
//     pub content: String,
//     /// The name of the user in a multi-user chat
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub name: Option<String>,
// }

#[derive(Clone)]
pub struct GptClientRequest {
    pub messages: Vec<ChatCompletionRequestMessage>,
}

#[derive(Clone)]
pub struct GptClient {
    client: Client,
    max_tokens: u16,
    n: u8,
    temperature: f32,
    model: String,
}

impl GptClient {
    pub fn new(max_tokens: u16,
               n: u8,
               temperature: f32,
               model: String) -> Self {
        let client = Client::new();
        GptClient {
            client,
            max_tokens,
            n,
            temperature,
            model,
        }
    }

    pub fn create_user_message(self, prompt: String) -> Result<ChatCompletionRequestMessage, Box<dyn Error>> {
        Ok(ChatCompletionRequestMessageArgs::default()
            .content(prompt.to_string())
            .role(Role::User)
            .build()?)
    }

    pub fn create_assistant_message(self, prompt: String) -> Result<ChatCompletionRequestMessage, Box<dyn Error>> {
        Ok(ChatCompletionRequestMessageArgs::default()
            .content(prompt.to_string())
            .role(Role::User)
            .build()?)
    }

    pub async fn generate_response(&mut self, client_request: &GptClientRequest) -> Result<ChatCompletionResponseMessage, OpenAIError> {
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

    pub async fn generate_response_stream(
        &mut self,
        client_request: &GptClientRequest,
    ) -> Result<ChatCompletionResponseStream, OpenAIError> {
        // Update the generate_response method in the GptClient implementation

        let request = CreateChatCompletionRequestArgs::default()
            .model(self.model.to_string())
            .n(self.n)
            .max_tokens(self.max_tokens)
            .temperature(self.temperature)
            // .n(value)
            .messages(client_request.messages.clone())
            .build()?;
        // };

       Ok(self.client.chat().create_stream(request).await?)
    }
}
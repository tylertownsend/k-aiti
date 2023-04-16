use std::error::Error;

use async_openai::error::OpenAIError;
use async_openai::types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, ChatCompletionResponseStream, Role, ChatCompletionRequestMessage};
// use serde::{Deserialize, Serialize};
use async_openai::Client;
// use futures::{StreamExt, Stream};


const OPENAI_API_URL_COMPLETIONS: & str = "https://api.openai.com/v1/completions";
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
    api_key: String,
    max_tokens: u16,
    n: u8,
    temperature: f32,
    model: String,
    stop: Option<String>,
}

impl GptClient {
    pub fn new(api_key: String,
               max_tokens: u16,
               n: u8,
               temperature: f32,
               model: String,
               stop: Option<String>) -> Self {
        let client = Client::new();
        GptClient {
            client,
            api_key,
            max_tokens,
            n,
            temperature,
            model,
            stop,
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

    pub async fn generate_response(
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
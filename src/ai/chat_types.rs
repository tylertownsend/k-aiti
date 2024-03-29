use std::pin::Pin;

use futures::Stream;
use serde::{Deserialize, Serialize};

// use crate::ai::chat::ChatDataStream;
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]//, Builder)]
// #[builder(name = "ChatCompletionRequestMessageArgs")]
// #[builder(pattern = "mutable")]
// #[builder(setter(into, strip_option), default)]
// #[builder(derive(Debug))]
// #[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionRequestMessage {
    /// The role of the author of this message.
    pub role: Role,
    /// The contents of the message
    pub content: String,
    /// The name of the user in a multi-user chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    #[default]
    User,
    Assistant,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ModelUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatCompletionDelta {
    pub content: Option<String>,
    pub role: Option<Role>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatCompletionChoice {
    pub index: u32,
    pub delta: ChatCompletionDelta,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ChatCompletionChunk {
    pub id: Option<String>,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<ChatCompletionChoice>,
    pub usage: Option<ModelUsage>,
}

pub type ChatCompletionStream =
    Pin<Box<dyn Stream<Item = Result<ChatCompletionChunk, Box<dyn std::error::Error>>> + Send>>;
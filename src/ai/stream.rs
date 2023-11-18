use std::{error::Error, pin::Pin};

use futures::{Stream, StreamExt};

use serde::{Deserialize, Serialize};

// use crate::ai::chat::ChatDataStream;

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

pub type CompletionStream =
    Pin<Box<dyn Stream<Item = Result<ChatCompletionChunk, Box<dyn std::error::Error>>> + Send>>;

#[async_trait::async_trait]
pub trait StreamHandler {
    type Item;
    type Error;
    type Stream: Stream<Item = Result<Self::Item, Self::Error>> + Send + 'static;

    // Define a method that will return a stream.
    async fn get_stream(&self) -> Self::Stream;
}
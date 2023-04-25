use std::{error::Error, pin::Pin};

use async_trait::async_trait;

use futures::{stream::{StreamExt, iter}, Stream};

#[async_trait]
pub trait ChatDataStream {
    type Stream: Stream<Item = Result<StreamResponse, Box<dyn Error>>> + Send + Sync;

    fn as_stream(&mut self) -> Pin<Box<Self::Stream>>;
}

pub struct StreamResponse {
    pub choices: Vec<ChatChoiceDelta>,
}

pub struct ChatChoiceDelta {
    pub delta: ChatCompletionResponseStreamMessage,
}

pub struct ChatCompletionResponseStreamMessage {
    pub content: Option<String>,
    pub role: Option<Role>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Role {
    System,
    #[default]
    User,
    Assistant,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ChatCompletionRequestMessage {
    /// The role of the author of this message.
    pub role: Role,
    /// The contents of the message
    pub content: String,
    /// The name of the user in a multi-user chat
    pub name: Option<String>,
}


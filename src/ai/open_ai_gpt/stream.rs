// use std::error::Error;
// use std::pin::Pin;
// use std::sync::Arc;
// use futures::{Stream, StreamExt};
// use async_openai::Client;
// use async_openai::types::{CreateChatCompletionRequestArgs, CreateChatCompletionStreamResponse};

// use crate::ai::chat::ChatDataStream;


// pub struct ChatDataStreamWrapper {
//     inner_stream: Pin<Box<dyn Stream<Item = Result<CreateChatCompletionStreamResponse, Box<dyn Error>>> + Send + Sync>>,
// }

// impl ChatDataStreamWrapper {
//     async fn new(client: Arc<Client>, request: CreateChatCompletionRequestArgs) -> Self {
//         let stream = Box::pin(client.chat().create_stream(request).await.unwrap());
//         ChatDataStreamWrapper { inner_stream: stream }
//     }
// }


// impl ChatDataStream for ChatDataStreamWrapper {
//     type Stream = dyn Stream<Item = Result<CreateChatCompletionStreamResponse, Box<dyn Error>>> + Send + Sync;

//     fn as_stream(&mut self) -> Pin<Box<Self::Stream>> {
//         self.inner_stream.as_mut()
//     }
// }

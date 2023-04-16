pub mod open_ai_gpt;

use std::error::Error;
use std::io::{stdout, Write};
use async_openai::types::{ChatCompletionRequestMessage, Role, ChatCompletionRequestMessageArgs};
use futures::stream::StreamExt;
use hyper::http::response;

use self::open_ai_gpt::{GptClientRequest, GptClient};

pub struct ChatClient {
    gpt_client: GptClient,
    chat_log: Vec<ChatCompletionRequestMessage>
}

impl ChatClient {
    pub fn new(gpt_client: GptClient) -> Self {
        Self { gpt_client, chat_log: Vec::new() }
    }

    pub async fn render_response(
        &mut self,
        user_input: String,
    ) -> Result<String, Box<dyn Error>> {
        let mut response_string = String::new();
        let mut lock = stdout().lock();

        let user_message = self.gpt_client.clone().create_user_message(user_input)?;
        self.chat_log.push(user_message);
        let client_request = GptClientRequest { messages: self.chat_log.clone() };

        let mut stream = self.gpt_client.generate_response(&client_request).await?;

        print!("\nAI: ");
        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    response.choices.iter().for_each(|chat_choice| {
                        if let Some(ref content) = chat_choice.delta.content {
                            write!(lock, "{}", content).unwrap();
                            response_string.push_str(content);
                        }
                    });
                }
                Err(err) => {
                    writeln!(lock, "error: {}", err).unwrap();
                }
            }
            stdout().flush()?;
        }
        println!("\n");
        let response_message = self.gpt_client
            .clone()
            .create_assistant_message(response_string.clone())?;
        self.chat_log.push(response_message);
        Ok(response_string)
        // match result {

        //     Ok(assistant_prompt) => {
        //         let response_message = self.gpt_client
        //             .clone()
        //             .create_assistant_message(assistant_prompt)?;
        //         self.chat_log.push(response_message);
        //         result
        //     }
        //     Err(err) => {
        //         writeln!(lock, "error: {}", err).unwrap();
        //         Err(err)
        //     }
        // }
    }
}
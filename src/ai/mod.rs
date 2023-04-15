pub mod open_ai_gpt;

use std::error::Error;
use std::io::{stdout, Write};
use futures::stream::StreamExt;

use self::open_ai_gpt::{ClientRequest, GptClient};

pub struct ChatClient {
    gpt_client: GptClient,
}

impl ChatClient {
    pub fn new(gpt_client: GptClient) -> Self {
        Self { gpt_client }
    }

    pub async fn render_response(
        &mut self,
        client_request: &ClientRequest,
    ) -> Result<String, Box<dyn Error>> {
        let mut response_string = String::new();
        let mut lock = stdout().lock();

        let mut stream = self.gpt_client.generate_response(client_request).await?;

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
        Ok(response_string)
    }
}
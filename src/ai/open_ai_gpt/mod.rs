use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::fmt;

const OPENAI_API_URL_COMPLETIONS: & str = "https://api.openai.com/v1/completions";

#[derive(Serialize)]
pub struct GptRequest {
    pub prompt: String,
    pub max_tokens: u32,
    pub n: u32,
    pub temperature: f32,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<String>,
}

#[derive(Clone)]
pub struct ClientRequest {
    pub prompt: String,
    pub max_tokens: u32,
    pub n: u32,
    pub temperature: f32,
    pub model: String,
    pub chat_log: Option<Vec<String>>,
    pub stop: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GptResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    // "usage":{"prompt_tokens":28,"completion_tokens":23,"total_tokens":51}}
    choices: Vec<GptChoice>,
}

#[derive(Deserialize, Debug)]
struct GptChoice {
    text: String,
    index: u32,
    // logprobs:,
    finish_reason: String
}

#[derive(Debug)]
pub enum GptClientError {
    NoResponse,
}
impl fmt::Display for GptClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GptClientError::NoResponse => write!(f, "no response from gpt"),
        }
    }
}
impl std::error::Error for GptClientError {}

pub struct GptClient {
    client: Client,
    api_key: String,
    chat_log: Vec<String>,
}
impl GptClient {
    pub fn new(api_key: String) -> Self {
        let client = Client::new();
        GptClient {
            client,
            api_key,
            chat_log: Vec::new(),
        }
    }

    pub async fn generate_response(
        &mut self,
        client_request: ClientRequest,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Update the generate_response method in the GptClient implementation

        let prompt = if let Some(chat_log) = &client_request.chat_log {
            let history = chat_log.join("##End chat##");
            format!("{}##End chat##\nYou: {}", history.trim(), client_request.prompt)
        } else {
            client_request.prompt
        };

        let request = GptRequest {
            max_tokens: client_request.max_tokens,
            prompt: prompt,
            n: client_request.n,
            temperature: client_request.temperature,
            model: client_request.model,
            stop: client_request.stop
        };

        let response: Response = self
            .client
            .post(OPENAI_API_URL_COMPLETIONS)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", format!("application/json"))
            .json(&request)
            .send()
            .await?;


        let response_bytes = response.bytes().await?;

        // Clone the bytes
        let response_bytes_clone = response_bytes.clone();
        // debug
        // if true {
            // Get the response body as Bytes
            // let response_text = String::from_utf8_lossy(&response_bytes);
            // Print the raw response
            //println!("OpenAI API raw response: {}", response_text);
        // }

        // Deserialize the response into a GptResponse struct using the cloned bytes
        let data: GptResponse = serde_json::from_slice(&response_bytes_clone)?;

        // // Print the parsed GptResponse struct
        // println!("Parsed GptResponse: {:?}", data);

        if let Some(choice) = data.choices.first() {
            let response_text = choice.text.clone();
            // self.chat_log.push(response_text.clone());
            // if let Some(stop_phrase) = &request.stop {
            //     if response_text.contains(stop_phrase) {
            //         self.chat_log.clear();
            //     }
            // }
            Ok(response_text)
        } else {
            Err(Box::new(GptClientError::NoResponse))
        }
    }
}
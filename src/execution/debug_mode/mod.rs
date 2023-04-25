// use std::error::Error;

// use crate::ai::open_ai_gpt::GptClientRequest;
// use crate::ai::{open_ai_gpt::GptClient, ChatClient};
// use crate::render::terminal_renderer::TerminalRenderer;
// use crate::terminal_capture::capture_instance;

// pub async fn run_debug_mode(gpt_client: GptClient) -> Result<(), Box<dyn Error>> {
//     let renderer = TerminalRenderer::new();
//     let mut chat_client = ChatClient::new(gpt_client.clone(), renderer);
//     let terminal = capture_instance();
//     let output = terminal.capture_output();

//     let mut messages = Vec::new();
//     let message: &str = "You are the solveit-ai. solveit-ai is an ai that accepts console messages and prints actions based on certain conditions. \
//         The supported conditions are: 1. \
//         if the message involves missing dependencies print a list of commands to run. Only the commands listed in order. Say nothing else \
//         if the message involves code errors, concisely explain the error, and list out the resolution steps. \
//         all other messages can use a relevant and concise response on what to do next \
//         these rules must be followed strictly. Do not break from these rules \
//         To confirm this processes, say only \"Confirm\". If you do not say confirm, we will prompt this again and we will not move forward until you say \"Confirm\"";

//     let prime_message = gpt_client.clone().create_user_message(message.to_string())?;
//     messages.push(prime_message);

//     let mut client_request = GptClientRequest { messages };
//     loop {
//         let response = gpt_client.clone().generate_response(&client_request).await;
//         let result = response.unwrap();
//         client_request.messages.push(gpt_client.clone().create_assistant_message(result.content.clone()).unwrap());
//         if result.content.clone().contains("Confirm") {
//             break;
//         }
//     }

//     chat_client.render_response_with_custom_history(output.unwrap(), &mut client_request.messages).await?;
//     Ok(())
// }
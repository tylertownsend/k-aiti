use aita::ai::open_ai_gpt::{GptClient};
use aita::execution;
use clap::{App, Arg, SubCommand};
use std::env;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    // let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let matches = App::new("aita")
        .version("0.1.0")
        .author("Tyler Townsend")
        .about("A smart terminal-based assistant to help engineers resolve errors and find relevant information")
        .subcommand(
            SubCommand::with_name("search")
                .about("Searches for information using the provided input string")
                .arg(
                    Arg::new("query")
                        .required(true)
                        // .about("The input string to search")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("debug")
                .about("Analyzes an error message")
                .arg(
                    Arg::new("error")
                        .short('e')
                        .long("error")
                        .value_name("ERROR_MESSAGE")
                        // .about("Provides an error message to analyze")
                        .takes_value(true),
                ),
        )
        .subcommand(SubCommand::with_name("chat").about("Enter chat mode with the GPT API"))
        .subcommand(SubCommand::with_name("config")
            .about("Configure your cli environment"))
            .aliases(&["configure", "config"])
        .get_matches();

    let gpt_client = GptClient::new(1000, 1, 0.8, String::from("gpt-3.5-turbo"));
    execution::process_command(matches, gpt_client).await
}
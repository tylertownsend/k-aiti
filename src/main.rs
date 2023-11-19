use clap::{App, Arg, SubCommand};

use k_aiti::execution;

#[tokio::main]
async fn main() {
    if execution::user_profile::validate().expect("user profile validation failed") {
        let result = execution::user_profile::setup().expect("User profile failed during creation");
        if result.abort {
            execution::user_profile::abort_message();
            return
        }
        execution::user_profile::welcome_message();
    }

    let matches = App::new("kaiti")
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
        .subcommand(SubCommand::with_name("chat").about("Chat with an AI"))
        .subcommand(SubCommand::with_name("config")
            .about("Configure your cli environment"))
            .aliases(&["configure", "config"])
        .get_matches();

    execution::process_command(matches).await
}
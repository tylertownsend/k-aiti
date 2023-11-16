# K-aiti

Welcome to the k-aiti, a tool written in Rust. This project provides a straightforward way to interact with LLMs directly from your terminal.

## Features

- Utilizes OpenAI's model as the default LLM.
- Supports a chat mode for easy and interactive conversations with the model.

## Prerequisites

Before you begin, ensure you have the following installed:

- [Rustup](https://rustup.rs/): The Rust installer and version management tool.
- **For Linux users**: Ensure you have `openssl` installed. Depending on your distribution, you can install it using the package manager:
  ```bash
  # For Debian/Ubuntu
  sudo apt-get install openssl libssl-dev
  
  # For Fedora
  sudo dnf install openssl openssl-devel
  
  # For Arch
  sudo pacman -S openssl
  ```

## Setup

Follow the steps below to set up the repo on your local machine:

1. **Clone the repository**:
   ```bash
   git clone https://github.com/tylertownsend/k-aiti.git
   ```

2. **Navigate into the project directory**:
   ```bash
   cd k-aiti
   ```

3. **Install the project**:
   ```bash
   cargo install --path .
   ```

Now you should be able to use kaiti from your shell!

## Using the Terminal Interface

1. **Start the tool**:
   ```bash
   kaiti
   ```
   Upon running the above command, you will be prompted to set up your profile. Follow the on-screen instructions to enter your OpenAI account key.

2. **Using Environment Variables**:
   If you have set up the OpenAI account key as an environment variable named `OPENAI_API_KEY`, the setup flow will detect and use it automatically.

3. **Start chatting with the AI**:
   ```bash
   kaiti chat
   ```
   This will initiate the chat mode where you can have interactive conversations with the model.

4. **Stopping the chat**:
   Simply type "stop" during the chat to exit the chat mode.

5. **Need Help?**:
   ```bash
   kaiti --help
   ```
   Run the above command for a list of available commands and usage details.
# Rusty - Chatbot powered by Rust and Ollama

This repository contains a Rust application that interacts with the Ollama API to create a chat-like interface using any model available to Ollama.

To change the model, you have to update the `main.rs` file.

## Features

- Uses the `ollama_rs` crate to interact with the Ollama API
- Implements a command-line interface for user input
- Streams responses from the language model
- Maintains conversation context for coherent multi-turn interactions
- Supports graceful exit using the "exit" command

## Dependencies

- `ollama_rs`: For interacting with the Ollama API
- `tokio`: For asynchronous runtime and I/O operations
- `tokio-stream`: For working with asynchronous streams

## How it works

1. The application creates an instance of the Ollama client.
2. It enters a loop that:
   - Prompts the user for input
   - Sends the input to the Mistral-Nemo model via the Ollama API
   - Streams and displays the model's response
   - Maintains conversation context for follow-up interactions
3. The loop continues until the user types "exit" (case-insensitive).

## Usage

To run the application, make sure you have Rust and Cargo installed, then execute:

```
cargo run
```

Enter your prompts at the `>` prompt. Type "exit" to quit the application.

## Note

This application assumes that an Ollama server is running locally with the Mistral-Nemo model available. Make sure to have Ollama set up and the required model installed before running this application.

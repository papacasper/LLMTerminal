use crossterm::event::{read, Event, KeyCode, KeyEvent};
use std::sync::Arc;
mod config;
mod llm;

use config::Config;
use llm::{ClaudeClient, LLMClient, OpenAIClient};

#[derive(Clone)]
enum LLMProvider {
    Claude(ClaudeClient),
    OpenAI(OpenAIClient),
}

impl LLMProvider {
    async fn send_message(&self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            LLMProvider::Claude(client) => client.send_message(message).await,
            LLMProvider::OpenAI(client) => client.send_message(message).await,
        }
    }
    
    fn provider(&self) -> String {
        match self {
            LLMProvider::Claude(client) => client.provider(),
            LLMProvider::OpenAI(client) => client.provider(),
        }
    }
}

#[tokio::main]
async fn main() {
    let config = Config::load_from_file("config.json");
    let claude_client = ClaudeClient::new(config.claude_api_key);
    let openai_client = OpenAIClient::new(config.openai_api_key);
    let clients = vec![LLMProvider::Claude(claude_client), LLMProvider::OpenAI(openai_client)];
    let app = AppState { clients };

    app.run().await;
}

struct AppState {
    clients: Vec<LLMProvider>,
}

impl AppState {
    async fn run(&self) {
        println!("Welcome to LLMTerminal! Type 'exit' to quit.");
        loop {
            if let Event::Key(KeyEvent { code, .. }) = read().unwrap() {
                match code {
                    KeyCode::Char('q') => {
                        println!("Exiting...");
                        break;
                    }
                    KeyCode::Char(c) => {
                        println!("You pressed: {}", c);
                    }
                    _ => {}
                }
            }
        }
    }
}

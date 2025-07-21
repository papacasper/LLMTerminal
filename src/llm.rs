use reqwest::Client;
use std::sync::Arc;
use async_trait::async_trait;

#[async_trait]
pub trait LLMClient {
    async fn send_message(&self, message: &str) -> Result<String, Box<dyn std::error::Error>>;
    fn provider(&self) -> String;
}

#[derive(Clone)]
pub struct ClaudeClient {
    api_key: String,
    client: Arc<Client>,
}

#[async_trait]
impl LLMClient for ClaudeClient {
    async fn send_message(&self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Implement sending message to Claude API
        Ok(String::from("response from Claude"))
    }

    fn provider(&self) -> String {
        String::from("Claude")
    }
}

impl ClaudeClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Arc::new(Client::new()),
        }
    }
}

#[derive(Clone)]
pub struct OpenAIClient {
    api_key: String,
    client: Arc<Client>,
}

#[async_trait]
impl LLMClient for OpenAIClient {
    async fn send_message(&self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Implement sending message to OpenAI API
        Ok(String::from("response from OpenAI"))
    }

    fn provider(&self) -> String {
        String::from("OpenAI")
    }
}

impl OpenAIClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Arc::new(Client::new()),
        }
    }
}


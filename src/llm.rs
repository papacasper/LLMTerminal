use reqwest::Client;
use std::sync::Arc;
use async_trait::async_trait;
use serde_json::{json, Value};

#[async_trait]
#[allow(dead_code)]
pub trait LLMClient {
    async fn send_message(&self, message: &str) -> Result<String, Box<dyn std::error::Error>>;
    fn provider(&self) -> String;
}

#[derive(Clone)]
pub struct ClaudeClient {
    #[allow(dead_code)]
    api_key: String,
    #[allow(dead_code)]
    client: Arc<Client>,
}

#[async_trait]
impl LLMClient for ClaudeClient {
    async fn send_message(&self, _message: &str) -> Result<String, Box<dyn std::error::Error>> {
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

    pub async fn query(&self, input: &str, model: &str, max_tokens: u32, temperature: f32) -> Result<String, String> {
        if self.api_key.is_empty() {
            return Err("Claude API key not provided".to_string());
        }

        let payload = json!({
            "model": model,
            "max_tokens": max_tokens,
            "temperature": temperature,
            "messages": [{
                "role": "user",
                "content": input
            }]
        });

        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("Content-Type", "application/json")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("API request failed with status {}: {}", status, error_text));
        }

        let json: Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        // Extract the response content from Claude's API response format
        if let Some(content) = json["content"].as_array() {
            if let Some(first_content) = content.first() {
                if let Some(text) = first_content["text"].as_str() {
                    return Ok(text.to_string());
                }
            }
        }

        Err("Unable to extract response from Claude API".to_string())
    }
}

#[derive(Clone)]
pub struct OpenAIClient {
    #[allow(dead_code)]
    api_key: String,
    #[allow(dead_code)]
    client: Arc<Client>,
}

#[async_trait]
impl LLMClient for OpenAIClient {
    async fn send_message(&self, _message: &str) -> Result<String, Box<dyn std::error::Error>> {
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

    pub async fn query(&self, input: &str, model: &str, max_tokens: u32, temperature: f32) -> Result<String, String> {
        if self.api_key.is_empty() {
            return Err("OpenAI API key not provided".to_string());
        }

        let payload = json!({
            "model": model,
            "max_tokens": max_tokens,
            "temperature": temperature,
            "messages": [{
                "role": "user",
                "content": input
            }]
        });

        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("API request failed with status {}: {}", status, error_text));
        }

        let json: Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        // Extract the response content from OpenAI's API response format
        if let Some(choices) = json["choices"].as_array() {
            if let Some(first_choice) = choices.first() {
                if let Some(message) = first_choice["message"].as_object() {
                    if let Some(content) = message["content"].as_str() {
                        return Ok(content.to_string());
                    }
                }
            }
        }

        Err("Unable to extract response from OpenAI API".to_string())
    }
}


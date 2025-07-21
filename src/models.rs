use serde::{Deserialize, Serialize};
use crate::llm::{ClaudeClient, OpenAIClient};

#[derive(Debug, Clone, PartialEq)]
pub enum AppTab {
    Terminal,
    Settings,
    Chat,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub current_tab: AppTab,
    pub settings: AppSettings,
    #[allow(dead_code)]
    pub input_buffer: String,
    #[allow(dead_code)]
    pub messages: Vec<ChatMessage>,
    #[allow(dead_code)]
    pub should_quit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub claude_api_key: String,
    pub openai_api_key: String,
    pub default_provider: LLMProviderType,
    pub max_tokens: u32,
    pub temperature: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LLMProviderType {
    Claude,
    OpenAI,
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    #[allow(dead_code)]
    pub role: MessageRole,
    #[allow(dead_code)]
    pub content: String,
    #[allow(dead_code)]
    pub timestamp: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum LLMProvider {
    Claude(ClaudeClient),
    OpenAI(OpenAIClient),
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            claude_api_key: String::new(),
            openai_api_key: String::new(),
            default_provider: LLMProviderType::Claude,
            max_tokens: 1000,
            temperature: 0.7,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_tab: AppTab::Terminal,
            settings: AppSettings::default(),
            input_buffer: String::new(),
            messages: Vec::new(),
            should_quit: false,
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }
}

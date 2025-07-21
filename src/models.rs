use serde::{Deserialize, Serialize};
use crate::llm::{ClaudeClient, OpenAIClient};

#[derive(Debug, Clone, PartialEq)]
pub enum AppTab {
    AgenticTerminal,
    Settings,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub current_tab: AppTab,
    pub settings: AppSettings,
    pub input_buffer: String,
    #[allow(dead_code)]
    pub messages: Vec<ChatMessage>,
    pub terminal_output: Vec<String>,
    pub command_history: Vec<String>,
    pub history_index: Option<usize>,
    pub show_history_popup: bool,
    pub is_thinking: bool,
    pub available_claude_models: Vec<String>,
    pub available_openai_models: Vec<String>,
    #[allow(dead_code)]
    pub should_quit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub claude_api_key: String,
    pub openai_api_key: String,
    pub default_provider: LLMProviderType,
    pub claude_model: String,
    pub openai_model: String,
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
            claude_model: "claude-3-5-sonnet-20241022".to_string(),
            openai_model: "gpt-4o".to_string(),
            max_tokens: 1000,
            temperature: 0.7,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_tab: AppTab::AgenticTerminal,
            settings: AppSettings::default(),
            input_buffer: String::new(),
            messages: Vec::new(),
            terminal_output: Vec::new(),
            command_history: Vec::new(),
            history_index: None,
            show_history_popup: false,
            is_thinking: false,
            available_claude_models: vec![
                "claude-3-5-sonnet-20241022".to_string(),
                "claude-3-5-haiku-20241022".to_string(),
                "claude-3-opus-20240229".to_string(),
            ],
            available_openai_models: vec![
                "gpt-4o".to_string(),
                "gpt-4o-mini".to_string(),
                "gpt-4-turbo".to_string(),
                "gpt-3.5-turbo".to_string(),
            ],
            should_quit: false,
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_available_models(&self) -> (Vec<String>, Vec<String>) {
        let claude_models = self.load_models_from_file("./models/claude_models.txt");
        let openai_models = self.load_models_from_file("./models/openai_models.txt");
        (claude_models, openai_models)
    }

    fn load_models_from_file(&self, path: &str) -> Vec<String> {
        std::fs::read_to_string(path)
            .unwrap_or_default()
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.trim().to_string())
            .collect()
    }
}

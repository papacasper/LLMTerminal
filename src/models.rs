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
    pub input_buffer: String,
    pub messages: Vec<ChatMessage>,
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
    pub role: MessageRole,
    pub content: String,
    pub timestamp: String,
}

#[derive(Debug, Clone)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

#[derive(Clone)]
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
    
    pub fn next_tab(&mut self) {
        self.current_tab = match self.current_tab {
            AppTab::Terminal => AppTab::Chat,
            AppTab::Chat => AppTab::Settings,
            AppTab::Settings => AppTab::Terminal,
        };
    }
    
    pub fn previous_tab(&mut self) {
        self.current_tab = match self.current_tab {
            AppTab::Terminal => AppTab::Settings,
            AppTab::Settings => AppTab::Chat,
            AppTab::Chat => AppTab::Terminal,
        };
    }
    
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}

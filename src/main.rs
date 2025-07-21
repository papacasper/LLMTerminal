mod config;
mod llm;
mod models;

use eframe::egui;
use config::Config;
use llm::{ClaudeClient, OpenAIClient};
use models::{AppState, AppTab, LLMProvider};
use pollster;

fn main() {
    pollster::block_on(start())
}

async fn start() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "LLMTerminal",
        options,
        Box::new(|_cc| Box::new(LlmTerminalApp::default())),
    ).unwrap();
}

struct LlmTerminalApp {
    app_state: AppState,
}

impl Default for LlmTerminalApp {
    fn default() -> Self {
        let config = Config::load_from_file("config.json");
        let claude_client = ClaudeClient::new(config.claude_api_key.clone());
        let openai_client = OpenAIClient::new(config.openai_api_key.clone());
        let _clients = vec![LLMProvider::Claude(claude_client), LLMProvider::OpenAI(openai_client)];

        let mut app_state = AppState::new();
        app_state.settings = config;

        Self { app_state }
    }
}

impl eframe::App for LlmTerminalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to LLMTerminal!");
            ui.horizontal(|ui| {
                if ui.button("Terminal").clicked() {
                    self.app_state.current_tab = AppTab::Terminal;
                }
                if ui.button("Chat").clicked() {
                    self.app_state.current_tab = AppTab::Chat;
                }
                if ui.button("Settings").clicked() {
                    self.app_state.current_tab = AppTab::Settings;
                }
            });
            ui.separator();

            match self.app_state.current_tab {
                AppTab::Terminal => {
                    ui.heading("Terminal Tab");
                    ui.label("Terminal functionality will be implemented here.");
                },
                AppTab::Settings => {
                    ui.heading("Settings Tab");
                    ui.label("Configure your LLM settings:");
                    
                    ui.horizontal(|ui| {
                        ui.label("Claude API Key:");
                        ui.text_edit_singleline(&mut self.app_state.settings.claude_api_key);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("OpenAI API Key:");
                        ui.text_edit_singleline(&mut self.app_state.settings.openai_api_key);
                    });
                    
                    if ui.button("Save Settings").clicked() {
                        if let Err(e) = self.app_state.settings.save_to_file("config.json") {
                            println!("Failed to save settings: {}", e);
                        } else {
                            println!("Settings saved successfully!");
                        }
                    }
                },
                AppTab::Chat => {
                    ui.heading("Chat Tab");
                    ui.label("Chat with your LLM will be implemented here.");
                },
            }
        });
    }
}


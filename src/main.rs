use crossterm::event::{read, Event, KeyCode, KeyEvent};
mod config;
mod llm;
mod models;

use config::Config;
use llm::{ClaudeClient, OpenAIClient};
use models::{AppState, AppTab, LLMProvider};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load_from_file("config.json");
    let claude_client = ClaudeClient::new(config.claude_api_key.clone());
    let openai_client = OpenAIClient::new(config.openai_api_key.clone());
    let clients = vec![LLMProvider::Claude(claude_client), LLMProvider::OpenAI(openai_client)];

    let mut app_state = AppState::new();
    app_state.settings = config;

    run_app(&mut app_state).await;
    Ok(())
}

async fn run_app(app_state: &mut AppState) {
    println!("Welcome to LLMTerminal! Navigate with tabs. Type 'exit' to quit.");
    loop {
        match app_state.current_tab {
            AppTab::Terminal => println!("You're on the Terminal tab."),
            AppTab::Settings => println!("You're on the Settings tab."),
            AppTab::Chat => println!("You're on the Chat tab."),
        }

        if let Event::Key(KeyEvent { code, .. }) = read().unwrap() {
            match code {
                KeyCode::Char('q') => {
                    println!("Exiting...");
                    app_state.quit();
                }
                KeyCode::Char('t') => app_state.next_tab(),
                KeyCode::Char('b') => app_state.previous_tab(),
                _ => {}
            }
        }

        if app_state.should_quit {
            break;
        }
    }
}

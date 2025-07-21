impl LlmTerminalApp {
    fn create_enhanced_input(&self, input: &str) -> String {
        let current_dir = std::env::current_dir()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|_| "Unknown".to_string());
        
        let folder_contents = self.get_folder_contents(&current_dir);
        
        format!(
            "{}

--- System Context ---
Current directory: {}
Operating System: {}
Architecture: {}

--- Directory Contents ---
{}

--- Recent Commands ---
{}"
            , input, 
            current_dir, 
            std::env::consts::OS,
            std::env::consts::ARCH,
            folder_contents,
            self.get_recent_commands()
        )
    }
    
    fn get_folder_contents(&self, current_dir: &str) -> String {
        match std::fs::read_dir(current_dir) {
            Ok(entries) => {
                let mut contents = Vec::new();
                for entry in entries.flatten() {
                    let path = entry.path();
                    let name = path.file_name().unwrap_or_default().to_string_lossy();
                    if path.is_dir() {
                        contents.push(format!("📁 {}/", name));
                    } else {
                        contents.push(format!("📄 {}", name));
                    }
                }
                contents.sort();
                if contents.len() > 20 {
                    contents.truncate(20);
                    contents.push("... (showing first 20 items)".to_string());
                }
                contents.join("\n")
            }
            Err(e) => format!("Error reading directory: {}", e)
        }
    }
    
    fn get_recent_commands(&self) -> String {
        let recent_count = 5.min(self.app_state.command_history.len());
        if recent_count == 0 {
            "(No recent commands)".to_string()
        } else {
            self.app_state.command_history
                .iter()
                .rev()
                .take(recent_count)
                .enumerate()
                .map(|(i, cmd)| format!("{}. {}", recent_count - i, cmd))
                .collect::<Vec<_>>()
                .join("\n")
        }
    }
}

mod config;
mod llm;
mod models;

use eframe::egui;
use config::Config;
use llm::{ClaudeClient, OpenAIClient};
use models::{AppState, AppTab, LLMProvider, LLMProviderType};
use tokio;
use std::sync::Arc;

fn main() -> Result<(), eframe::Error> {
    let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());
    let options = eframe::NativeOptions::default();
    
    eframe::run_native(
        "LLMTerminal",
        options,
        Box::new(move |_cc| Box::new(LlmTerminalApp::new(rt.clone()))),
    )
}

struct LlmTerminalApp {
    app_state: AppState,
    runtime: Arc<tokio::runtime::Runtime>,
}

impl LlmTerminalApp {
    fn new(runtime: Arc<tokio::runtime::Runtime>) -> Self {
        let config = Config::load_from_file("config.json");
        let claude_client = ClaudeClient::new(config.claude_api_key.clone());
        let openai_client = OpenAIClient::new(config.openai_api_key.clone());
        let _clients = vec![LLMProvider::Claude(claude_client), LLMProvider::OpenAI(openai_client)];

        let mut app_state = AppState::new();
        app_state.settings = config;

        Self { 
            app_state,
            runtime,
        }
    }
}

impl eframe::App for LlmTerminalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to LLMTerminal!");
            ui.horizontal(|ui| {
                if ui.button("Agentic Terminal").clicked() {
                    self.app_state.current_tab = AppTab::AgenticTerminal;
                }
                if ui.button("Settings").clicked() {
                    self.app_state.current_tab = AppTab::Settings;
                }
            });
            ui.separator();

            match self.app_state.current_tab {
                AppTab::AgenticTerminal => {
                    self.render_agentic_terminal(ui);
                },
                AppTab::Settings => {
                    self.render_settings(ui);
                },
            }
        });
    }
}

impl LlmTerminalApp {
    fn fetch_latest_models(&mut self) {
        self.app_state.terminal_output.push("Fetching latest models from APIs...".to_string());
        
        // Create models directory if it doesn't exist
        if let Err(e) = std::fs::create_dir_all("./models") {
            self.app_state.terminal_output.push(format!("Failed to create models directory: {}", e));
            return;
        }
        
        // Fetch OpenAI models
        if !self.app_state.settings.openai_api_key.is_empty() {
            self.app_state.terminal_output.push("Fetching OpenAI models...".to_string());
            match self.fetch_openai_models() {
                Ok(models) => {
                    if let Err(e) = self.save_models_to_file(&models, "./models/openai_models.txt") {
                        self.app_state.terminal_output.push(format!("Failed to save OpenAI models: {}", e));
                    } else {
                        self.app_state.terminal_output.push(format!("Fetched {} OpenAI models", models.len()));
                        self.app_state.available_openai_models = models;
                    }
                }
                Err(e) => {
                    self.app_state.terminal_output.push(format!("Failed to fetch OpenAI models: {}", e));
                }
            }
        } else {
            self.app_state.terminal_output.push("OpenAI API key not provided. Skipping OpenAI models.".to_string());
        }
        
        // Fetch Claude models
        if !self.app_state.settings.claude_api_key.is_empty() {
            self.app_state.terminal_output.push("Fetching Claude models...".to_string());
            match self.fetch_claude_models() {
                Ok(models) => {
                    if let Err(e) = self.save_models_to_file(&models, "./models/claude_models.txt") {
                        self.app_state.terminal_output.push(format!("Failed to save Claude models: {}", e));
                    } else {
                        self.app_state.terminal_output.push(format!("Fetched {} Claude models", models.len()));
                        self.app_state.available_claude_models = models;
                    }
                }
                Err(e) => {
                    self.app_state.terminal_output.push(format!("Failed to fetch Claude models: {}", e));
                }
            }
        } else {
            self.app_state.terminal_output.push("Claude API key not provided. Skipping Claude models.".to_string());
        }
        
        self.app_state.terminal_output.push("Model fetch complete!".to_string());
    }

    fn load_available_models_to_ui(&mut self) {
        let (claude_models, openai_models) = self.app_state.load_available_models();
        
        if !claude_models.is_empty() {
            self.app_state.available_claude_models = claude_models.clone();
            self.app_state.terminal_output.push(format!("Loaded {} Claude models", claude_models.len()));
        }
        
        if !openai_models.is_empty() {
            self.app_state.available_openai_models = openai_models.clone();
            self.app_state.terminal_output.push(format!("Loaded {} OpenAI models", openai_models.len()));
        }
    }

    fn fetch_openai_models(&self) -> Result<Vec<String>, String> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .get("https://api.openai.com/v1/models")
            .header("Authorization", format!("Bearer {}", self.app_state.settings.openai_api_key))
            .header("Content-Type", "application/json")
            .send()
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API request failed with status: {}", response.status()));
        }

        let json: serde_json::Value = response
            .json()
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        let models = json["data"]
            .as_array()
            .ok_or("Invalid response format")?;

        let mut model_names: Vec<String> = models
            .iter()
            .filter_map(|model| model["id"].as_str())
            .map(|s| s.to_string())
            .collect();

        model_names.sort();
        Ok(model_names)
    }

    fn fetch_claude_models(&self) -> Result<Vec<String>, String> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .get("https://api.anthropic.com/v1/models")
            .header("x-api-key", &self.app_state.settings.claude_api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .send()
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API request failed with status: {}", response.status()));
        }

        let json: serde_json::Value = response
            .json()
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        let models = json["data"]
            .as_array()
            .ok_or("Invalid response format")?;

        let mut model_names: Vec<String> = models
            .iter()
            .filter_map(|model| model["id"].as_str())
            .map(|s| s.to_string())
            .collect();

        model_names.sort();
        Ok(model_names)
    }

    fn save_models_to_file(&self, models: &[String], path: &str) -> Result<(), std::io::Error> {
        let content = models.join("\n");
        std::fs::write(path, content)
    }
    fn render_agentic_terminal(&mut self, ui: &mut egui::Ui) {
        ui.heading("Agentic Terminal");
        
        // Use a vertical layout with the input area fixed at the bottom
        ui.vertical(|ui| {
            // Calculate space needed for the fixed bottom area
            let bottom_area_height = 80.0; // Space for input, separator, and thinking indicator
            let available_height = ui.available_height() - bottom_area_height;
            
            // Terminal output area - takes up remaining space
            egui::ScrollArea::vertical()
                .max_height(available_height)
                .stick_to_bottom(true)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    if self.app_state.terminal_output.is_empty() {
                        ui.label("Terminal ready. Commands are auto-detected, or ask questions directly.");
                    } else {
                        for output in &self.app_state.terminal_output {
                            self.render_ansi_text(ui, output);
                        }
                    }
                });
            
            // Fixed input area at the bottom
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                // Show thinking indicator
                if self.app_state.is_thinking {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label("Thinking...");
                    });
                }
                
                // Input field - always at the very bottom
                ui.horizontal(|ui| {
                    ui.label("Input:");
                    let response = ui.text_edit_singleline(&mut self.app_state.input_buffer);
                    
                    // Handle keyboard input
                    if response.has_focus() {
                        ui.input(|i| {
                            // Handle up arrow key to show history
                            if i.key_pressed(egui::Key::ArrowUp) {
                                self.navigate_history_up();
                            }
                            // Handle down arrow key to navigate history
                            if i.key_pressed(egui::Key::ArrowDown) {
                                self.navigate_history_down();
                            }
                            // Handle escape key to hide history popup
                            if i.key_pressed(egui::Key::Escape) {
                                self.app_state.show_history_popup = false;
                                self.app_state.history_index = None;
                            }
                        });
                    }
                    
                    // Handle Enter key press
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.handle_input();
                        self.app_state.show_history_popup = false;
                        self.app_state.history_index = None;
                    }
                    
                    if ui.button("Send").clicked() {
                        self.handle_input();
                        self.app_state.show_history_popup = false;
                        self.app_state.history_index = None;
                    }
                });
                
                // Show floating history popup if needed
                if self.app_state.show_history_popup && !self.app_state.command_history.is_empty() {
                    self.render_history_popup(ui);
                }
                
                ui.separator();
            });
        });
    }

    fn handle_input(&mut self) {
        let input = self.app_state.input_buffer.trim().to_string();
        if !input.is_empty() {
            self.app_state.command_history.push(input.clone());
            self.process_input(input);
            self.app_state.input_buffer.clear();
        }
    }

    fn process_input(&mut self, input: String) {
        // Add user input to output
        self.app_state.terminal_output.push(format!("User: {}", input));
        
        // Check if it's a valid terminal command
        if self.is_valid_command(&input) {
            // Handle terminal commands
            let command = input.trim();
            self.app_state.terminal_output.push(format!("Executing command: {}", command));
            
            match self.execute_command(command)
            {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    
                    if !stdout.is_empty() {
                        self.app_state.terminal_output.push(stdout.trim().to_string());
                    }
                    if !stderr.is_empty() {
                        self.app_state.terminal_output.push(format!("Error: {}", stderr.trim()));
                    }
                    if stdout.is_empty() && stderr.is_empty() {
                        self.app_state.terminal_output.push("Command executed successfully (no output)".to_string());
                    }
                }
                Err(e) => {
                    self.app_state.terminal_output.push(format!("Failed to execute command: {}", e));
                }
            }
        } else {
            // Handle LLM queries
            self.app_state.is_thinking = true;
            self.app_state.terminal_output.push("Querying LLM...".to_string());
            
            // Add system context to the input
            let enhanced_input = self.create_enhanced_input(&input);
            
            let llm_response = match self.app_state.settings.default_provider {
                LLMProviderType::Claude => {
                    let client = ClaudeClient::new(self.app_state.settings.claude_api_key.clone());
                    let runtime = self.runtime.clone();
                    let enhanced_input = enhanced_input.clone();
                    let model = self.app_state.settings.claude_model.clone();
                    let max_tokens = self.app_state.settings.max_tokens;
                    let temperature = self.app_state.settings.temperature;
                    
                    runtime.block_on(async move {
                        client.query(&enhanced_input, &model, max_tokens, temperature).await
                    })
                }
                LLMProviderType::OpenAI => {
                    let client = OpenAIClient::new(self.app_state.settings.openai_api_key.clone());
                    let runtime = self.runtime.clone();
                    let enhanced_input = enhanced_input.clone();
                    let model = self.app_state.settings.openai_model.clone();
                    let max_tokens = self.app_state.settings.max_tokens;
                    let temperature = self.app_state.settings.temperature;
                    
                    runtime.block_on(async move {
                        client.query(&enhanced_input, &model, max_tokens, temperature).await
                    })
                }
            };

            self.app_state.is_thinking = false;
            
            match llm_response {
                Ok(response) => {
                    self.app_state.terminal_output.push(format!("Assistant: {}", response));
                }
                Err(e) => {
                    self.app_state.terminal_output.push(format!("Failed to query LLM: {}", e));
                }
            }
        }
    }

    fn navigate_history_up(&mut self) {
        if self.app_state.command_history.is_empty() {
            return;
        }
        
        self.app_state.show_history_popup = true;
        
        let history_len = self.app_state.command_history.len();
        self.app_state.history_index = match self.app_state.history_index {
            None => {
                // First time pressing up arrow, go to the last command
                let index = history_len - 1;
                self.app_state.input_buffer = self.app_state.command_history[index].clone();
                Some(index)
            }
            Some(current_index) => {
                if current_index > 0 {
                    // Go further back in history
                    let new_index = current_index - 1;
                    self.app_state.input_buffer = self.app_state.command_history[new_index].clone();
                    Some(new_index)
                } else {
                    // Already at the oldest command
                    Some(current_index)
                }
            }
        };
    }
    
    fn navigate_history_down(&mut self) {
        if self.app_state.command_history.is_empty() || self.app_state.history_index.is_none() {
            return;
        }
        
        let history_len = self.app_state.command_history.len();
        self.app_state.history_index = match self.app_state.history_index {
            Some(current_index) => {
                if current_index < history_len - 1 {
                    // Go forward in history
                    let new_index = current_index + 1;
                    self.app_state.input_buffer = self.app_state.command_history[new_index].clone();
                    Some(new_index)
                } else {
                    // At the newest command, clear input and hide popup
                    self.app_state.input_buffer.clear();
                    self.app_state.show_history_popup = false;
                    None
                }
            }
            None => None,
        };
    }
    
    fn render_history_popup(&mut self, ui: &mut egui::Ui) {
        // Create a floating window for command history
        let window_response = egui::Window::new("Command History")
            .id(egui::Id::new("command_history_popup"))
            .collapsible(false)
            .resizable(false)
            .title_bar(false)
            .anchor(egui::Align2::LEFT_BOTTOM, egui::Vec2::new(0.0, -50.0))
            .show(ui.ctx(), |ui| {
                ui.set_min_width(400.0);
                ui.set_max_width(600.0);
                
                ui.label("📜 Command History (↑/↓ to navigate, Esc to close)");
                ui.separator();
                
                // Show recent history items (last 10)
                let history_len = self.app_state.command_history.len();
                let start_index = if history_len > 10 { history_len - 10 } else { 0 };
                
                for (display_index, (actual_index, cmd)) in self.app_state.command_history
                    .iter()
                    .enumerate()
                    .skip(start_index)
                    .enumerate()
                {
                    let is_selected = self.app_state.history_index == Some(actual_index);
                    
                    let response = ui.selectable_label(
                        is_selected,
                        format!("{}: {}", display_index + start_index + 1, cmd)
                    );
                    
                    if response.clicked() {
                        self.app_state.input_buffer = cmd.clone();
                        self.app_state.history_index = Some(actual_index);
                        self.app_state.show_history_popup = false;
                    }
                }
            });
        
        // Close popup if user clicks outside
        if let Some(response) = window_response {
            if response.response.clicked_elsewhere() {
                self.app_state.show_history_popup = false;
                self.app_state.history_index = None;
            }
        }
    }

    fn is_valid_command(&self, command: &str) -> bool {
        let trimmed = command.trim();
        
        // Skip obvious natural language patterns
        if self.looks_like_natural_language(trimmed) {
            return false;
        }
        
        if cfg!(target_os = "windows") {
            self.is_valid_command_windows(command)
        } else {
            self.is_valid_command_unix(command)
        }
    }
    
    fn looks_like_natural_language(&self, text: &str) -> bool {
        let text_lower = text.to_lowercase();
        
        // Common question patterns - check if text contains these anywhere, not just starts with
        let question_patterns = [
            "can you", "could you", "would you", "will you",
            "how do", "how can", "how to", "what is", "what are",
            "where is", "where are", "when is", "when are",
            "why is", "why are", "why do", "why does",
            "please", "help me", "i need", "i want",
            "show me", "tell me", "explain", "describe",
            "hi,", "hello,", "hey,"
        ];
        
        // Check for question patterns anywhere in the text
        for pattern in &question_patterns {
            if text_lower.contains(pattern) {
                return true;
            }
        }
        
        // Check if it ends with a question mark
        if text.ends_with('?') {
            return true;
        }
        
        // Check for greeting patterns
        let greeting_patterns = ["hi", "hello", "hey", "greetings"];
        let first_word = text.split_whitespace().next().unwrap_or("").to_lowercase();
        let cleaned_first_word = first_word.trim_end_matches(',');
        if greeting_patterns.contains(&cleaned_first_word) {
            return true;
        }
        
        // Check if it contains multiple words that suggest natural language
        let words: Vec<&str> = text.split_whitespace().collect();
        if words.len() >= 3 {
            // Look for common English words that suggest natural language
            let natural_indicators = [
                "a", "an", "the", "that", "this", "with", "for", "and", "or",
                "make", "create", "write", "generate", "build", "show", "display",
                "file", "txt", "simple", "says", "text", "document"
            ];
            
            let natural_word_count = words.iter()
                .filter(|word| {
                    let cleaned_word = word.to_lowercase();
                    let trimmed_word = cleaned_word.trim_end_matches(&[',', '.', '!', '?']);
                    natural_indicators.contains(&trimmed_word)
                })
                .count();
            
            // If more than 1/4 of words are natural language indicators, treat as natural language
            if natural_word_count as f32 / words.len() as f32 > 0.25 {
                return true;
            }
        }
        
        // Check for conversational patterns
        if text_lower.contains(" a ") || text_lower.contains(" an ") || text_lower.contains(" the ") {
            if words.len() >= 4 {
                return true;
            }
        }
        
        false
    }

    fn is_valid_command_windows(&self, command: &str) -> bool {
        use std::process::{Command, Stdio};
        
        let cmd_name = command.split_whitespace().next().unwrap_or("");
        if cmd_name.is_empty() {
            return false;
        }
        
        // List of PowerShell executables to try (different versions and installations)
        let powershell_variants = ["pwsh", "powershell", "powershell.exe", "pwsh.exe"];
        
        // Try PowerShell variants with Get-Command
        for ps_exe in &powershell_variants {
            // First check if PowerShell executable exists
            if Command::new(ps_exe)
                .arg("-NoProfile")
                .arg("-NonInteractive")
                .arg("-Command")
                .arg("$true")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .map_or(false, |s| s.success()) {
                
                // Then check if command exists
                if Command::new(ps_exe)
                    .arg("-NoProfile")
                    .arg("-NonInteractive")
                    .arg("-Command")
                    .arg(format!("Get-Command '{}' -ErrorAction SilentlyContinue | Out-Null; exit $?", cmd_name))
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()
                    .map_or(false, |status| status.success()) {
                    return true;
                }
            }
        }
        
        // Try CMD variants
        let cmd_variants = ["cmd", "cmd.exe", "command.com"];
        
        for cmd_exe in &cmd_variants {
            // Try 'where' command (modern CMD)
            if Command::new(cmd_exe)
                .arg("/C")
                .arg(format!("where {} >nul 2>nul", cmd_name))
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .map_or(false, |status| status.success()) {
                return true;
            }
            
            // Fallback for older CMD versions without 'where'
            if Command::new(cmd_exe)
                .arg("/C")
                .arg(format!("for %i in ({}.exe {}.com {}.bat {}) do @if exist \"%~$PATH:i\" exit /b 0 & exit /b 1", cmd_name, cmd_name, cmd_name, cmd_name))
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .map_or(false, |status| status.success()) {
                return true;
            }
        }
        
        false
    }

    fn is_valid_command_unix(&self, command: &str) -> bool {
        use std::process::{Command, Stdio};
        
        let cmd_name = command.split_whitespace().next().unwrap_or("");
        if cmd_name.is_empty() {
            return false;
        }
        
        // Try different shell variants and methods for maximum compatibility
        let shell_variants = [
            // Modern shells with command -v support
            ("bash", "-c", format!("command -v {} >/dev/null 2>&1", cmd_name)),
            ("zsh", "-c", format!("command -v {} >/dev/null 2>&1", cmd_name)),
            ("fish", "-c", format!("command -v {} >/dev/null 2>&1", cmd_name)),
            ("dash", "-c", format!("command -v {} >/dev/null 2>&1", cmd_name)),
            ("ksh", "-c", format!("command -v {} >/dev/null 2>&1", cmd_name)),
            ("tcsh", "-c", format!("which {} >/dev/null 2>&1", cmd_name)),
            ("csh", "-c", format!("which {} >/dev/null", cmd_name)),
            
            // POSIX sh fallback (should work on all Unix-like systems)
            ("sh", "-c", format!("command -v {} >/dev/null 2>&1", cmd_name)),
            
            // Alternative methods for older systems
            ("sh", "-c", format!("type {} >/dev/null 2>&1", cmd_name)),
            ("sh", "-c", format!("which {} >/dev/null 2>&1", cmd_name)),
            
            // Direct executable check in common paths
            ("sh", "-c", format!("test -x /usr/bin/{} || test -x /bin/{} || test -x /usr/local/bin/{}", cmd_name, cmd_name, cmd_name)),
        ];
        
        // Try each shell variant
        for (shell, flag, check_cmd) in &shell_variants {
            if Command::new(shell)
                .arg(flag)
                .arg(check_cmd)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .map_or(false, |status| status.success()) {
                return true;
            }
        }
        
        false
    }
    
    fn render_ansi_text(&self, ui: &mut egui::Ui, text: &str) {
        // Split text into lines to handle line breaks properly
        let lines = text.split('\n');
        
        for line in lines {
            self.render_ansi_line(ui, line);
        }
    }
    
    fn render_ansi_line(&self, ui: &mut egui::Ui, line: &str) {
        // Check if this looks like PowerShell Get-ChildItem output
        if self.is_powershell_listing_line(line) {
            self.render_powershell_listing(ui, line);
            return;
        }
        
        // Default ANSI rendering for other content
        let mut current_color = egui::Color32::WHITE;
        let mut current_bg_color = egui::Color32::TRANSPARENT;
        let mut is_bold = false;
        let mut current_text = String::new();
        let mut chars = line.chars().peekable();
        let mut text_segments = Vec::new();
        
        while let Some(ch) = chars.next() {
            if ch == '\x1b' || ch == '\x1B' { // ESC character
                // Save any accumulated text first
                if !current_text.is_empty() {
                    text_segments.push((current_text.clone(), current_color, current_bg_color, is_bold));
                    current_text.clear();
                }
                
                // Parse the escape sequence
                if chars.peek() == Some(&'[') {
                    chars.next(); // consume '['
                    let mut escape_sequence = String::new();
                    
                    // Collect the escape sequence
                    while let Some(escape_ch) = chars.next() {
                        if escape_ch.is_alphabetic() {
                            break;
                        }
                        escape_sequence.push(escape_ch);
                    }
                    
                    // Parse the color codes
                    let codes: Vec<u8> = escape_sequence
                        .split(';')
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    
                    for code in codes {
                        match code {
                            0 => { // Reset
                                current_color = egui::Color32::WHITE;
                                current_bg_color = egui::Color32::TRANSPARENT;
                                is_bold = false;
                            }
                            1 => is_bold = true, // Bold
                            22 => is_bold = false, // Normal intensity
                            30 => current_color = egui::Color32::BLACK,
                            31 => current_color = egui::Color32::RED,
                            32 => current_color = egui::Color32::GREEN,
                            33 => current_color = egui::Color32::YELLOW,
                            34 => current_color = egui::Color32::BLUE,
                            35 => current_color = egui::Color32::from_rgb(255, 0, 255), // Magenta
                            36 => current_color = egui::Color32::from_rgb(0, 255, 255), // Cyan
                            37 => current_color = egui::Color32::WHITE,
                            90 => current_color = egui::Color32::GRAY, // Bright black
                            91 => current_color = egui::Color32::from_rgb(255, 100, 100), // Bright red
                            92 => current_color = egui::Color32::from_rgb(100, 255, 100), // Bright green
                            93 => current_color = egui::Color32::from_rgb(255, 255, 100), // Bright yellow
                            94 => current_color = egui::Color32::from_rgb(100, 100, 255), // Bright blue
                            95 => current_color = egui::Color32::from_rgb(255, 100, 255), // Bright magenta
                            96 => current_color = egui::Color32::from_rgb(100, 255, 255), // Bright cyan
                            97 => current_color = egui::Color32::from_rgb(240, 240, 240), // Bright white
                            40 => current_bg_color = egui::Color32::BLACK,
                            41 => current_bg_color = egui::Color32::RED,
                            42 => current_bg_color = egui::Color32::GREEN,
                            43 => current_bg_color = egui::Color32::YELLOW,
                            44 => current_bg_color = egui::Color32::BLUE,
                            45 => current_bg_color = egui::Color32::from_rgb(255, 0, 255), // Magenta
                            46 => current_bg_color = egui::Color32::from_rgb(0, 255, 255), // Cyan
                            47 => current_bg_color = egui::Color32::WHITE,
                            _ => {} // Ignore unknown codes
                        }
                    }
                }
            } else {
                current_text.push(ch);
            }
        }
        
        // Save any remaining text
        if !current_text.is_empty() {
            text_segments.push((current_text, current_color, current_bg_color, is_bold));
        }
        
        // Render all segments on the same line
        if !text_segments.is_empty() {
            ui.horizontal(|ui| {
                for (text, color, bg_color, is_bold) in text_segments {
                    self.render_colored_text(ui, &text, color, bg_color, is_bold);
                }
            });
        }
    }
    
    fn human_readable_size(&self, size: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = size as f64;
        let mut unit_index = 0;
        
        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }
        
        if unit_index == 0 {
            format!("{} {}", size as u64, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size, UNITS[unit_index])
        }
    }
    
    fn is_powershell_listing_line(&self, line: &str) -> bool {
        // Remove ANSI codes for pattern matching
        let clean_line = self.strip_ansi_codes(line);
        
        // Check if it matches PowerShell Get-ChildItem output pattern
        // Pattern: Mode LastWriteTime Length Name
        // Example: d---- 7/21/2025 4:41 AM models
        // Example: -a--- 7/21/2025 3:51 AM 82833 src
        let parts: Vec<&str> = clean_line.split_whitespace().collect();
        
        if parts.len() >= 4 {
            // First part should be mode (like d----, -a---, etc.)
            if parts[0].len() >= 4 && (parts[0].starts_with('d') || parts[0].starts_with('-')) {
                // Check if we have date pattern (MM/DD/YYYY or similar)
                if parts.len() >= 5 && parts[1].contains('/') && parts[3].to_lowercase().ends_with('m') {
                    return true;
                }
            }
        }
        
        false
    }
    
    fn strip_ansi_codes(&self, text: &str) -> String {
        let mut result = String::new();
        let mut chars = text.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '\x1b' || ch == '\x1B' {
                if chars.peek() == Some(&'[') {
                    chars.next(); // consume '['
                    // Skip until we find an alphabetic character
                    while let Some(escape_ch) = chars.next() {
                        if escape_ch.is_alphabetic() {
                            break;
                        }
                    }
                }
            } else {
                result.push(ch);
            }
        }
        
        result
    }
    
    fn render_powershell_listing(&self, ui: &mut egui::Ui, line: &str) {
        // Parse the line with ANSI codes preserved
        let clean_line = self.strip_ansi_codes(line);
        let parts: Vec<&str> = clean_line.split_whitespace().collect();
        
        if parts.len() < 4 {
            // Fallback to regular rendering if parsing fails
            self.render_ansi_line_default(ui, line);
            return;
        }
        
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0; // Add some spacing between columns
            
            // Mode column (fixed width)
            let mode = parts[0];
            let mode_color = if mode.starts_with('d') {
                egui::Color32::from_rgb(100, 149, 237) // Blue for directories
            } else {
                egui::Color32::WHITE
            };
            ui.colored_label(mode_color, format!("{:<6}", mode));
            
            // Date and time columns
            if parts.len() >= 4 {
                let date = parts[1];
                let time1 = parts[2];
                let time2 = parts[3];
                ui.colored_label(egui::Color32::GRAY, format!("{} {} {}", date, time1, time2));
            }
            
            // Length column (if present) - right aligned
            let name_start_index = if parts.len() > 4 && parts[4].chars().all(|c| c.is_ascii_digit()) {
                // Has size column
                if let Ok(size) = parts[4].parse::<u64>() {
                    ui.colored_label(egui::Color32::YELLOW, format!("{:>10}", self.human_readable_size(size)));
                } else {
                    ui.colored_label(egui::Color32::YELLOW, format!("{:>10}", parts[4]));
                }
                5
            } else {
                // No size column, it's a directory
                ui.colored_label(egui::Color32::YELLOW, format!("{:>10}", ""));
                4
            };
            
            // Name column (rest of the line)
            if parts.len() > name_start_index {
                let name = parts[name_start_index..].join(" ");
                let name_color = if mode.starts_with('d') {
                    egui::Color32::from_rgb(100, 149, 237) // Blue for directories
                } else {
                    egui::Color32::WHITE
                };
                ui.colored_label(name_color, name);
            }
        });
    }
    
    fn render_ansi_line_default(&self, ui: &mut egui::Ui, line: &str) {
        // This is the original ANSI rendering logic for non-PowerShell content
        let mut current_color = egui::Color32::WHITE;
        let mut current_bg_color = egui::Color32::TRANSPARENT;
        let mut is_bold = false;
        let mut current_text = String::new();
        let mut chars = line.chars().peekable();
        let mut text_segments = Vec::new();
        
        while let Some(ch) = chars.next() {
            if ch == '\x1b' || ch == '\x1B' {
                if !current_text.is_empty() {
                    text_segments.push((current_text.clone(), current_color, current_bg_color, is_bold));
                    current_text.clear();
                }
                
                if chars.peek() == Some(&'[') {
                    chars.next();
                    let mut escape_sequence = String::new();
                    
                    while let Some(escape_ch) = chars.next() {
                        if escape_ch.is_alphabetic() {
                            break;
                        }
                        escape_sequence.push(escape_ch);
                    }
                    
                    let codes: Vec<u8> = escape_sequence
                        .split(';')
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    
                    for code in codes {
                        match code {
                            0 => {
                                current_color = egui::Color32::WHITE;
                                current_bg_color = egui::Color32::TRANSPARENT;
                                is_bold = false;
                            }
                            1 => is_bold = true,
                            30 => current_color = egui::Color32::BLACK,
                            31 => current_color = egui::Color32::RED,
                            32 => current_color = egui::Color32::GREEN,
                            33 => current_color = egui::Color32::YELLOW,
                            34 => current_color = egui::Color32::BLUE,
                            35 => current_color = egui::Color32::from_rgb(255, 0, 255),
                            36 => current_color = egui::Color32::from_rgb(0, 255, 255),
                            37 => current_color = egui::Color32::WHITE,
                            _ => {}
                        }
                    }
                }
            } else {
                current_text.push(ch);
            }
        }
        
        if !current_text.is_empty() {
            text_segments.push((current_text, current_color, current_bg_color, is_bold));
        }
        
        if !text_segments.is_empty() {
            ui.horizontal(|ui| {
                for (text, color, bg_color, is_bold) in text_segments {
                    self.render_colored_text(ui, &text, color, bg_color, is_bold);
                }
            });
        }
    }
    
    fn render_colored_text(&self, ui: &mut egui::Ui, text: &str, color: egui::Color32, bg_color: egui::Color32, is_bold: bool) {
        let mut rich_text = egui::RichText::new(text).color(color);
        
        if is_bold {
            rich_text = rich_text.strong();
        }
        
        if bg_color != egui::Color32::TRANSPARENT {
            rich_text = rich_text.background_color(bg_color);
        }
        
        ui.label(rich_text);
    }
    
    
    fn execute_command(&self, command: &str) -> Result<std::process::Output, std::io::Error> {
        use std::process::{Command, Stdio};
        
        if cfg!(target_os = "windows") {
            // Try PowerShell Core first (pwsh)
            if Command::new("pwsh")
                .arg("--version")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .map_or(false, |s| s.success()) {
                return Command::new("pwsh")
                    .arg("-NoProfile")
                    .arg("-NonInteractive")
                    .arg("-Command")
                    .arg(command)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .output();
            }
            
            // Try Windows PowerShell (powershell)
            if Command::new("powershell")
                .arg("-Command")
                .arg("$true")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .map_or(false, |s| s.success()) {
                return Command::new("powershell")
                    .arg("-NoProfile")
                    .arg("-NonInteractive")
                    .arg("-Command")
                    .arg(command)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .output();
            }
            
            // Fallback to CMD
            Command::new("cmd")
                .arg("/C")
                .arg(command)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
        } else {
            // Unix/Linux systems - allow colors
            Command::new("sh")
                .arg("-c")
                .arg(command)
                .env("TERM", "xterm-256color")
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
        }
    }

    fn render_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");
        ui.label("Configure your LLM settings:");

        // Provider selection
        ui.horizontal(|ui| {
            ui.label("Default Provider:");
            egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", self.app_state.settings.default_provider))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.app_state.settings.default_provider, LLMProviderType::Claude, "Claude");
                    ui.selectable_value(&mut self.app_state.settings.default_provider, LLMProviderType::OpenAI, "OpenAI");
                });
        });

        ui.separator();
        ui.label("API Keys:");

        ui.horizontal(|ui| {
            ui.label("Claude API Key:");
            ui.text_edit_singleline(&mut self.app_state.settings.claude_api_key);
        });

        ui.horizontal(|ui| {
            ui.label("OpenAI API Key:");
            ui.text_edit_singleline(&mut self.app_state.settings.openai_api_key);
        });

        ui.separator();
        ui.label("Models:");

        ui.horizontal(|ui| {
            ui.label("Claude Model:");
            egui::ComboBox::from_label("claude_model")
                .selected_text(&self.app_state.settings.claude_model)
                .width(200.0)
                .show_ui(ui, |ui| {
                    for model in &self.app_state.available_claude_models {
                        ui.selectable_value(&mut self.app_state.settings.claude_model, model.clone(), model);
                    }
                    // Always include current model as an option even if not in available list
                    if !self.app_state.available_claude_models.contains(&self.app_state.settings.claude_model) {
                        let current_model = self.app_state.settings.claude_model.clone();
                        ui.selectable_value(&mut self.app_state.settings.claude_model, current_model.clone(), &format!("{} (current)", current_model));
                    }
                });
        });

        ui.horizontal(|ui| {
            ui.label("OpenAI Model:");
            egui::ComboBox::from_label("openai_model")
                .selected_text(&self.app_state.settings.openai_model)
                .width(200.0)
                .show_ui(ui, |ui| {
                    for model in &self.app_state.available_openai_models {
                        ui.selectable_value(&mut self.app_state.settings.openai_model, model.clone(), model);
                    }
                    // Always include current model as an option even if not in available list
                    if !self.app_state.available_openai_models.contains(&self.app_state.settings.openai_model) {
                        let current_model = self.app_state.settings.openai_model.clone();
                        ui.selectable_value(&mut self.app_state.settings.openai_model, current_model.clone(), &format!("{} (current)", current_model));
                    }
                });
        });

        // Allow custom model input
        ui.horizontal(|ui| {
            ui.label("Custom Claude Model:");
            ui.text_edit_singleline(&mut self.app_state.settings.claude_model);
        });

        ui.horizontal(|ui| {
            ui.label("Custom OpenAI Model:");
            ui.text_edit_singleline(&mut self.app_state.settings.openai_model);
        });

        ui.separator();
        ui.label("Generation Parameters:");

        ui.horizontal(|ui| {
            ui.label("Max Tokens:");
            ui.add(egui::Slider::new(&mut self.app_state.settings.max_tokens, 100..=4000));
        });

        ui.horizontal(|ui| {
            ui.label("Temperature:");
            ui.add(egui::Slider::new(&mut self.app_state.settings.temperature, 0.0..=2.0).step_by(0.1));
        });

        ui.separator();
        ui.label("Model Management:");

        ui.horizontal(|ui| {
            if ui.button("🔄 Fetch Latest Models").clicked() {
                self.fetch_latest_models();
            }
            if ui.button("📁 Load Available Models").clicked() {
                self.load_available_models_to_ui();
            }
        });

        ui.separator();

        if ui.button("Save Settings").clicked() {
            if let Err(e) = self.app_state.settings.save_to_file("config.json") {
                println!("Failed to save settings: {}", e);
                self.app_state.terminal_output.push(format!("Error saving settings: {}", e));
            } else {
                println!("Settings saved successfully!");
                self.app_state.terminal_output.push("Settings saved successfully!".to_string());
            }
        }
    }
}

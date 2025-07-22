# agentic_code_assistant.py with AutoGen, PyQt GUI, knowledge base, and dynamic model picker

import os
import json
import openai
import dotenv
import platform
import subprocess
import sys
import requests
from langchain.agents import create_openai_functions_agent, AgentExecutor
from langchain_openai import ChatOpenAI
from langchain.tools import Tool
from langchain import hub
from PyQt5.QtWidgets import (QApplication, QWidget, QVBoxLayout, QPushButton, QTextEdit, 
                             QLineEdit, QLabel, QComboBox, QTabWidget, QHBoxLayout, 
                             QFormLayout, QGroupBox, QCheckBox, QMessageBox, QSplitter)
# AutoGen imports - will handle these when needed
try:
    from autogen_agentchat import AssistantAgent, UserProxyAgent, GroupChat, GroupChatManager
except ImportError:
    # Fallback if AutoGen not available
    AssistantAgent = None
    UserProxyAgent = None
    GroupChat = None
    GroupChatManager = None

dotenv.load_dotenv()
OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")
ANTHROPIC_API_KEY = os.getenv("ANTHROPIC_API_KEY")

if OPENAI_API_KEY:
    openai.api_key = OPENAI_API_KEY

KNOWLEDGE_DB_PATH = "knowledge_base.json"

def load_knowledge():
    if os.path.exists(KNOWLEDGE_DB_PATH):
        with open(KNOWLEDGE_DB_PATH, "r", encoding="utf-8") as f:
            return json.load(f)
    return {}

def query_knowledge(prompt):
    knowledge = load_knowledge()
    matches = [f"{k}: {v}" for k, v in knowledge.items() if prompt.lower() in k.lower() or prompt.lower() in v.lower()]
    return "\n".join(matches) if matches else "No relevant info in knowledge base."

def get_openai_models():
    if not OPENAI_API_KEY:
        return []
    try:
        models = openai.Model.list().data
        return sorted([m.id for m in models if m.id.startswith("gpt")])
    except Exception:
        return ["gpt-4o", "gpt-3.5-turbo"]

def get_claude_models():
    if not ANTHROPIC_API_KEY:
        return []
    try:
        headers = {
            "x-api-key": ANTHROPIC_API_KEY,
            "anthropic-version": "2023-06-01"
        }
        response = requests.get("https://api.anthropic.com/v1/models", headers=headers)
        return [m["id"] for m in response.json().get("data", [])]
    except Exception:
        return ["claude-3-opus-20240229", "claude-3-sonnet-20240229"]

def create_file(path, content):
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)
    return f"Created file at: {path}"

def read_file(path):
    with open(path, "r", encoding="utf-8") as f:
        return f.read()

def modify_file(path, content):
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)
    return f"Modified file: {path}"

def delete_path(path):
    if os.path.isdir(path):
        os.rmdir(path)
        return f"Deleted folder: {path}"
    elif os.path.isfile(path):
        os.remove(path)
        return f"Deleted file: {path}"
    return "Path does not exist"

def list_dir(path):
    return os.listdir(path)

def run_file(path):
    try:
        if platform.system() == "Windows":
            os.startfile(path)
        elif platform.system() == "Darwin":
            subprocess.run(["open", path], check=True)
        else:
            subprocess.run(["xdg-open", path], check=True)
        return f"Executed: {path}"
    except Exception as e:
        return f"Error executing file: {e}"

def run_command(command):
    try:
        result = subprocess.run(command, shell=True, capture_output=True, text=True)
        return result.stdout or result.stderr
    except Exception as e:
        return f"Error running command: {e}"

# Safe wrapper functions for tools
def create_file_safe(x):
    try:
        data = json.loads(x) if isinstance(x, str) else x
        return create_file(data.get('path', ''), data.get('content', ''))
    except Exception as e:
        return f"Error creating file: {e}"

def read_file_safe(x):
    try:
        data = json.loads(x) if isinstance(x, str) else x
        return read_file(data.get('path', ''))
    except Exception as e:
        return f"Error reading file: {e}"

def modify_file_safe(x):
    try:
        data = json.loads(x) if isinstance(x, str) else x
        return modify_file(data.get('path', ''), data.get('content', ''))
    except Exception as e:
        return f"Error modifying file: {e}"

def delete_path_safe(x):
    try:
        data = json.loads(x) if isinstance(x, str) else x
        return delete_path(data.get('path', ''))
    except Exception as e:
        return f"Error deleting path: {e}"

def list_dir_safe(x):
    try:
        data = json.loads(x) if isinstance(x, str) else x
        path = data.get('path', '.')
        files = list_dir(path)
        return f"Files in {path}: {', '.join(files)}"
    except Exception as e:
        return f"Error listing directory: {e}"

def run_file_safe(x):
    try:
        data = json.loads(x) if isinstance(x, str) else x
        return run_file(data.get('path', ''))
    except Exception as e:
        return f"Error running file: {e}"

def run_command_safe(x):
    try:
        data = json.loads(x) if isinstance(x, str) else x
        return run_command(data.get('command', ''))
    except Exception as e:
        return f"Error running command: {e}"

def autonomous_agent(model_name="gpt-4o"):
    tools = [
        Tool(name="Create File", func=lambda x: create_file_safe(x), description="Create a file at path with content. Input: '{\"path\": \"filename\", \"content\": \"file content\"}'."),
        Tool(name="Read File", func=lambda x: read_file_safe(x), description="Read content from file. Input: '{\"path\": \"filename\"}'."),
        Tool(name="Modify File", func=lambda x: modify_file_safe(x), description="Modify content of file. Input: '{\"path\": \"filename\", \"content\": \"new content\"}'."),
        Tool(name="Delete Path", func=lambda x: delete_path_safe(x), description="Delete file or folder. Input: '{\"path\": \"filename\"}'."),
        Tool(name="List Dir", func=lambda x: list_dir_safe(x), description="List files in a directory. Input: '{\"path\": \"directory_path\"}'."),
        Tool(name="Run File", func=lambda x: run_file_safe(x), description="Execute any type of file using the system default. Input: '{\"path\": \"filename\"}'."),
        Tool(name="Run Command", func=lambda x: run_command_safe(x), description="Run a terminal command and return output. Input: '{\"command\": \"command_to_run\"}'.")
    ]
    
    llm = ChatOpenAI(model=model_name, api_key=OPENAI_API_KEY)
    
    # Use a simple prompt for function calling
    prompt = hub.pull("hwchase17/openai-functions-agent")
    
    agent = create_openai_functions_agent(llm, tools, prompt)
    agent_executor = AgentExecutor(agent=agent, tools=tools, verbose=True)
    
    return agent_executor

def plan_and_execute(user_goal, model_name="gpt-4o"):
    if not all([AssistantAgent, UserProxyAgent, GroupChat, GroupChatManager]):
        return "AutoGen multi-agent system not available. Using single agent instead."
    
    try:
        planner = AssistantAgent(name="Planner", llm_config={"config_list": [{"model": model_name}]})
        executor = AssistantAgent(name="Executor", llm_config={"config_list": [{"model": model_name}]})
        user_proxy = UserProxyAgent(name="User", code_execution_config={"work_dir": "."})
        chat = GroupChat(agents=[user_proxy, planner, executor], messages=[], max_round=10)
        manager = GroupChatManager(groupchat=chat, llm_config={"config_list": [{"model": model_name}]})
        user_proxy.initiate_chat(manager, message=user_goal)
        return "Task passed to autonomous multi-agent system."
    except Exception as e:
        return f"AutoGen error: {e}. Falling back to single agent."

class SettingsTab(QWidget):
    def __init__(self, parent=None):
        super().__init__()
        self.parent = parent
        self.init_ui()
        
    def init_ui(self):
        main_layout = QVBoxLayout()
        
        # API Keys Group
        api_group = QGroupBox("API Configuration")
        api_layout = QFormLayout()
        
        # Current API key status
        self.openai_status = QLabel()
        self.anthropic_status = QLabel()
        self.update_status_labels()
        
        # API Key Inputs with show/hide toggle
        openai_layout = QHBoxLayout()
        self.openai_key_input = QLineEdit()
        self.openai_key_input.setEchoMode(QLineEdit.Password)
        self.openai_key_input.setPlaceholderText("Enter OpenAI API Key")
        self.openai_show_btn = QPushButton("Show")
        self.openai_show_btn.clicked.connect(lambda: self.toggle_visibility(self.openai_key_input, self.openai_show_btn))
        openai_layout.addWidget(self.openai_key_input)
        openai_layout.addWidget(self.openai_show_btn)
        
        anthropic_layout = QHBoxLayout()
        self.anthropic_key_input = QLineEdit()
        self.anthropic_key_input.setEchoMode(QLineEdit.Password)
        self.anthropic_key_input.setPlaceholderText("Enter Anthropic API Key (Optional)")
        self.anthropic_show_btn = QPushButton("Show")
        self.anthropic_show_btn.clicked.connect(lambda: self.toggle_visibility(self.anthropic_key_input, self.anthropic_show_btn))
        anthropic_layout.addWidget(self.anthropic_key_input)
        anthropic_layout.addWidget(self.anthropic_show_btn)
        
        # Add to form layout
        api_layout.addRow("OpenAI Status:", self.openai_status)
        api_layout.addRow("OpenAI API Key:", openai_layout)
        api_layout.addRow("Anthropic Status:", self.anthropic_status)
        api_layout.addRow("Anthropic API Key:", anthropic_layout)
        
        api_group.setLayout(api_layout)
        
        # Buttons
        button_layout = QHBoxLayout()
        save_button = QPushButton("Save Settings")
        save_button.clicked.connect(self.save_keys)
        test_button = QPushButton("Test API Keys")
        test_button.clicked.connect(self.test_api_keys)
        clear_button = QPushButton("Clear Keys")
        clear_button.clicked.connect(self.clear_keys)
        
        button_layout.addWidget(save_button)
        button_layout.addWidget(test_button)
        button_layout.addWidget(clear_button)
        button_layout.addStretch()
        
        # Instructions
        instructions = QLabel(
            "Instructions:\n"
            "1. Enter your API keys above\n"
            "2. Click 'Test API Keys' to verify they work\n"
            "3. Click 'Save Settings' to store them\n\n"
            "OpenAI API Key is required for AI functionality.\n"
            "Anthropic API Key is optional for Claude models."
        )
        instructions.setWordWrap(True)
        instructions.setStyleSheet("QLabel { color: #666; padding: 10px; }")
        
        main_layout.addWidget(api_group)
        main_layout.addLayout(button_layout)
        main_layout.addWidget(instructions)
        main_layout.addStretch()
        
        self.setLayout(main_layout)
    
    def update_status_labels(self):
        # Check OpenAI key status
        if OPENAI_API_KEY and len(OPENAI_API_KEY) > 10:
            self.openai_status.setText(f"✓ Configured ({OPENAI_API_KEY[:10]}...)")
            self.openai_status.setStyleSheet("color: green;")
        else:
            self.openai_status.setText("✗ Not configured")
            self.openai_status.setStyleSheet("color: red;")
            
        # Check Anthropic key status  
        if ANTHROPIC_API_KEY and len(ANTHROPIC_API_KEY) > 10:
            self.anthropic_status.setText(f"✓ Configured ({ANTHROPIC_API_KEY[:10]}...)")
            self.anthropic_status.setStyleSheet("color: green;")
        else:
            self.anthropic_status.setText("✗ Not configured")
            self.anthropic_status.setStyleSheet("color: orange;")
    
    def toggle_visibility(self, input_field, button):
        if input_field.echoMode() == QLineEdit.Password:
            input_field.setEchoMode(QLineEdit.Normal)
            button.setText("Hide")
        else:
            input_field.setEchoMode(QLineEdit.Password)
            button.setText("Show")
    
    def save_keys(self):
        new_openai_key = self.openai_key_input.text().strip()
        new_anthropic_key = self.anthropic_key_input.text().strip()
        
        try:
            dotenv_file_path = '.env'
            
            # Update OpenAI key
            if new_openai_key:
                dotenv.set_key(dotenv_file_path, "OPENAI_API_KEY", new_openai_key)
                global OPENAI_API_KEY
                OPENAI_API_KEY = new_openai_key
                openai.api_key = new_openai_key
                
            # Update Anthropic key
            if new_anthropic_key:
                dotenv.set_key(dotenv_file_path, "ANTHROPIC_API_KEY", new_anthropic_key)
                global ANTHROPIC_API_KEY
                ANTHROPIC_API_KEY = new_anthropic_key
            
            # Clear input fields
            self.openai_key_input.clear()
            self.anthropic_key_input.clear()
            
            # Update status
            self.update_status_labels()
            
            # Refresh models if parent exists
            if hasattr(self.parent, 'assistant_tab'):
                self.parent.assistant_tab.refresh_models()
            
            QMessageBox.information(self, "Success", "API keys have been saved successfully!")
            
        except Exception as e:
            QMessageBox.critical(self, "Error", f"Failed to save API keys: {e}")
    
    def test_api_keys(self):
        # Test OpenAI
        openai_result = "✗ Not tested"
        if OPENAI_API_KEY:
            try:
                # Simple test call
                client = openai.OpenAI(api_key=OPENAI_API_KEY)
                models = client.models.list()
                openai_result = "✓ Working"
            except Exception as e:
                openai_result = f"✗ Error: {str(e)[:50]}..."
        
        # Test Anthropic (basic check)
        anthropic_result = "✗ Not tested"
        if ANTHROPIC_API_KEY:
            try:
                headers = {
                    "x-api-key": ANTHROPIC_API_KEY,
                    "anthropic-version": "2023-06-01"
                }
                response = requests.get("https://api.anthropic.com/v1/models", headers=headers, timeout=10)
                if response.status_code == 200:
                    anthropic_result = "✓ Working"
                else:
                    anthropic_result = f"✗ HTTP {response.status_code}"
            except Exception as e:
                anthropic_result = f"✗ Error: {str(e)[:50]}..."
        
        # Show results
        message = f"API Key Test Results:\n\nOpenAI: {openai_result}\nAnthropic: {anthropic_result}"
        QMessageBox.information(self, "API Test Results", message)
    
    def clear_keys(self):
        reply = QMessageBox.question(self, "Clear API Keys", 
                                   "Are you sure you want to clear all API keys?",
                                   QMessageBox.Yes | QMessageBox.No)
        
        if reply == QMessageBox.Yes:
            try:
                dotenv_file_path = '.env'
                dotenv.set_key(dotenv_file_path, "OPENAI_API_KEY", "")
                dotenv.set_key(dotenv_file_path, "ANTHROPIC_API_KEY", "")
                
                global OPENAI_API_KEY, ANTHROPIC_API_KEY
                OPENAI_API_KEY = None
                ANTHROPIC_API_KEY = None
                openai.api_key = None
                
                self.update_status_labels()
                
                QMessageBox.information(self, "Success", "API keys have been cleared.")
            except Exception as e:
                QMessageBox.critical(self, "Error", f"Failed to clear API keys: {e}")


class MainWindow(QWidget):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("Agentic Code Assistant")
        self.setGeometry(100, 100, 800, 600)

        # Create tabs
        self.tabs = QTabWidget()
        self.assistant_tab = AssistantGUI()
        self.settings_tab = SettingsTab(parent=self)
        
        self.tabs.addTab(self.assistant_tab, "Main")
        self.tabs.addTab(self.settings_tab, "Settings")

        # Set layout
        self.layout = QVBoxLayout()
        self.layout.addWidget(self.tabs)
        self.setLayout(self.layout)


class AssistantGUI(QWidget):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("Agentic Code Assistant")
        self.setGeometry(100, 100, 800, 600)
        self.layout = QVBoxLayout()
        self.label = QLabel("Enter your request:")
        self.layout.addWidget(self.label)
        self.input_field = QLineEdit()
        self.layout.addWidget(self.input_field)
        self.model_picker = QComboBox()
        self.refresh_models()
        self.layout.addWidget(QLabel("Select model:"))
        self.layout.addWidget(self.model_picker)
        self.run_button = QPushButton("Execute")
        self.run_button.clicked.connect(self.run_task)
        self.layout.addWidget(self.run_button)
        self.output_area = QTextEdit()
        self.output_area.setReadOnly(True)
        self.layout.addWidget(self.output_area)
        self.setLayout(self.layout)

    def refresh_models(self):
        models = get_openai_models() + get_claude_models()
        self.model_picker.clear()
        self.model_picker.addItems(models if models else ["No models available"])

    def run_task(self):
        user_input = self.input_field.text()
        model = self.model_picker.currentText()
        if user_input.lower() in ["exit", "quit"]:
            self.close()
        else:
            context_info = query_knowledge(user_input)
            full_prompt = f"User request: {user_input}\n\nRelevant knowledge:\n{context_info}"
            if any(keyword in user_input.lower() for keyword in ["plan", "project", "steps", "build"]):
                result = plan_and_execute(full_prompt, model_name=model)
            else:
                agent = autonomous_agent(model_name=model)
                result = agent.invoke({"input": full_prompt})
            self.output_area.append(f"> {user_input}\n{result}\n")

if __name__ == "__main__":
    app = QApplication(sys.argv)
    window = MainWindow()
    window.show()
    sys.exit(app.exec_())


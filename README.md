# LLMTerminal - Agentic Code Assistant

[![Python 3.8+](https://img.shields.io/badge/python-3.8+-blue.svg)](https://www.python.org/downloads/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![PyQt5](https://img.shields.io/badge/GUI-PyQt5-green.svg)](https://pypi.org/project/PyQt5/)

An intelligent, multi-agent code assistant that combines OpenAI GPT models, Anthropic Claude, AutoGen, and LangChain to provide autonomous coding assistance through a user-friendly PyQt5 GUI.

## 📚 Table of Contents

- [✨ Features](#-features)
- [📝 Prerequisites](#prerequisites)
- [🚀 Quick Start Guide](#-quick-start-guide)
- [💡 Usage Examples](#-usage-examples)
- [⚙️ Settings Tab Features](#-settings-tab-features)
- [🏗️ Architecture](#-architecture)
- [📦 Dependencies](#-dependencies)
- [🧪 Testing](#-testing)
- [🔧 Troubleshooting](#-troubleshooting)
- [🤝 Contributing](#-contributing)

## 📸 Screenshots

### Main Interface
The application features a clean tabbed interface with separate Main and Settings tabs.

### Settings Tab
Secure API key management with real-time status indicators and testing capabilities.

*Note: Screenshots will be added in future updates*

## ✨ Features

### 🤖 AI & Automation
- **Multi-Agent System**: Uses AutoGen for collaborative task planning and execution
- **Multiple AI Models**: Supports OpenAI GPT-4, GPT-3.5, and Anthropic Claude models
- **Intelligent Tool Selection**: Automatically chooses the right tools for each task
- **Context-Aware Responses**: Maintains conversation context and learns from interactions

### 🖥️ User Interface
- **Modern Tabbed Interface**: Clean PyQt5 GUI with Main and Settings tabs
- **Settings Management**: Built-in API key configuration with secure input fields
- **Real-time Status**: Visual indicators showing system status and configuration
- **Cross-Platform**: Works seamlessly on Windows, macOS, and Linux

### 📁 File & System Operations
- **File Management**: Create, read, modify, and delete files with ease
- **Directory Operations**: List and navigate directory structures
- **Command Execution**: Run system commands and capture output
- **File Execution**: Launch files using system default applications

### 🧠 Knowledge & Learning
- **Knowledge Base**: JSON-based storage for learned information and context
- **Smart Querying**: Context-aware information retrieval
- **Persistent Memory**: Retains information across sessions

## Prerequisites

- Python 3.8 or higher
- OpenAI API key (required)
- Anthropic API key (optional, for Claude models)

## Quick Setup

### Windows (PowerShell)
```powershell
# Run the setup script
.\setup.ps1

# Edit the .env file with your API keys
notepad .env

# Run the application
.\run.ps1
```

### Windows (Command Prompt)
```cmd
# Run the setup script
setup.bat

# Edit the .env file with your API keys
notepad .env

# Run the application
run.bat
```

### Manual Setup

1. **Clone and navigate to the repository**
   ```bash
   cd LLMTerminal
   ```

2. **Create virtual environment**
   ```bash
   python -m venv venv
   ```

3. **Activate virtual environment**
   - Windows: `venv\Scripts\activate`
   - macOS/Linux: `source venv/bin/activate`

4. **Install dependencies**
   ```bash
   pip install -r requirements.txt
   ```

5. **Setup environment variables**
   ```bash
   cp .env.example .env
   # Edit .env and add your API keys
   ```

6. **Run the application**
   ```bash
   python main.py
   ```

## Configuration

Edit the `.env` file to add your API keys:

```env
# Required
OPENAI_API_KEY=your_openai_api_key_here

# Optional (for Claude models)
ANTHROPIC_API_KEY=your_anthropic_api_key_here
```

## 🚀 Quick Start Guide

### Option 1: GUI Configuration (Recommended)

1. **Launch the Application**
   ```bash
   python main.py
   ```

2. **Configure API Keys**
   - Click on the **Settings** tab
   - Enter your OpenAI API key in the first field
   - (Optional) Enter your Anthropic API key for Claude models
   - Click **Test API Keys** to verify they work
   - Click **Save Settings** to store them securely

3. **Start Using the Assistant**
   - Switch to the **Main** tab
   - Select your preferred AI model from the dropdown
   - Enter your request and click **Execute**

### Option 2: Manual Configuration

Edit the `.env` file directly (created during setup):

```env
OPENAI_API_KEY=sk-your-openai-api-key-here
ANTHROPIC_API_KEY=your-anthropic-api-key-here  # Optional
```

## 💡 Usage Examples

### Simple Tasks
```
"Create a Python function to calculate factorial"
"Read the contents of config.json file"
"List all Python files in the current directory"
"Delete the temporary log files"
```

### Complex Project Tasks
```
"Plan a web scraping project with error handling"
"Build a REST API with user authentication"
"Create a data processing pipeline for CSV files"
"Design a logging system for a Python application"
```

### File Operations
```
"Create a requirements.txt file with these dependencies: requests, pandas, numpy"
"Modify the database configuration in settings.py"
"Read and summarize the README.md file"
"Organize files in the downloads folder by file type"
```

## File Operations

The assistant can perform various file operations:
- Create new files with specific content
- Read and analyze existing files
- Modify file contents
- Delete files and directories
- List directory contents
- Execute files using system defaults
- Run terminal commands

## Settings Tab Features

### API Key Management
- 🔐 **Secure Input**: Password-masked input fields for API keys
- 👁️ **Show/Hide Toggle**: Click "Show" to reveal or hide API keys
- ✅ **Status Display**: Real-time status showing if keys are configured
- 🧪 **API Testing**: Test button to verify API keys work correctly
- 💾 **Auto-Save**: Automatically saves keys to .env file
- 🗑️ **Clear Keys**: Option to clear all stored API keys

### User-Friendly Features
- Visual status indicators (✓ Working, ✗ Not configured)
- Error handling with helpful messages
- Automatic model refresh after saving keys
- No need to manually edit configuration files

## Knowledge Base

The application maintains a `knowledge_base.json` file to store learned information and context for better responses over time.

## 🏗️ Architecture

### Core Components
- **Main Window**: Tabbed interface with Main and Settings tabs
- **Assistant Engine**: LangChain-powered agent with OpenAI Functions
- **Multi-Agent System**: AutoGen integration for complex task planning
- **Tool Framework**: Extensible system for file operations and commands
- **Knowledge Base**: JSON-based persistent storage for context

### Supported AI Models
- **OpenAI**: GPT-4, GPT-4 Turbo, GPT-3.5 Turbo
- **Anthropic**: Claude-3 Opus, Claude-3 Sonnet, Claude-3 Haiku

## 📦 Dependencies

### Core Dependencies
- `openai>=1.0.0` - OpenAI API integration
- `langchain>=0.1.0` - Agent and tool framework
- `langchain-openai>=0.1.0` - OpenAI integration for LangChain
- `langchain-community>=0.1.0` - Community tools and integrations
- `langchainhub>=0.1.0` - Pre-built prompts and chains
- `PyQt5>=5.15.0` - GUI framework
- `pyautogen>=0.2.0` - Multi-agent conversation framework

### Utility Dependencies
- `python-dotenv>=1.0.0` - Environment variable management
- `requests>=2.31.0` - HTTP requests for API calls

## 🧪 Testing

Run the test suite to verify everything is working:

```bash
python test_app.py
```

The test suite checks:
- ✅ All imports and dependencies
- ✅ GUI creation and tab functionality
- ✅ File operations (create, read, delete)
- ✅ Core application functionality

## 🎯 Demo

Try the interactive demo to see all features:

```bash
python demo.py
```

## 🔧 Troubleshooting

### Common Issues

#### Installation Issues
- **Python not found**: Ensure Python 3.8+ is installed and in your PATH
- **PyQt5 fails to install**: Try `pip install PyQt5 --user` or install Qt development tools
- **Virtual environment issues**: Make sure you're in the correct directory and the venv is activated

#### Runtime Issues
- **Module import errors**: Ensure virtual environment is activated (prompt shows `(venv)`)
- **API errors**: Verify your API keys in the Settings tab or `.env` file
- **No models available**: Check API key configuration and internet connection
- **GUI doesn't appear**: Ensure you have a display environment (for WSL users, install X server)

#### Performance Issues
- **Slow responses**: Try using a faster model like GPT-3.5 Turbo
- **Memory issues**: Restart the application if it becomes unresponsive

### Getting Help

1. **Check Installation**:
   ```bash
   pip list
   python --version
   ```

2. **Verify Environment**:
   - Ensure virtual environment is active: `(venv)` in prompt
   - Check current directory: should be in LLMTerminal folder

3. **Test Components**:
   ```bash
   python test_app.py  # Run full test suite
   python demo.py      # Interactive demo
   ```

4. **Debug Mode**:
   - Check the console output when running `python main.py`
   - Look for error messages in the GUI output area

## 🔐 Security Notes

- API keys are stored locally in the `.env` file
- Keys are password-masked in the GUI for security
- No API keys are transmitted except to official OpenAI/Anthropic endpoints
- Clear sensitive data using the "Clear Keys" button when needed

## 🚀 Performance Tips

- **Model Selection**: GPT-3.5 Turbo is faster and cheaper than GPT-4
- **Task Complexity**: Use simple requests for basic operations
- **Batch Operations**: Combine related tasks in a single request
- **Knowledge Base**: Let the system learn from your interactions

## 🤝 Contributing

We welcome contributions! Here's how you can help:

1. **Report Issues**: Use GitHub Issues for bugs and feature requests
2. **Submit Pull Requests**: Fork the repo, make changes, and submit a PR
3. **Improve Documentation**: Help make the docs clearer and more comprehensive
4. **Add Features**: Implement new tools, models, or UI improvements

### Development Setup
```bash
git clone <repository-url>
cd LLMTerminal
python -m venv venv
source venv/bin/activate  # or venv\Scripts\activate on Windows
pip install -r requirements.txt
python test_app.py
```

## 📜 License

This project is open source under the MIT License. See the LICENSE file for details.

**Important**: Please ensure you comply with the terms of service for OpenAI and Anthropic APIs when using their models. This application is a client that interfaces with these services.

## 🙏 Acknowledgments

- [OpenAI](https://openai.com/) for the GPT models and API
- [Anthropic](https://www.anthropic.com/) for Claude models
- [LangChain](https://langchain.com/) for the agent framework
- [AutoGen](https://github.com/microsoft/autogen) for multi-agent capabilities
- [PyQt5](https://pypi.org/project/PyQt5/) for the GUI framework

---

**Made with ❤️ by the LLMTerminal Team**

*Star ⭐ this repository if you find it useful!*

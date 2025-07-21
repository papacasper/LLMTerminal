# LLMTerminal

LLMTerminal is a robust terminal application integrated with advanced LLM capabilities.

## Key Features

- **Natural Language Detection**: Enhanced to recognize conversational inputs anywhere in the query.
- **System Context Awareness**: Automatically includes system and directory information in LLM queries.
- **Terminal and LLM Integration**: Capable of distinguishing between natural language queries and terminal commands.
- **Enhanced Query Context**:
  - Current directory and its contents (first 20 items).
  - Operating system and architecture details.
  - Recent terminal commands.

## Getting Started

To start using LLMTerminal, clone the repository and run using the specified environment settings.

```bash
# Clone the repository
git clone <repository_url>

# Change into the project directory
cd LLMTerminal

# Run the application
cargo run
```

Ensure your configuration file has valid API keys for OpenAI and Claude integrations in `config.json`.

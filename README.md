# OmniAgent - Rust A2A + MCP Agent Framework

A comprehensive Rust framework for building agents that support both A2A (Agent-to-Agent) and MCP (Model Context Protocol) protocols with multi-provider LLM support.

## Features

- **A2A Protocol Support**: Full Agent-to-Agent communication via HTTP REST API
- **MCP Protocol Support**: Model Context Protocol for tool integration
- **Multi-Provider LLM Support**:
  - OpenAI (GPT-3.5, GPT-4, etc.)
  - Anthropic Claude (3.5 Sonnet, 3 Haiku, etc.)
  - Google Gemini (Pro, Pro Vision, etc.)
- **Mock Mode**: Development and testing without API keys
- **Async/Await**: Built on tokio runtime
- **Comprehensive Testing**: Mock servers for all providers

## Quick Start

### Installation

```bash
git clone https://github.com/your-username/omni-agent
cd omni-agent
cargo build --release
```

### Basic Usage

```rust
use omni_agent::{
    llm::providers::{LLMRequest, Message, MessageRole},
    llm::providers::google::GoogleProvider,
    llm::providers::LLMProvider,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = GoogleProvider::new(
        "your-google-api-key".to_string(),
        Some("gemini-pro".to_string())
    );

    let request = LLMRequest {
        messages: vec![
            Message {
                role: MessageRole::User,
                content: "Hello, Gemini!".to_string(),
            }
        ],
        model: "gemini-pro".to_string(),
        temperature: Some(0.7),
        max_tokens: Some(100),
        stream: Some(false),
    };

    let response = provider.chat(request).await?;
    println!("Response: {}", response.content);

    Ok(())
}
```

### Environment Variables

- `OPENAI_API_KEY`: OpenAI API key
- `ANTHROPIC_API_KEY`: Claude API key  
- `GOOGLE_API_KEY`: Google AI API key
- `PORT`: Server port (default: 8080)

## Testing

Run tests with mock servers:

```bash
# Run Google Gemini tests
cargo test --test google_integration_test -- --nocapture

# Run Claude tests
cargo test --test claude_integration_test -- --nocapture

# Run all tests
cargo test
```

## Development

### Project Structure

```
src/
├── agent/         # Agent implementation
├── a2a/           # A2A protocol
├── mcp/           # MCP protocol
├── llm/           # LLM providers
├── protocol/      # Shared protocols
└── server/        # HTTP server
```

### LLM Providers

#### Google Gemini
```rust
use omni_agent::llm::providers::google::GoogleProvider;

let provider = GoogleProvider::new(
    "your-google-api-key".to_string(),
    Some("gemini-pro".to_string())
);
```

#### Claude
```rust
use omni_agent::llm::providers::claude::ClaudeProvider;

let provider = ClaudeProvider::new(
    "your-anthropic-api-key".to_string(),
    Some("claude-3-haiku-20240307".to_string())
);
```

#### OpenAI
```rust
use omni_agent::llm::providers::openai::OpenAIProvider;

let provider = OpenAIProvider::new(
    "your-openai-api-key".to_string(),
    Some("gpt-3.5-turbo".to_string())
);
```

## Acknowledgments

- Google Gemini Cookbook for API reference
- Anthropic Claude API documentation
- OpenAI API documentation
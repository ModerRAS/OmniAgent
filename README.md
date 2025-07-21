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

### 1. Installation

```bash
git clone https://github.com/your-username/omni-agent
cd omni-agent
cargo build --release
```

### 2. Configuration Setup

Create a `config.json` file in the project root (auto-generated on first run if missing):

```json
{
  "server": {
    "port": 8080,
    "host": "127.0.0.1"
  },
  "llm": {
    "provider": "google",
    "model": "gemini-pro",
    "temperature": 0.7,
    "max_tokens": 1000,
    "use_mock": false
  },
  "mcp": {
    "servers": [
      {
        "name": "example-mcp",
        "url": "http://localhost:3000"
      }
    ]
  },
  "a2a": {
    "agents": [
      {
        "name": "example-agent",
        "url": "http://localhost:8081"
      }
    ]
  }
}
```

Set your API keys as environment variables:

```bash
# Required for real LLM usage (choose one or more)
export GOOGLE_API_KEY="your-google-api-key"
export OPENAI_API_KEY="your-openai-api-key"
export ANTHROPIC_API_KEY="your-anthropic-api-key"

# Optional: Use mock mode for development
export USE_MOCK=true
```

### 3. Start the Agent Server

```bash
# Run with default configuration
cargo run

# Run with custom configuration
cargo run -- --config custom.json

# Run in mock mode (no API keys needed)
USE_MOCK=true cargo run
```

### 4. Test the Server

Once the server is running, you can test it with curl:

```bash
# Check server health
curl http://localhost:8080/health

# Get agent manifest
curl http://localhost:8080/manifest

# Send a message to the agent
curl -X POST http://localhost:8080/messages \
  -H "Content-Type: application/json" \
  -d '{
    "content": {
      "type": "text",
      "text": "Hello, can you help me?"
    },
    "metadata": {}
  }'
```

## Usage Examples

### Basic Agent Creation

```rust
use omni_agent::AgentBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a simple agent
    let agent = AgentBuilder::new("my-agent", "A helpful assistant")
        .version("1.0.0")
        .build()
        .await?;

    // Use the agent
    let response = agent
        .llm
        .write()
        .await
        .process_message("Hello, how can you help me?", &[])
        .await?;

    println!("Agent response: {}", response.content);
    Ok(())
}
```

### Agent with MCP and A2A Support

```rust
use omni_agent::AgentBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let agent = AgentBuilder::new("advanced-agent", "An agent with tool support")
        .version("1.0.0")
        .add_mcp("file-tools", "http://localhost:3000")
        .add_a2a("weather-agent", "http://localhost:8081")
        .build()
        .await?;

    println!("Agent created with {} MCP clients and {} A2A clients", 
             agent.mcp_clients.len(), 
             agent.a2a_clients.len());

    Ok(())
}
```

### Direct LLM Provider Usage

#### Google Gemini
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
                content: "Explain quantum computing in simple terms".to_string(),
            }
        ],
        model: "gemini-pro".to_string(),
        temperature: Some(0.7),
        max_tokens: Some(200),
        stream: Some(false),
    };

    let response = provider.chat(request).await?;
    println!("Response: {}", response.content);

    Ok(())
}
```

#### Claude
```rust
use omni_agent::{
    llm::providers::{LLMRequest, Message, MessageRole},
    llm::providers::claude::ClaudeProvider,
    llm::providers::LLMProvider,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = ClaudeProvider::new(
        "your-anthropic-api-key".to_string(),
        Some("claude-3-haiku-20240307".to_string())
    );

    let request = LLMRequest {
        messages: vec![
            Message {
                role: MessageRole::System,
                content: "You are a helpful coding assistant".to_string(),
            },
            Message {
                role: MessageRole::User,
                content: "Write a Rust function to reverse a string".to_string(),
            }
        ],
        model: "claude-3-haiku-20240307".to_string(),
        temperature: Some(0.5),
        max_tokens: Some(150),
        stream: Some(false),
    };

    let response = provider.chat(request).await?;
    println!("Response: {}", response.content);

    Ok(())
}
```

#### OpenAI
```rust
use omni_agent::{
    llm::providers::{LLMRequest, Message, MessageRole},
    llm::providers::openai::OpenAIProvider,
    llm::providers::LLMProvider,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = OpenAIProvider::new(
        "your-openai-api-key".to_string(),
        Some("gpt-3.5-turbo".to_string())
    );

    let request = LLMRequest {
        messages: vec![
            Message {
                role: MessageRole::User,
                content: "What's the weather like today?".to_string(),
            }
        ],
        model: "gpt-3.5-turbo".to_string(),
        temperature: Some(0.7),
        max_tokens: Some(100),
        stream: Some(false),
    };

    let response = provider.chat(request).await?;
    println!("Response: {}", response.content);

    Ok(())
}
```

## Testing

### Run All Tests
```bash
cargo test
```

### Run Specific Provider Tests
```bash
# Google Gemini tests
cargo test --test google_integration_test -- --nocapture

# Claude tests
cargo test --test claude_integration_test -- --nocapture

# End-to-end tests (with mock servers)
cargo test --test end_to_end_test -- --nocapture
```

### Run Tests in Mock Mode
```bash
# All tests without real API calls
USE_MOCK=true cargo test
```

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `OPENAI_API_KEY` | OpenAI API key | - |
| `ANTHROPIC_API_KEY` | Claude API key | - |
| `GOOGLE_API_KEY` | Google AI API key | - |
| `PORT` | Server port | 8080 |
| `HOST` | Server host | 127.0.0.1 |
| `USE_MOCK` | Enable mock mode | false |
| `LOG_LEVEL` | Logging level (debug, info, warn, error) | info |
| `OMNI_AGENT_CONFIG` | Custom config file path | config.json |

### Configuration File (config.json)

```json
{
  "server": {
    "port": 8080,
    "host": "127.0.0.1"
  },
  "llm": {
    "provider": "google",
    "model": "gemini-pro",
    "temperature": 0.7,
    "max_tokens": 1000,
    "use_mock": false
  },
  "mcp": {
    "servers": [
      {
        "name": "file-server",
        "url": "http://localhost:3000"
      }
    ]
  },
  "a2a": {
    "agents": [
      {
        "name": "weather-agent",
        "url": "http://localhost:8081"
      }
    ]
  }
}
```

## API Endpoints

### Health Check
```http
GET /health
```

### Agent Manifest
```http
GET /manifest
```

### Send Message
```http
POST /messages
Content-Type: application/json

{
  "content": {
    "type": "text",
    "text": "Your message here"
  },
  "metadata": {}
}
```

### Get Message Status
```http
GET /messages/{message_id}
```

## Development

### Project Structure

```
src/
├── agent/         # Agent implementation and builder
├── a2a/           # A2A protocol client and server
├── llm/           # LLM providers and service layer
├── mcp/           # MCP protocol client
├── protocol/      # Shared protocol definitions
├── server/        # HTTP server implementation
├── config.rs      # Configuration management
├── app.rs         # Main application orchestrator
└── main.rs        # Entry point
```

### Adding New LLM Provider

1. Create provider in `src/llm/providers/[provider].rs`
2. Add config struct to `ProviderConfig`
3. Update `LLMManager` to handle new provider
4. Add tests in appropriate test file

### Running Development Server

```bash
# Watch mode for development
cargo watch -x run

# With custom configuration
cargo run -- --config dev.json

# With logging
cargo run -- --config dev.json 2>&1 | tee agent.log
```

## Troubleshooting

### Common Issues

**Q: The agent won't start and says "API key not found"**
A: Set the appropriate environment variable or enable mock mode:
```bash
export USE_MOCK=true
```

**Q: Tests are failing with network errors**
A: Run tests in mock mode:
```bash
USE_MOCK=true cargo test
```

**Q: How do I use a different LLM provider?**
A: Update the `llm.provider` field in `config.json`:
```json
{
  "llm": {
    "provider": "claude",
    "model": "claude-3-haiku-20240307"
  }
}
```

**Q: Server won't start on port 8080**
A: Change the port in config or use environment variable:
```bash
export PORT=8081
cargo run
```

## Examples

Check out the `examples/` directory for more detailed usage examples:

- `examples/basic_agent.rs` - Simple agent creation
- `examples/with_mcp.rs` - Agent with MCP tool support
- `examples/with_a2a.rs` - Agent with A2A communication
- `examples/custom_config.rs` - Custom configuration usage

## Acknowledgments

- Google Gemini Cookbook for API reference
- Anthropic Claude API documentation
- OpenAI API documentation
- MCP Protocol specification
- A2A Protocol specification
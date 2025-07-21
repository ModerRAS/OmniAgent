# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

OmniAgent is a comprehensive Rust framework for building agents that support both A2A (Agent-to-Agent) and MCP (Model Context Protocol) protocols with multi-provider LLM support.

## Architecture

The codebase follows a modular architecture with these key components:

- **Core Agent System**: Built around the `Agent` struct in `src/agent/`
- **Protocol Support**: A2A and MCP protocol implementations in `src/a2a/` and `src/mcp/`
- **LLM Integration**: Multi-provider support (OpenAI, Claude, Google) in `src/llm/`
- **HTTP Server**: Axum-based A2A server in `src/server/`
- **Configuration**: JSON-based config system in `src/config.rs`

## Key Components

### 1. Configuration (`src/config.rs`)
- `AppConfig`: Central configuration struct with server, LLM, MCP, and A2A settings
- Environment variable overrides for API keys and settings
- Auto-generates `config.json` if not found

### 2. LLM Service (`src/llm/`)
- **Providers**: OpenAI, Claude, Google Gemini with unified interface
- **Mock Mode**: Development/testing without API keys
- **Manager**: `LLMManager` handles provider switching and requests

### 3. Agent System (`src/agent/`)
- **Agent**: Core agent implementation with state management
- **Builder**: `AgentBuilder` for fluent agent creation
- **Capabilities**: Dynamic capability discovery and management

### 4. Protocol Support
- **A2A**: Agent-to-Agent communication via HTTP REST API
- **MCP**: Model Context Protocol for tool integration
- **Messages**: Unified message format across protocols

### 5. Server (`src/server/`)
- **A2AServer**: HTTP server with Axum framework
- **Routes**: `/health`, `/manifest`, `/messages`, `/messages/:id`
- **State Management**: RwLock-based shared state

## Development Commands

### Build & Run
```bash
# Development build
cargo build

# Release build
cargo build --release

# Run with default config
cargo run

# Run with custom config
cargo run -- --config custom.json
```

### Testing
```bash
# Run all tests
cargo test

# Run specific integration tests
cargo test --test google_integration_test -- --nocapture
cargo test --test claude_integration_test -- --nocapture
cargo test --test integration_test -- --nocapture

# Run end-to-end tests
cargo test --test end_to_end_test -- --nocapture
```

### Environment Variables
- `OPENAI_API_KEY`: OpenAI API key
- `ANTHROPIC_API_KEY`: Claude API key
- `GOOGLE_API_KEY`: Google AI API key
- `PORT`: Server port (default: 8080)
- `USE_MOCK`: Enable mock mode (true/false)
- `LOG_LEVEL`: Logging level (debug, info, warn, error)
- `OMNI_AGENT_CONFIG`: Custom config file path

### Configuration
- Default config: `config.json`
- Example config: `config.example.json`
- Auto-generated if missing with sensible defaults

## Code Structure

```
src/
├── agent/           # Agent implementation and builder
├── a2a/            # A2A protocol client and types
├── llm/            # LLM providers and service layer
├── mcp/            # MCP protocol client and types
├── protocol/       # Shared protocol definitions
├── server/         # HTTP server implementation
├── config.rs       # Configuration management
├── app.rs          # Main application orchestrator
└── main.rs         # Entry point
```

## LLM Provider Usage

### OpenAI
```rust
use omni_agent::llm::providers::openai::OpenAIProvider;
let provider = OpenAIProvider::new("api-key".to_string(), Some("gpt-3.5-turbo".to_string()));
```

### Claude
```rust
use omni_agent::llm::providers::claude::ClaudeProvider;
let provider = ClaudeProvider::new("api-key".to_string(), Some("claude-3-haiku".to_string()));
```

### Google Gemini
```rust
use omni_agent::llm::providers::google::GoogleProvider;
let provider = GoogleProvider::new("api-key".to_string(), Some("gemini-pro".to_string()));
```

## Common Development Tasks

### Adding New LLM Provider
1. Create provider in `src/llm/providers/[provider].rs`
2. Add config struct to `ProviderConfig`
3. Update `LLMManager` to handle new provider
4. Add tests in appropriate test file

### Extending A2A Protocol
1. Add new message types to `src/protocol/message.rs`
2. Update handlers in `src/server/mod.rs`
3. Add appropriate tests

### Configuration Changes
1. Update `AppConfig` struct in `src/config.rs`
2. Add environment variable override in `override_with_env()`
3. Update example config files

## Testing Strategy
- **Unit Tests**: Individual component testing
- **Integration Tests**: Provider-specific tests with mock servers
- **End-to-End Tests**: Full workflow testing
- **Mock Mode**: All tests can run without real API keys
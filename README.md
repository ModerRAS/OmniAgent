# OmniAgent - A2A + MCP Protocol Agent in Rust

A comprehensive Rust implementation of an Agent that supports both the **Agent-to-Agent (A2A)** protocol and **Model Context Protocol (MCP)** for seamless agent communication and tool integration.

## 🚀 Features

- **A2A Protocol Support**: Full implementation of the Agent-to-Agent protocol
- **MCP Client**: Connect to multiple MCP servers for tool capabilities
- **Concurrent Connections**: Async support for multiple MCP and A2A endpoints
- **State Machine**: Robust message processing with state management
- **Mock LLM**: Built-in mock LLM for testing and development
- **RESTful API**: HTTP endpoints for A2A communication
- **Builder Pattern**: Clean configuration with `AgentBuilder`
- **Comprehensive Testing**: Unit and integration tests included

## 📋 Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   A2A Server    │    │   OmniAgent     │    │   MCP Clients   │
│   (External)    │◄──►│   (This App)    │◄──►│   (External)    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                │
                       ┌─────────────────┐
                       │   Mock LLM      │
                       │   (Internal)    │
                       └─────────────────┘
```

## 🛠️ Quick Start

### Prerequisites

- Rust 1.70+ (for async support)
- Cargo

### Installation

1. Clone the repository:
```bash
git clone <your-repo-url>
cd omni-agent
```

2. Build the project:
```bash
cargo build --release
```

3. Run the A2A server:
```bash
cargo run --release
# or
PORT=8080 cargo run --release
```

## 📖 Usage

### Running the A2A Server

```bash
# Basic usage
cargo run

# Custom port
cargo run -- --port 3000

# With debug logging
RUST_LOG=debug cargo run
```

### Using the Agent Builder

```rust
use omni_agent::AgentBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let agent = AgentBuilder::new("my-agent", "A helpful AI agent")
        .version("1.0.0")
        .add_mcp("weather", "http://localhost:8081")
        .add_a2a("assistant", "http://localhost:8082")
        .build()
        .await?;

    println!("Agent capabilities: {:?}", agent.get_capabilities().await);
    Ok(())
}
```

### HTTP API Endpoints

Once running, the server provides these endpoints:

- `GET /` - Server info and available endpoints
- `GET /health` - Health check
- `GET /manifest` - Agent capabilities and metadata
- `POST /messages` - Send a message to the agent
- `GET /messages/:id` - Retrieve a specific message

### Example API Usage

```bash
# Get agent capabilities
curl http://localhost:8080/manifest

# Send a message
curl -X POST http://localhost:8080/messages \
  -H "Content-Type: application/json" \
  -d '{
    "sender": "user",
    "recipient": "omni-agent",
    "content": {
      "type": "Text",
      "text": "What can you do?"
    }
  }'
```

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test integration_test

# Run specific test
cargo test test_agent_integration
```

## 🔧 Configuration

### Environment Variables

- `PORT` - Server port (default: 8080)
- `RUST_LOG` - Logging level (debug, info, warn, error)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
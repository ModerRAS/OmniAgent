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

## 🏗️ Development

### Project Structure
```
omni-agent/
├── src/
│   ├── agent/          # Core agent implementation
│   │   ├── builder.rs  # AgentBuilder pattern
│   │   └── state.rs    # State machine
│   ├── mcp/            # MCP protocol client
│   ├── a2a/            # A2A protocol client
│   ├── protocol/       # Shared message types
│   ├── server/         # HTTP server
│   ├── llm/            # Mock LLM integration
│   └── main.rs         # Application entry
├── tests/              # Integration tests
├── .github/            # GitHub Actions workflows
├── Dockerfile          # Container configuration
└── README.md           # This file
```

### Adding New Features

1. **New MCP Tool Support**: Extend `src/mcp/client.rs`
2. **New A2A Message Type**: Add to `src/protocol/message.rs`
3. **New State Handling**: Update `src/agent/state.rs`

## 🚀 Deployment Options

### Docker
```bash
# Build and run
docker build -t omni-agent .
docker run -p 8080:8080 omni-agent

# With custom port
docker run -p 3000:8080 -e PORT=8080 omni-agent
```

### Docker Compose
```yaml
version: '3.8'
services:
  omni-agent:
    build: .
    ports:
      - "8080:8080"
    environment:
      - PORT=8080
      - RUST_LOG=info
    restart: unless-stopped
```

### Kubernetes
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: omni-agent
spec:
  replicas: 3
  selector:
    matchLabels:
      app: omni-agent
  template:
    metadata:
      labels:
        app: omni-agent
    spec:
      containers:
      - name: omni-agent
        image: ghcr.io/your-org/omni-agent:latest
        ports:
        - containerPort: 8080
        env:
        - name: PORT
          value: "8080"
        - name: RUST_LOG
          value: "info"
```

## 📊 Monitoring & Observability

### Health Checks
- `GET /health` - Basic health check
- `GET /manifest` - Agent capabilities

### Logging
Configure logging levels:
```bash
RUST_LOG=debug cargo run
RUST_LOG=omni_agent=debug,hyper=info cargo run
```

### Metrics (Future)
- Request/response timing
- Error rates
- Connection status
- Queue sizes

## 🔍 Troubleshooting

### Common Issues

1. **Port Already in Use**
   ```bash
   # Check what's using the port
   lsof -i :8080
   # Use different port
   PORT=3000 cargo run
   ```

2. **Build Failures**
   ```bash
   # Clean build
   cargo clean
   cargo build --release
   ```

3. **Test Failures**
   ```bash
   # Check test output
   cargo test -- --nocapture
   # Run specific test
   cargo test test_agent_integration
   ```

4. **Docker Issues**
   ```bash
   # Debug Docker build
   docker build --no-cache .
   # Check running containers
   docker ps -a
   ```

### Debug Mode
Enable detailed logging:
```bash
RUST_LOG=debug cargo run
```

## 🤝 Contributing

1. **Fork the repository**
2. **Create feature branch**: `git checkout -b feature/amazing-feature`
3. **Add tests** for new functionality
4. **Run tests**: `cargo test`
5. **Format code**: `cargo fmt`
6. **Lint code**: `cargo clippy`
7. **Commit changes**: `git commit -m 'feat: add amazing feature'`
8. **Push to branch**: `git push origin feature/amazing-feature`
9. **Open Pull Request**

### Development Setup
```bash
# Install development tools
cargo install cargo-audit cargo-watch

# Watch for changes during development
cargo watch -x check -x test
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 References

- [A2A Protocol Specification](https://github.com/EmilLindfors/a2a-rs)
- [MCP Protocol Documentation](https://modelcontextprotocol.io/)
- [Rust Async Book](https://rust-lang.github.io/async-book/)
- [Axum Framework](https://docs.rs/axum/latest/axum/)

## 📞 Support

For issues and questions:
1. Check the [Issues](https://github.com/your-repo/issues) page
2. Review the [Discussions](https://github.com/your-repo/discussions)
3. Create a new issue with:
   - Detailed description
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details
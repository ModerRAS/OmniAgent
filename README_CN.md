# OmniAgent - Rust A2A + MCP 智能体框架

一个全面的 Rust 框架，用于构建支持 A2A（智能体到智能体）和 MCP（模型上下文协议）协议的多提供商 LLM 智能体。

## 特性

- **A2A 协议支持**: 通过 HTTP REST API 实现完整的智能体间通信
- **MCP 协议支持**: 模型上下文协议用于工具集成
- **多提供商 LLM 支持**:
  - OpenAI (GPT-3.5, GPT-4 等)
  - Anthropic Claude (3.5 Sonnet, 3 Haiku 等)
  - Google Gemini (Pro, Pro Vision 等)
- **模拟模式**: 开发和测试无需 API 密钥
- **异步/等待**: 基于 tokio 运行时构建
- **全面测试**: 所有提供商的模拟服务器

## 快速开始

### 1. 安装

```bash
git clone https://github.com/your-username/omni-agent
cd omni-agent
cargo build --release
```

### 2. 配置设置

在项目根目录创建 `config.json` 文件（首次运行时会自动生成）：

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
        "name": "文件工具",
        "url": "http://localhost:3000"
      }
    ]
  },
  "a2a": {
    "agents": [
      {
        "name": "天气智能体",
        "url": "http://localhost:8081"
      }
    ]
  }
}
```

设置 API 密钥作为环境变量：

```bash
# 实际 LLM 使用所需的密钥（选择一个或多个）
export GOOGLE_API_KEY="你的谷歌API密钥"
export OPENAI_API_KEY="你的OpenAI API密钥"
export ANTHROPIC_API_KEY="你的Anthropic API密钥"

# 可选：开发使用模拟模式
export USE_MOCK=true
```

### 3. 启动智能体服务器

```bash
# 使用默认配置运行
cargo run

# 使用自定义配置运行
cargo run -- --config custom.json

# 模拟模式运行（无需API密钥）
USE_MOCK=true cargo run
```

### 4. 测试服务器

服务器运行后，可以用 curl 测试：

```bash
# 检查服务器健康状态
curl http://localhost:8080/health

# 获取智能体清单
curl http://localhost:8080/manifest

# 向智能体发送消息
curl -X POST http://localhost:8080/messages \
  -H "Content-Type: application/json" \
  -d '{
    "content": {
      "type": "text",
      "text": "你好，能帮助我吗？"
    },
    "metadata": {}
  }'
```

## 使用示例

### 基础智能体创建

```rust
use omni_agent::AgentBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建简单智能体
    let agent = AgentBuilder::new("我的智能体", "一个有帮助的助手")
        .version("1.0.0")
        .build()
        .await?;

    // 使用智能体
    let response = agent
        .llm
        .write()
        .await
        .process_message("你好，你能怎么帮助我？", &[])
        .await?;

    println!("智能体回应: {}", response.content);
    Ok(())
}
```

### 支持 MCP 和 A2A 的智能体

```rust
use omni_agent::AgentBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let agent = AgentBuilder::new("高级智能体", "支持工具的智能体")
        .version("1.0.0")
        .add_mcp("文件工具", "http://localhost:3000")
        .add_a2a("天气智能体", "http://localhost:8081")
        .build()
        .await?;

    println!("智能体创建完成，包含 {} 个 MCP 客户端和 {} 个 A2A 客户端", 
             agent.mcp_clients.len(), 
             agent.a2a_clients.len());

    Ok(())
}
```

### 直接使用 LLM 提供商

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
        "你的谷歌API密钥".to_string(),
        Some("gemini-pro".to_string())
    );

    let request = LLMRequest {
        messages: vec![
            Message {
                role: MessageRole::User,
                content: "用简单术语解释量子计算".to_string(),
            }
        ],
        model: "gemini-pro".to_string(),
        temperature: Some(0.7),
        max_tokens: Some(200),
        stream: Some(false),
    };

    let response = provider.chat(request).await?;
    println!("回应: {}", response.content);

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
        "你的Anthropic API密钥".to_string(),
        Some("claude-3-haiku-20240307".to_string())
    );

    let request = LLMRequest {
        messages: vec![
            Message {
                role: MessageRole::System,
                content: "你是一个有帮助的编程助手".to_string(),
            },
            Message {
                role: MessageRole::User,
                content: "写一个反转字符串的Rust函数".to_string(),
            }
        ],
        model: "claude-3-haiku-20240307".to_string(),
        temperature: Some(0.5),
        max_tokens: Some(150),
        stream: Some(false),
    };

    let response = provider.chat(request).await?;
    println!("回应: {}", response.content);

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
        "你的OpenAI API密钥".to_string(),
        Some("gpt-3.5-turbo".to_string())
    );

    let request = LLMRequest {
        messages: vec![
            Message {
                role: MessageRole::User,
                content: "今天天气怎么样？".to_string(),
            }
        ],
        model: "gpt-3.5-turbo".to_string(),
        temperature: Some(0.7),
        max_tokens: Some(100),
        stream: Some(false),
    };

    let response = provider.chat(request).await?;
    println!("回应: {}", response.content);

    Ok(())
}
```

## 测试

### 运行所有测试
```bash
cargo test
```

### 运行特定提供商测试
```bash
# Google Gemini 测试
cargo test --test google_integration_test -- --nocapture

# Claude 测试
cargo test --test claude_integration_test -- --nocapture

# 端到端测试（带模拟服务器）
cargo test --test end_to_end_test -- --nocapture
```

### 模拟模式下运行测试
```bash
# 无需真实API调用运行所有测试
USE_MOCK=true cargo test
```

## 配置

### 环境变量

| 变量名 | 描述 | 默认值 |
|--------|------|--------|
| `OPENAI_API_KEY` | OpenAI API 密钥 | - |
| `ANTHROPIC_API_KEY` | Claude API 密钥 | - |
| `GOOGLE_API_KEY` | Google AI API 密钥 | - |
| `PORT` | 服务器端口 | 8080 |
| `HOST` | 服务器主机 | 127.0.0.1 |
| `USE_MOCK` | 启用模拟模式 | false |
| `LOG_LEVEL` | 日志级别 (debug, info, warn, error) | info |
| `OMNI_AGENT_CONFIG` | 自定义配置文件路径 | config.json |

### 配置文件 (config.json)

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
        "name": "文件服务器",
        "url": "http://localhost:3000"
      }
    ]
  },
  "a2a": {
    "agents": [
      {
        "name": "天气智能体",
        "url": "http://localhost:8081"
      }
    ]
  }
}
```

## API 端点

### 健康检查
```http
GET /health
```

### 智能体清单
```http
GET /manifest
```

### 发送消息
```http
POST /messages
Content-Type: application/json

{
  "content": {
    "type": "text",
    "text": "你的消息内容"
  },
  "metadata": {}
}
```

### 获取消息状态
```http
GET /messages/{message_id}
```

## 开发

### 项目结构

```
src/
├── agent/         # 智能体实现和构建器
├── a2a/           # A2A 协议客户端和服务器
├── llm/           # LLM 提供商和服务层
├── mcp/           # MCP 协议客户端
├── protocol/      # 共享协议定义
├── server/        # HTTP 服务器实现
├── config.rs      # 配置管理
├── app.rs         # 主应用程序协调器
└── main.rs        # 入口点
```

### 添加新的 LLM 提供商

1. 在 `src/llm/providers/[provider].rs` 中创建提供商
2. 将配置结构添加到 `ProviderConfig`
3. 更新 `LLMManager` 以处理新提供商
4. 在相应的测试文件中添加测试

### 运行开发服务器

```bash
# 开发监视模式
cargo watch -x run

# 使用自定义配置
cargo run -- --config dev.json

# 带日志运行
cargo run -- --config dev.json 2>&1 | tee agent.log
```

## 故障排除

### 常见问题

**问：智能体无法启动，显示 "API密钥未找到"**
答：设置相应的环境变量或启用模拟模式：
```bash
export USE_MOCK=true
```

**问：测试因网络错误失败**
答：在模拟模式下运行测试：
```bash
USE_MOCK=true cargo test
```

**问：如何使用不同的 LLM 提供商？**
答：更新 `config.json` 中的 `llm.provider` 字段：
```json
{
  "llm": {
    "provider": "claude",
    "model": "claude-3-haiku-20240307"
  }
}
```

**问：服务器无法在端口 8080 启动**
答：在配置中更改端口或使用环境变量：
```bash
export PORT=8081
cargo run
```

## 示例

查看 `examples/` 目录获取更多详细使用示例：

- `examples/basic_agent.rs` - 简单智能体创建
- `examples/with_mcp.rs` - 支持 MCP 工具的智能体
- `examples/with_a2a.rs` - 支持 A2A 通信的智能体
- `examples/custom_config.rs` - 自定义配置使用

## 致谢

- Google Gemini 食谱 API 参考
- Anthropic Claude API 文档
- OpenAI API 文档
- MCP 协议规范
- A2A 协议规范
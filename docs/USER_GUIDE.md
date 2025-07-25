# OmniAgent 用户指南

## 简介

OmniAgent 是一个完整的可配置应用程序，能够连接多个 LLM 提供商、MCP 服务器，并同时作为 A2A 服务器对外提供服务。

## 架构概览

有关 OmniAgent 的架构信息，请参阅以下文档：

- [当前架构](current_architecture.md) - 现有的实现架构
- [理想架构](ideal_architecture.md) - 未来的发展方向
- [架构对比](architecture_comparison.md) - 当前与理想架构的差异分析

## 🚀 快速开始

### 1. 环境准备

```bash
# 克隆项目
git clone https://github.com/your-username/omni-agent.git
cd omni-agent

# 安装依赖
cargo build --release
```

### 2. 配置 LLM 提供商

#### 使用 Claude
```bash
# 设置环境变量
export ANTHROPIC_API_KEY="your-anthropic-api-key"
```

#### 使用 OpenAI
```bash
# 设置环境变量
export OPENAI_API_KEY="your-openai-api-key"
```

#### 使用 Google Gemini
```bash
# 设置环境变量
export GOOGLE_API_KEY="your-google-api-key"
```

### 3. 配置文件

复制示例配置文件：

```bash
cp config.example.json config.json
```

编辑配置文件：

```json
{
  "llm": {
    "provider": "claude",
    "api_key": "your-anthropic-api-key",
    "model": "claude-3-haiku-20240307",
    "use_mock": false
  }
}
```

### 4. 启动应用

```bash
# 直接运行
cargo run --release

# 或使用配置文件
OMNI_AGENT_CONFIG=my-config.json cargo run --release
```

## 📋 完整配置示例

### 连接多个 MCP 服务器

```json
{
  "mcp": {
    "enabled": true,
    "servers": {
      "weather": {
        "name": "Weather Service",
        "url": "http://localhost:8081",
        "enabled": true
      },
      "calculator": {
        "name": "Calculator Service",
        "url": "http://localhost:8082",
        "enabled": true
      }
    }
  }
}
```

### 连接多个 A2A 服务器

```json
{
  "a2a": {
    "enabled": true,
    "servers": {
      "assistant1": {
        "name": "Research Assistant",
        "url": "http://localhost:8083",
        "auth_token": "optional-token",
        "enabled": true
      },
      "assistant2": {
        "name": "Code Assistant",
        "url": "http://localhost:8084",
        "enabled": true
      }
    }
  }
}
```

### 完整配置示例

```json
{
  "server": {
    "port": 8080,
    "host": "0.0.0.0"
  },
  "llm": {
    "provider": "claude",
    "api_key": "your-key",
    "model": "claude-3-haiku-20240307",
    "temperature": 0.7,
    "max_tokens": 1000,
    "use_mock": false
  },
  "mcp": {
    "enabled": true,
    "servers": {
      "weather": {
        "name": "Weather MCP",
        "url": "http://localhost:8081",
        "timeout": 30,
        "enabled": true
      }
    }
  },
  "a2a": {
    "enabled": true,
    "servers": {
      "assistant": {
        "name": "External Agent",
        "url": "http://localhost:8083",
        "timeout": 30,
        "enabled": true
      }
    }
  }
}
```

## 🔧 环境变量配置

| 变量名 | 描述 | 示例 |
|--------|------|------|
| `ANTHROPIC_API_KEY` | Claude API 密钥 | `sk-ant-...` |
| `OPENAI_API_KEY` | OpenAI API 密钥 | `sk-...` |
| `GOOGLE_API_KEY` | Google AI API 密钥 | `AIza...` |
| `PORT` | 服务器端口 | `8080` |
| `OMNI_AGENT_CONFIG` | 配置文件路径 | `config.json` |
| `USE_MOCK` | 使用 mock 模式 | `true` 或 `false` |
| `LOG_LEVEL` | 日志级别 | `info`, `debug`, `error` |

## 🌐 使用应用

### 1. 启动后访问

应用启动后，可以通过以下方式访问：

- **Web 界面**: http://localhost:8080
- **健康检查**: http://localhost:8080/health
- **能力清单**: http://localhost:8080/manifest
- **发送消息**: POST http://localhost:8080/messages

### 2. 发送消息示例

```bash
# 发送简单消息
curl -X POST http://localhost:8080/messages \
  -H "Content-Type: application/json" \
  -d '{
    "sender": "user",
    "recipient": "OmniAgent",
    "content": {
      "type": "text",
      "text": "Hello, what can you do?"
    }
  }'
```

### 3. 检查状态

```bash
# 查看整体状态
curl http://localhost:8080/health

# 查看能力清单
curl http://localhost:8080/manifest
```

## 🔄 动态配置

### 运行时添加 MCP 服务器

通过配置文件，可以在不重启应用的情况下添加新的 MCP 服务器。

### 运行时切换 LLM 提供商

通过修改配置文件并重启应用，可以切换 LLM 提供商。

## 🐛 调试模式

### 使用 Mock 模式

```json
{
  "llm": {
    "use_mock": true
  }
}
```

### 详细日志

```bash
LOG_LEVEL=debug cargo run --release
```

## 📊 监控和日志

### 日志格式

支持 JSON 格式日志：

```json
{
  "timestamp": "2024-12-20T10:30:00Z",
  "level": "INFO",
  "message": "Connected to MCP server",
  "server": "weather"
}
```

### 健康检查

应用提供健康检查端点：

```bash
curl http://localhost:8080/health
```

返回示例：

```json
{
  "status": "healthy",
  "timestamp": "2024-12-20T10:30:00Z",
  "services": {
    "llm": {
      "provider": "claude",
      "connected": true,
      "mock_mode": false
    },
    "mcp": {
      "connected_servers": 2,
      "total_servers": 2
    },
    "a2a": {
      "connected_servers": 1,
      "total_servers": 1,
      "port": 8080
    }
  }
}
```

## 🔍 故障排除

### 常见问题

1. **API 密钥无效**
   - 检查环境变量是否正确设置
   - 验证 API 密钥是否有效

2. **MCP 服务器连接失败**
   - 检查服务器 URL 是否正确
   - 验证服务器是否运行
   - 检查网络连接

3. **A2A 服务器连接失败**
   - 检查服务器 URL 是否正确
   - 验证认证令牌（如果需要）
   - 检查服务器是否运行

4. **端口被占用**
   - 修改配置文件中的端口
   - 或使用环境变量 `PORT=8081 cargo run`

### 调试信息

使用详细日志模式获取更多信息：

```bash
LOG_LEVEL=debug cargo run --release
```

## 🎯 使用场景

### 个人 AI 助手
- 配置 Claude 作为 LLM
- 连接天气、计算器 MCP 服务器
- 作为 A2A 服务器供其他应用调用

### 企业级部署
- 配置多个 LLM 提供商
- 连接内部 MCP 服务器
- 作为 A2A 网关协调多个代理

### 开发测试
- 使用 Mock 模式进行开发
- 连接本地 MCP 服务器
- 快速原型验证
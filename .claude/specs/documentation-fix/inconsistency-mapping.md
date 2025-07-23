# OmniAgent 文档不一致性映射报告

## 1. 配置结构不匹配

### 实际结构 vs 文档示例

**实际 AppConfig 结构：**
```rust
pub struct AppConfig {
    pub server: ServerConfig,
    pub llm: LLMConfig,
    pub mcp: McpConfig,
    pub a2a: A2AConfig,
    pub logging: LoggingConfig,
}

pub struct McpConfig {
    pub servers: HashMap<String, McpServerConfig>,
    pub enabled: bool,
}

pub struct McpServerConfig {
    pub name: String,
    pub description: String,
    pub url: String,
    pub timeout: u64,
    pub retry_attempts: u32,
    pub enabled: bool,
}

pub struct A2AConfig {
    pub servers: HashMap<String, A2AServerConfig>,
    pub enabled: bool,
    pub allow_external: bool,
}

pub struct A2AServerConfig {
    pub name: String,
    pub description: String,
    pub url: String,
    pub auth_token: Option<String>,
    pub timeout: u64,
    pub enabled: bool,
}
```

**文档中的错误示例：**
```json
{
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

## 2. API端点不匹配

### 实际端点 vs 文档端点

**实际 A2A 端点：**
- `GET /health` - 健康检查
- `GET /manifest` - 智能体能力清单
- `GET /agent.json` - 智能体卡片(A2A规范)
- `POST /messages` - 发送消息
- `GET /messages/:id` - 获取消息

**实际主应用端点：**
- `GET /health` - 健康检查
- `GET /info` - 智能体信息
- `POST /chat` - 聊天接口

**文档中的错误端点：**
- 文档显示 `/messages` 但缺少 `/agent.json`
- 文档显示 `/messages/:id` 但实际主应用使用 `/chat`

## 3. 环境变量不匹配

### 实际支持的环境变量
- `OPENAI_API_KEY`
- `ANTHROPIC_API_KEY`
- `GOOGLE_API_KEY`
- `PORT`
- `USE_MOCK`
- `LOG_LEVEL`
- `OMNI_AGENT_CONFIG`

### 文档中缺失的环境变量
- 缺少 `ANTHROPIC_API_KEY`
- 缺少 `OMNI_AGENT_CONFIG`

## 4. MessageContent 结构不匹配

### 实际 MessageContent 枚举
```rust
#[serde(tag = "type")]
pub enum MessageContent {
    Text { text: String },
    ToolCall { tool: String, parameters: serde_json::Value },
    ToolResult { tool: String, result: serde_json::Value },
    AgentRequest { request_type: String, payload: serde_json::Value },
    Error { code: String, message: String },
}
```

### 文档中的错误格式
```json
{
  "content": {
    "type": "text",
    "text": "Hello, can you help me?"
  }
}
```

## 5. LLM 提供商初始化不匹配

### 实际初始化方式
```rust
// 通过配置自动选择
let config = AppConfig::load_from_file("config.json")?;
let agent = Agent::new(config);
```

### 文档中的错误示例
```rust
// 手动初始化提供商（已过时）
let provider = GoogleProvider::new("api-key".to_string(), Some("model".to_string()));
```

## 6. MCP 配置格式问题

### 标准 MCP 格式
```json
{
  "mcpServers": {
    "server-name": {
      "command": "uvx",
      "args": ["mcp-server-name"],
      "env": {},
      "disabled": false,
      "autoApprove": []
    }
  }
}
```

### 当前非标准格式
```json
{
  "mcp": {
    "servers": {
      "weather": {
        "name": "Weather MCP Server",
        "description": "Provides weather information",
        "url": "http://localhost:8081",
        "timeout": 30,
        "retry_attempts": 3,
        "enabled": true
      }
    }
  }
}
```

## 7. 命名约定不一致

### 项目命名
- **正确**: OmniAgent (标题), omni-agent (代码)
- **实际使用**: 一致

### 模块路径
- **实际结构**: `src/server/`, `src/llm/`, `src/mcp/`, `src/a2a/`
- **文档描述**: 匹配

### 配置字段
- **实际字段**: 使用 snake_case (port, host, api_key)
- **文档示例**: 匹配

## 8. 关键修复清单

### 高优先级修复
1. **配置示例修复**: 更新所有配置示例以匹配实际结构
2. **API端点文档**: 修正端点路径和请求格式
3. **环境变量**: 补充缺失的环境变量文档
4. **MessageContent**: 修正消息格式示例
5. **MCP格式**: 考虑支持标准MCP格式

### 中优先级修复
1. **代码示例**: 更新过时的初始化方式
2. **测试命令**: 验证测试命令的准确性
3. **项目结构**: 确认目录结构描述

### 低优先级修复
1. **格式统一**: 统一代码块格式和语法高亮
2. **链接验证**: 检查所有内部链接
3. **示例验证**: 确保所有示例可运行
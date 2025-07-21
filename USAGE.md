# 智能体应用使用指南

## 快速开始

### 1. 启动应用

```bash
# 默认启动（模拟模式）
cargo run

# 使用自定义配置
cargo run -- --config my-config.json

# 指定端口
cargo run -- --port 8081

# 启用模拟模式
cargo run -- --mock

# 查看帮助
cargo run -- --help
```

### 2. 测试应用

启动后，应用会自动在 http://localhost:8080 运行。

#### 健康检查
```bash
curl http://localhost:8080/health
```

#### 获取智能体信息
```bash
curl http://localhost:8080/info
```

#### 发送消息
```bash
curl -X POST http://localhost:8080/chat \
  -H "Content-Type: application/json" \
  -d '{
    "message": "今天天气怎么样？",
    "context": {}
  }'
```

### 3. 示例对话

#### 文件操作请求
```bash
curl -X POST http://localhost:8080/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "请帮我读取一个文件"}'
# 预期：使用 MCP 文件工具
```

#### 天气查询
```bash
curl -X POST http://localhost:8080/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "北京今天的天气如何？"}'
# 预期：使用天气智能体
```

#### 通用问题
```bash
curl -X POST http://localhost:8080/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "解释一下量子计算"}'
# 预期：使用本地 LLM
```

### 4. 配置说明

配置文件 `config.json` 包含：

- **server**: 服务器配置（端口、主机）
- **llm**: LLM配置（提供商、模型、模拟模式）
- **mcp**: MCP工具配置
- **a2a**: A2A智能体配置

### 5. 环境变量

- `GOOGLE_API_KEY`: Google API密钥
- `OPENAI_API_KEY`: OpenAI API密钥
- `ANTHROPIC_API_KEY`: Claude API密钥
- `PORT`: 服务器端口
- `USE_MOCK`: 启用模拟模式
- `LOG_LEVEL`: 日志级别

### 6. 日志输出示例

应用启动时会显示：
```
🚀 启动智能体应用...
📋 配置加载完成:
   LLM 提供商: google
   模型: gemini-pro
   MCP 服务器: 2
   A2A 智能体: 2
   模拟模式: true
✅ 智能体创建完成
🌐 服务器启动于 http://127.0.0.1:8080
```

处理消息时会显示：
```
🔍 分析用户消息: 今天天气怎么样？
🤝 使用 A2A 智能体: 天气智能体
```
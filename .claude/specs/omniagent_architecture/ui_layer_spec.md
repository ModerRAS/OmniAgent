# OmniAgent 用户界面层详细规格

## 概述

用户界面层是OmniAgent与外部系统和用户交互的入口点。该层提供多种接口选项，包括REST API、WebSocket和CLI，以支持不同的使用场景和集成需求。

## REST API 接口规格

### 基础端点

#### GET /health
**描述**: 检查服务健康状态
**响应**:
```json
{
  "status": "healthy",
  "timestamp": "2023-10-01T12:00:00Z"
}
```

#### GET /info
**描述**: 获取智能体基本信息
**响应**:
```json
{
  "name": "OmniAgent",
  "description": "全能智能体助手",  
  "version": "1.0.0",
  "capabilities": ["text_processing", "llm_integration"],
  "llm_provider": "google",
  "llm_model": "gemini-pro"
}
```

#### POST /chat
**描述**: 处理用户聊天请求
**请求体**:
```json
{
  "message": "你好，能帮我查询天气吗？",
  "context": {
    "user_id": "user123",
    "session_id": "session456"
  }
}
```
**响应**:
```json
{
  "message": "我已经理解您的请求，正在为您查询天气信息。",
  "source": "local_llm",
  "details": {}
}
```

#### POST /buffer
**描述**: 管理对话缓冲区
**请求体**:
```json
{
  "action": "add",
  "message": "用户询问了关于天气的问题",
  "type": "user_message"
}
```
**响应**:
```json
{
  "status": "success",
  "buffer_size": 5
}
```

### 高级端点

#### GET /capabilities
**描述**: 获取可用能力列表
**响应**:
```json
{
  "capabilities": [
    {
      "id": "text_processing",
      "name": "文本处理",
      "description": "处理和响应文本消息"
    }
  ]
}
```

#### POST /workflows
**描述**: 执行复杂工作流
**请求体**:
```json
{
  "workflow_id": "weather_search",
  "parameters": {
    "location": "北京"
  }
}
```

#### GET /status
**描述**: 获取实时状态更新
**响应**:
```json
{
  "status": "processing",
  "current_task": "llm_generation",
  "progress": 0.75
}
```

## WebSocket API 规格（未来实现）

### 连接
**端点**: `ws://localhost:8080/ws`

### 消息格式
```json
{
  "type": "message",
  "id": "uuid",
  "payload": {
    "content": "消息内容",
    "timestamp": "2023-10-01T12:00:00Z"
  }
}
```

### 支持的消息类型
- `message`: 普通消息
- `status_update`: 状态更新  
- `notification`: 系统通知
- `error`: 错误信息

## CLI 接口规格（未来实现）

### 基础命令
```bash
# 启动服务
omni-agent start

# 健康检查
omni-agent health

# 发送消息
omni-agent chat "你好，世界！"

# 查看信息
omni-agent info
```

### 配置选项
```bash
# 指定配置文件
omni-agent --config custom.json

# 设置端口
omni-agent --port 8080

# 启用模拟模式
omni-agent --mock
```

## 对话双重缓冲机制

### 缓冲区管理
对话缓冲区采用先进先出（FIFO）策略，自动管理对话历史，确保上下文连贯性。

#### 缓冲区操作
1. **添加消息**: 新消息自动添加到缓冲区尾部
2. **上下文注入**: 系统自动将缓冲区内容注入到下一次请求的上下文中
3. **智能过滤**: 自动过滤不相关的旧消息以保持上下文相关性
4. **大小管理**: 缓冲区大小可配置，默认保留最近10条消息

### 数据结构
```rust
pub struct ConversationBuffer {
    messages: VecDeque<BufferedMessage>,
    max_size: usize,
    current_size: AtomicUsize,
}

pub struct BufferedMessage {
    pub id: Uuid,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub message_type: MessageType,
    pub context_relevance: f32,
}
```

## 中间件和安全

### 认证中间件
- JWT令牌验证
- API密钥验证
- 会话管理

### 日志中间件
- 请求/响应日志记录
- 性能监控
- 错误追踪

### 限流中间件
- 请求频率限制
- 并发连接控制
- 资源配额管理

## 错误处理

### 标准错误响应格式
```json
{
  "error": {
    "code": "INVALID_REQUEST",
    "message": "请求参数无效",
    "details": {
      "field": "message",
      "reason": "不能为空"
    }
  }
}
```

### 错误码
- `INVALID_REQUEST`: 请求参数无效
- `AUTHENTICATION_FAILED`: 认证失败
- `INTERNAL_ERROR`: 内部服务器错误
- `SERVICE_UNAVAILABLE`: 服务不可用

## 性能要求

### 响应时间
- 95%的请求响应时间 < 500ms
- 99%的请求响应时间 < 1000ms

### 并发处理
- 支持至少1000个并发连接
- 支持每秒100个请求处理能力

### 内存使用
- 单个请求处理内存占用 < 10MB
- 缓冲区总内存占用 < 100MB

## 部署要求

### 环境变量
```bash
OMNI_AGENT_PORT=8080
OMNI_AGENT_HOST=0.0.0.0
OMNI_AGENT_LOG_LEVEL=info
```

### 配置文件
```json
{
  "server": {
    "port": 8080,
    "host": "0.0.0.0",
    "cors_origins": ["*"]
  }
}
```

## 监控和指标

### 健康检查端点
- `/health`: 基本健康检查
- `/metrics`: Prometheus指标

### 日志格式
```json
{
  "timestamp": "2023-10-01T12:00:00Z",
  "level": "info",
  "target": "omni_agent::ui",
  "message": "处理用户请求",
  "fields": {
    "user_id": "user123",
    "request_id": "req456"
  }
}
```

## 测试要求

### 单元测试
- 每个端点的功能测试
- 错误处理测试
- 边界条件测试

### 集成测试
- 端到端API测试
- 性能基准测试
- 安全测试

### 负载测试
- 并发用户测试
- 长时间运行测试
- 资源使用监控
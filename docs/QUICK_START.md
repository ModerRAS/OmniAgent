# 🚀 OmniAgent 快速开始

项目已经完成！这是一个完整的可配置多代理应用程序。

## 立即开始

```bash
# 1. 初始化项目
./scripts/init.sh

# 2. 编辑配置
vim config.json

# 3. 运行应用
cargo run --release

# 4. 测试
open http://localhost:8080/health
```

## 核心功能

- ✅ **LLM集成**: Claude, OpenAI, Google Gemini
- ✅ **MCP服务器连接**: 可配置多个MCP工具服务器
- ✅ **A2A服务器**: 对外提供HTTP REST API服务
- ✅ **A2A客户端**: 连接其他A2A代理服务器
- ✅ **完整配置系统**: JSON文件 + 环境变量

## 项目结构

```
omni-agent/
├── src/
│   ├── app.rs           # 主应用程序
│   ├── config.rs        # 配置系统
│   ├── llm/             # LLM服务
│   ├── mcp/             # MCP客户端
│   ├── a2a/             # A2A客户端
│   └── server/          # HTTP服务器
├── config.json          # 运行时配置
├── config.example.json  # 配置示例
├── docs/                # 完整文档
└── scripts/init.sh      # 初始化脚本
```

## 配置示例

编辑 `config.json`:
```json
{
  "llm": {
    "provider": "claude",
    "api_key": "your-key",
    "model": "claude-3-haiku-20240307",
    "use_mock": true
  }
}
```

## API端点

- `GET /health` - 健康检查
- `GET /manifest` - 能力清单
- `POST /messages` - 发送消息

项目已推送到GitHub，可直接使用！
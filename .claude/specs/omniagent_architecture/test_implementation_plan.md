# OmniAgent 测试 implementation plan

## 1. 单元测试 implementation plan

### 1.1 核心路由器单元测试 (src/core/router/mod.rs)
- [x] 创建 router_tests.rs 测试文件
- [ ] 实现路由决策逻辑测试
- [ ] 添加不同类型消息的路由测试
- [ ] 实现边界条件和错误情况测试

### 1.2 状态管理器单元测试 (src/core/state/mod.rs)
- [x] 创建 state_tests.rs 测试文件
- [ ] 实现对话缓冲区的添加、获取和清空功能测试
- [ ] 添加缓冲区满时的行为测试
- [ ] 实现并发访问测试
- [ ] 添加缓冲消息的正确性验证

### 1.3 能力管理器单元测试 (src/core/capabilities/mod.rs)
- [x] 创建 capabilities_tests.rs 测试文件
- [ ] 实现能力注册和获取功能测试
- [ ] 添加能力信息的正确性验证
- [ ] 实现并发注册测试

### 1.4 LLM服务单元测试 (src/services/llm/mod.rs)
- [x] 创建 llm_tests.rs 测试文件
- [ ] 实现令牌计算准确性测试
- [ ] 添加缓存功能测试（命中、未命中、过期）
- [ ] 实现模拟模式和实际模式切换测试
- [ ] 添加错误处理测试

### 1.5 内存服务单元测试 (src/services/memory/mod.rs)
- [x] 创建 memory_tests.rs 测试文件
- [ ] 实现上下文压缩功能测试
- [ ] 添加压缩质量评估测试
- [ ] 实现边界条件测试

### 1.6 工具执行引擎单元测试 (src/services/tools/enhanced_engine.rs)
- [x] 创建 tools_tests.rs 测试文件
- [ ] 实现8阶段生命周期的每个阶段测试
- [ ] 添加并发控制测试
- [ ] 实现权限检查测试
- [ ] 添加缓存管理测试
- [ ] 实现错误处理和恢复测试

### 1.7 协议适配器单元测试 (src/integrations/adapters/mod.rs)
- [x] 创建 adapters_tests.rs 测试文件
- [ ] 实现协议适配器注册和选择测试
- [ ] 添加能力匹配测试
- [ ] 实现错误处理测试

### 1.8 事件系统单元测试 (src/integrations/events/bus.rs)
- [x] 创建 events_tests.rs 测试文件
- [ ] 实现事件发布和订阅测试
- [ ] 添加多处理器处理测试
- [ ] 实现错误传播和处理测试
- [ ] 添加并发事件处理测试

## 2. 集成测试 implementation plan

### 2.1 API端点集成测试 (src/ui/api/mod.rs)
- [x] 创建 api_tests.rs 测试文件
- [ ] 实现所有API端点的功能测试
- [ ] 添加健康检查端点测试
- [ ] 实现错误响应测试
- [ ] 添加负载测试

### 2.2 对话缓冲区专门测试 (src/core/state/mod.rs)
- [x] 创建 conversation_buffer_tests.rs 测试文件
- [ ] 实现缓冲区容量管理测试
- [ ] 添加消息顺序保持测试
- [ ] 实现缓冲区清空和重置测试
- [ ] 添加性能基准测试

### 2.3 LLM管理器集成测试 (src/llm/manager.rs)
- [x] 创建 llm_manager_tests.rs 测试文件
- [ ] 实现多提供商管理测试
- [ ] 添加提供商切换测试
- [ ] 实现初始化和配置测试

### 2.4 A2A协议集成测试 (src/a2a/)
- [x] 创建 a2a_tests.rs 测试文件
- [ ] 实现客户端功能测试
- [ ] 添加协议消息处理测试
- [ ] 实现错误处理测试

### 2.5 MCP协议集成测试 (src/mcp/)
- [x] 创建 mcp_tests.rs 测试文件
- [ ] 实现客户端功能测试
- [ ] 添加协议消息处理测试
- [ ] 实现工具调用测试

## 3. 测试文件组织结构

```
tests/
├── unit_tests/
│   ├── core/
│   │   ├── router_tests.rs
│   │   ├── state_tests.rs
│   │   ├── capabilities_tests.rs
│   ├── services/
│   │   ├── llm_tests.rs
│   │   ├── memory_tests.rs
│   │   ├── tools_tests.rs
│   ├── integrations/
│   │   ├── adapters_tests.rs
│   │   ├── events_tests.rs
├── integration_tests/
│   ├── api_tests.rs
│   ├── conversation_buffer_tests.rs
│   ├── protocol_tests/
│   │   ├── a2a_tests.rs
│   │   ├── mcp_tests.rs
│   ├── llm_manager_tests.rs
```
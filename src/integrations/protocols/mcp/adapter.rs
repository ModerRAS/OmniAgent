//! MCP协议适配器实现
//!
//! 该模块提供了MCP协议的适配器实现，用于集成到协议适配器框架中。

use crate::integrations::adapters::{ProtocolAdapter, AdapterError};
use crate::integrations::protocols::mcp::client::MCPClient;
use async_trait::async_trait;
use std::sync::Arc;

/// MCP协议适配器
pub struct MCPAdapter {
    /// MCP客户端
    client: Arc<MCPClient>,
    
    /// 适配器名称
    name: String,
    
    /// 适配器能力列表
    capabilities: Vec<String>,
}

impl MCPAdapter {
    /// 创建新的MCP适配器
    ///
    /// # 参数
    /// * `client` - MCP客户端实例
    /// * `name` - 适配器名称
    /// * `capabilities` - 适配器能力列表
    ///
    /// # 返回值
    /// 返回新的MCP适配器实例
    pub fn new(client: Arc<MCPClient>, name: String, capabilities: Vec<String>) -> Self {
        Self {
            client,
            name,
            capabilities,
        }
    }
}

#[async_trait]
impl ProtocolAdapter for MCPAdapter {
    /// 发送请求到MCP服务器
    ///
    /// # 参数
    /// * `request` - 请求内容（JSON格式的字符串）
    ///
    /// # 返回值
    /// 返回服务器响应或错误
    async fn send_request(&self, request: &str) -> Result<String, AdapterError> {
        // 解析请求内容
        let request_value: serde_json::Value = match serde_json::from_str(request) {
            Ok(value) => value,
            Err(e) => return Err(AdapterError::MockError(format!("Failed to parse request: {}", e))),
        };
        
        // 提取工具名称和参数
        let tool_name = request_value.get("tool").and_then(|v| v.as_str()).unwrap_or("default");
        let parameters = request_value.get("parameters").cloned().unwrap_or(serde_json::Value::Null);
        
        // 调用工具
        match self.client.call_tool(tool_name, parameters).await {
            Ok(result) => {
                // 将结果转换为字符串
                match serde_json::to_string(&result) {
                    Ok(result_str) => Ok(result_str),
                    Err(e) => Err(AdapterError::MockError(format!("Failed to serialize result: {}", e))),
                }
            },
            Err(e) => Err(AdapterError::MockError(format!("Failed to call tool: {}", e))),
        }
    }
    
    /// 获取适配器能力列表
    ///
    /// # 返回值
    /// 返回适配器支持的能力列表
    fn get_capabilities(&self) -> Vec<String> {
        self.capabilities.clone()
    }
    
    /// 获取适配器名称
    ///
    /// # 返回值
    /// 返回适配器名称
    fn get_name(&self) -> &str {
        &self.name
    }
}
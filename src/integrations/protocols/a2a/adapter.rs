//! A2A协议适配器实现
//!
//! 该模块提供了A2A协议的适配器实现，用于集成到协议适配器框架中。

use crate::integrations::adapters::{ProtocolAdapter, AdapterError};
use crate::integrations::protocols::a2a::client::A2AClient;
use crate::integrations::protocols::a2a::message::{A2AMessage, A2AMessageContent};
use async_trait::async_trait;
use std::sync::Arc;

/// A2A协议适配器
pub struct A2AAdapter {
    /// A2A客户端
    client: Arc<A2AClient>,
    
    /// 适配器名称
    name: String,
    
    /// 适配器能力列表
    capabilities: Vec<String>,
}

impl A2AAdapter {
    /// 创建新的A2A适配器
    ///
    /// # 参数
    /// * `client` - A2A客户端实例
    /// * `name` - 适配器名称
    /// * `capabilities` - 适配器能力列表
    ///
    /// # 返回值
    /// 返回新的A2A适配器实例
    pub fn new(client: Arc<A2AClient>, name: String, capabilities: Vec<String>) -> Self {
        Self {
            client,
            name,
            capabilities,
        }
    }
}

#[async_trait]
impl ProtocolAdapter for A2AAdapter {
    /// 发送请求到A2A服务器
    ///
    /// # 参数
    /// * `request` - 请求内容（JSON格式的字符串）
    ///
    /// # 返回值
    /// 返回服务器响应或错误
    async fn send_request(&self, request: &str) -> Result<String, AdapterError> {
        // 解析请求内容
        let message_content: A2AMessageContent = match serde_json::from_str(request) {
            Ok(content) => content,
            Err(e) => return Err(AdapterError::MockError(format!("Failed to parse request: {}", e))),
        };
        
        // 创建A2A消息
        let message = A2AMessage::new(
            "omni-agent".to_string(),
            "remote-agent".to_string(),
            message_content,
            None,
        );
        
        // 发送消息
        match self.client.send_message(message).await {
            Ok(response) => {
                // 将响应转换为字符串
                match serde_json::to_string(&response) {
                    Ok(response_str) => Ok(response_str),
                    Err(e) => Err(AdapterError::MockError(format!("Failed to serialize response: {}", e))),
                }
            },
            Err(e) => Err(AdapterError::MockError(format!("Failed to send message: {}", e))),
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
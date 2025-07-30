//! MCP协议客户端实现
//!
//! 该模块提供了与MCP协议服务器通信的客户端功能。

use crate::integrations::protocols::mcp::message::{MCPMessage, MCPResponse};
use crate::integrations::protocols::mcp::manifest::MCPManifest;
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

/// MCP协议错误类型
#[derive(Debug, Error)]
pub enum MCPError {
    /// HTTP请求错误
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    
    /// JSON序列化/反序列化错误
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    /// 协议错误
    #[error("Protocol error: {0}")]
    Protocol(String),
}

/// MCP协议客户端
#[derive(Debug, Clone)]
pub struct MCPClient {
    /// 基础URL
    pub base_url: String,
    
    /// HTTP客户端
    pub client: reqwest::Client,
}

impl MCPClient {
    /// 创建新的MCP客户端
    ///
    /// # 参数
    /// * `base_url` - MCP服务器的基础URL
    ///
    /// # 返回值
    /// 返回新的MCP客户端实例
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    /// 获取MCP服务器清单
    ///
    /// # 返回值
    /// 返回服务器清单信息或错误
    pub async fn fetch_manifest(&self) -> Result<MCPManifest, MCPError> {
        let url = format!("{}/manifest", self.base_url);
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(MCPError::Protocol(format!(
                "Failed to fetch manifest: {}",
                response.status()
            )));
        }

        let manifest: MCPManifest = response.json().await?;
        Ok(manifest)
    }

    /// 调用MCP工具
    ///
    /// # 参数
    /// * `tool_name` - 工具名称
    /// * `parameters` - 工具参数
    ///
    /// # 返回值
    /// 返回工具调用结果或错误
    pub async fn call_tool(
        &self,
        tool_name: &str,
        parameters: serde_json::Value,
    ) -> Result<serde_json::Value, MCPError> {
        let url = format!("{}/tools/{}/call", self.base_url, tool_name);

        #[derive(Serialize)]
        struct ToolCallRequest {
            parameters: serde_json::Value,
            id: Uuid,
        }

        let request = ToolCallRequest {
            parameters,
            id: Uuid::new_v4(),
        };

        let response = self.client.post(&url).json(&request).send().await?;

        if !response.status().is_success() {
            return Err(MCPError::Protocol(format!(
                "Tool call failed: {}",
                response.status()
            )));
        }

        let result: serde_json::Value = response.json().await?;
        Ok(result)
    }
}
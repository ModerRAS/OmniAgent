//! MCP协议消息定义
//!
//! 该模块定义了MCP协议中使用的消息结构和响应格式。

use serde::{Deserialize, Serialize};

/// MCP消息结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPMessage {
    /// 方法名称
    pub method: String,
    
    /// 参数
    pub params: serde_json::Value,
    
    /// 消息ID
    pub id: Option<String>,
}

/// MCP响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPResponse {
    /// 结果
    pub result: Option<serde_json::Value>,
    
    /// 错误信息
    pub error: Option<MCPError>,
    
    /// 消息ID
    pub id: Option<String>,
}

/// MCP错误结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPError {
    /// 错误代码
    pub code: i32,
    
    /// 错误消息
    pub message: String,
    
    /// 错误数据
    pub data: Option<serde_json::Value>,
}
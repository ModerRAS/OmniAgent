//! MCP协议清单定义
//!
//! 该模块定义了MCP协议中使用的清单结构，用于描述MCP工具的能力和信息。

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MCP清单结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPManifest {
    /// 工具名称
    pub name: String,
    
    /// 工具版本
    pub version: String,
    
    /// 工具描述
    pub description: String,
    
    /// 工具定义列表
    pub tools: Vec<MCPTool>,
    
    /// 能力信息
    pub capabilities: Option<serde_json::Value>,
}

/// MCP工具结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPTool {
    /// 工具名称
    pub name: String,
    
    /// 工具描述
    pub description: String,
    
    /// 工具输入参数
    pub input_schema: serde_json::Value,
}
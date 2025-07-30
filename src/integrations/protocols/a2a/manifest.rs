//! A2A协议清单定义
//!
//! 该模块定义了A2A协议中使用的清单结构，用于描述A2A智能体的能力和信息。

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A2A清单结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2AManifest {
    /// 智能体名称
    pub name: String,
    
    /// 智能体版本
    pub version: String,
    
    /// 智能体描述
    pub description: String,
    
    /// 智能体能力列表
    pub capabilities: Vec<A2ACapability>,
    
    /// 智能体端点信息
    pub endpoints: HashMap<String, String>,
}

/// A2A能力结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2ACapability {
    /// 能力名称
    pub name: String,
    
    /// 能力描述
    pub description: String,
    
    /// 能力参数
    pub parameters: Option<serde_json::Value>,
}
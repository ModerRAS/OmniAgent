//! MCP协议适配器实现
//!
//! 该模块提供了完整的MCP (Model Context Protocol) 协议支持，包括工具调用、清单获取等功能。

pub mod client;
pub mod message;
pub mod manifest;
pub mod adapter;

pub use client::MCPClient;
pub use message::{MCPMessage, MCPResponse, MCPError};
pub use manifest::MCPManifest;
pub use adapter::MCPAdapter;
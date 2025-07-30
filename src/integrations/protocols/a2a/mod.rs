//! A2A协议适配器实现
//!
//! 该模块提供了完整的A2A (Agent-to-Agent) 协议支持，包括消息传输、清单获取等功能。

pub mod client;
pub mod message;
pub mod manifest;
pub mod adapter;

pub use client::A2AClient;
pub use message::{A2AMessage, A2AMessageContent};
pub use manifest::A2AManifest;
pub use adapter::A2AAdapter;
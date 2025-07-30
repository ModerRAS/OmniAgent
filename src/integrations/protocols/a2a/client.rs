//! A2A协议客户端实现
//!
//! 该模块提供了与A2A协议服务器通信的客户端功能。

use crate::integrations::protocols::a2a::message::A2AMessage;
use crate::integrations::protocols::a2a::manifest::A2AManifest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

/// A2A协议错误类型
#[derive(Debug, Error)]
pub enum A2AError {
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

/// A2A协议客户端
#[derive(Debug, Clone)]
pub struct A2AClient {
    /// 基础URL
    pub base_url: String,
    
    /// HTTP客户端
    pub client: reqwest::Client,
}

impl A2AClient {
    /// 创建新的A2A客户端
    ///
    /// # 参数
    /// * `base_url` - A2A服务器的基础URL
    ///
    /// # 返回值
    /// 返回新的A2A客户端实例
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    /// 获取A2A服务器清单
    ///
    /// # 返回值
    /// 返回服务器清单信息或错误
    pub async fn fetch_manifest(&self) -> Result<A2AManifest, A2AError> {
        let url = format!("{}/manifest", self.base_url);
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(A2AError::Protocol(format!(
                "Failed to fetch manifest: {}",
                response.status()
            )));
        }

        let manifest: A2AManifest = response.json().await?;
        Ok(manifest)
    }

    /// 发送A2A消息
    ///
    /// # 参数
    /// * `message` - 要发送的A2A消息
    ///
    /// # 返回值
    /// 返回服务器响应消息或错误
    pub async fn send_message(&self, message: A2AMessage) -> Result<A2AMessage, A2AError> {
        let url = format!("{}/messages", self.base_url);

        let response = self.client.post(&url).json(&message).send().await?;

        if !response.status().is_success() {
            return Err(A2AError::Protocol(format!(
                "Failed to send message: {}",
                response.status()
            )));
        }

        let response_message: A2AMessage = response.json().await?;
        Ok(response_message)
    }
}
//! 事件总线实现
//!
//! 该模块提供了事件总线的实现，用于在系统中发布和订阅事件。

use crate::integrations::events::types::{Event, EventType};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use thiserror::Error;

/// 事件总线错误类型
#[derive(Debug, Error)]
pub enum EventBusError {
    /// 广播错误
    #[error("Broadcast error: {0}")]
    Broadcast(#[from] broadcast::error::SendError<Event>),
    
    /// 订阅错误
    #[error("Subscription error: {0}")]
    Subscription(String),
}

/// 事件处理器 trait
#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    /// 处理事件
    ///
    /// # 参数
    /// * `event` - 要处理的事件
    ///
    /// # 返回值
    /// 返回处理结果
    async fn handle_event(&self, event: &Event) -> Result<(), EventBusError>;
}

/// 事件总线
pub struct EventBus {
    /// 事件处理器映射表
    handlers: Arc<RwLock<HashMap<String, Arc<dyn EventHandler>>>>,
    
    /// 事件广播发送器
    sender: broadcast::Sender<Event>,
}

impl EventBus {
    /// 创建新的事件总线
    ///
    /// # 返回值
    /// 返回新的事件总线实例
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
            sender,
        }
    }
    
    /// 订阅事件
    ///
    /// # 参数
    /// * `handler_id` - 处理器ID
    /// * `handler` - 事件处理器
    ///
    /// # 返回值
    /// 返回订阅结果
    pub async fn subscribe(&self, handler_id: String, handler: Arc<dyn EventHandler>) -> Result<(), EventBusError> {
        let mut handlers = self.handlers.write().await;
        handlers.insert(handler_id, handler);
        Ok(())
    }
    
    /// 取消订阅事件
    ///
    /// # 参数
    /// * `handler_id` - 处理器ID
    ///
    /// # 返回值
    /// 返回取消订阅结果
    pub async fn unsubscribe(&self, handler_id: &str) -> Result<(), EventBusError> {
        let mut handlers = self.handlers.write().await;
        handlers.remove(handler_id);
        Ok(())
    }
    
    /// 发布事件
    ///
    /// # 参数
    /// * `event` - 要发布的事件
    ///
    /// # 返回值
    /// 返回发布结果
    pub async fn publish(&self, event: Event) -> Result<(), EventBusError> {
        // 发送事件到所有订阅者
        let _ = self.sender.send(event.clone());
        
        // 调用注册的处理器
        let handlers = self.handlers.read().await;
        for handler in handlers.values() {
            if let Err(e) = handler.handle_event(&event).await {
                eprintln!("Error handling event: {}", e);
            }
        }
        
        Ok(())
    }
    
    /// 订阅特定类型的事件
    ///
    /// # 返回值
    /// 返回事件接收器
    pub fn subscribe_to_events(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }
}
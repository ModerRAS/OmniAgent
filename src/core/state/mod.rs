//! 状态管理器模块 - 简化版

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::VecDeque;

/// 内存层级
#[derive(Debug, Clone, PartialEq)]
pub enum MemoryTier {
    ShortTerm,
    MediumTerm,
    LongTerm,
}

/// 消息类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    UserMessage,
    SystemMessage,
    ToolResponse,
    LLMResponse,
}

/// 缓冲消息结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferedMessage {
    pub id: uuid::Uuid,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub message_type: MessageType,
    pub context_relevance: f32,
}

/// 对话缓冲区
pub struct ConversationBuffer {
    messages: Arc<RwLock<VecDeque<BufferedMessage>>>,
    max_size: usize,
    current_size: AtomicUsize,
}

impl Clone for ConversationBuffer {
    fn clone(&self) -> Self {
        Self {
            messages: Arc::clone(&self.messages),
            max_size: self.max_size,
            current_size: AtomicUsize::new(self.current_size.load(Ordering::Relaxed)),
        }
    }
}

impl ConversationBuffer {
    pub fn new(max_size: usize) -> Self {
        Self {
            messages: Arc::new(RwLock::new(VecDeque::with_capacity(max_size))),
            max_size,
            current_size: AtomicUsize::new(0),
        }
    }

    /// 添加消息到缓冲区
    pub async fn add_message(&self, message: BufferedMessage) -> Result<(), String> {
        let mut messages = self.messages.write().await;
        
        // 如果缓冲区已满，移除最旧的消息
        if messages.len() >= self.max_size {
            messages.pop_front();
        }
        
        messages.push_back(message);
        self.current_size.store(messages.len(), Ordering::Relaxed);
        
        Ok(())
    }

    /// 获取缓冲区中的所有消息
    pub async fn get_messages(&self) -> Vec<BufferedMessage> {
        let messages = self.messages.read().await;
        messages.iter().cloned().collect()
    }

    /// 清空缓冲区
    pub async fn clear(&self) -> Result<(), String> {
        let mut messages = self.messages.write().await;
        messages.clear();
        self.current_size.store(0, Ordering::Relaxed);
        Ok(())
    }
}

/// 状态管理器
#[derive(Clone)]
pub struct StateManager {
    conversation_buffer: Arc<ConversationBuffer>,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            conversation_buffer: Arc::new(ConversationBuffer::new(10)), // 默认缓冲区大小为10
        }
    }

    /// 添加消息到对话缓冲区
    pub async fn add_to_buffer(&self, message: BufferedMessage) -> Result<(), String> {
        self.conversation_buffer.add_message(message).await
    }

    /// 从对话缓冲区获取消息
    pub async fn get_buffer_messages(&self) -> Vec<BufferedMessage> {
        self.conversation_buffer.get_messages().await
    }

    /// 清空对话缓冲区
    pub async fn clear_buffer(&self) -> Result<(), String> {
        self.conversation_buffer.clear().await
    }

    /// 获取缓冲区大小
    pub fn buffer_size(&self) -> usize {
        self.conversation_buffer.current_size.load(Ordering::Relaxed)
    }
}
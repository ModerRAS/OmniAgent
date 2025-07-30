//! 数据访问仓库
//!
//! 该模块提供了数据访问仓库实现，用于操作数据库中的记录。

use crate::integrations::database::models::{AgentRecord, ConversationRecord, MessageRecord};
use crate::integrations::database::connection::DatabaseManager;
use sqlx::types::Uuid;
use chrono::Utc;
use thiserror::Error;

/// 数据仓库错误类型
#[derive(Debug, Error)]
pub enum RepositoryError {
    /// 数据库错误
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    /// 记录未找到错误
    #[error("Record not found")]
    NotFound,
}

/// 数据仓库 trait
#[async_trait::async_trait]
pub trait DataRepository: Send + Sync {
    /// 保存智能体记录
    async fn save_agent(&self, agent: &AgentRecord) -> Result<(), RepositoryError>;
    
    /// 根据ID获取智能体记录
    async fn get_agent_by_id(&self, id: &Uuid) -> Result<Option<AgentRecord>, RepositoryError>;
    
    /// 保存对话记录
    async fn save_conversation(&self, conversation: &ConversationRecord) -> Result<(), RepositoryError>;
    
    /// 根据ID获取对话记录
    async fn get_conversation_by_id(&self, id: &Uuid) -> Result<Option<ConversationRecord>, RepositoryError>;
    
    /// 保存消息记录
    async fn save_message(&self, message: &MessageRecord) -> Result<(), RepositoryError>;
    
    /// 根据对话ID获取消息记录列表
    async fn get_messages_by_conversation_id(&self, conversation_id: &Uuid) -> Result<Vec<MessageRecord>, RepositoryError>;
}

/// PostgreSQL数据仓库实现
pub struct PostgresRepository {
    /// 数据库管理器
    db_manager: DatabaseManager,
}

impl PostgresRepository {
    /// 创建新的PostgreSQL数据仓库
    ///
    /// # 参数
    /// * `db_manager` - 数据库管理器实例
    ///
    /// # 返回值
    /// 返回新的PostgreSQL数据仓库实例
    pub fn new(db_manager: DatabaseManager) -> Self {
        Self { db_manager }
    }
}

#[async_trait::async_trait]
impl DataRepository for PostgresRepository {
    /// 保存智能体记录
    async fn save_agent(&self, agent: &AgentRecord) -> Result<(), RepositoryError> {
        sqlx::query!(
            "INSERT INTO agents (id, name, agent_type, config, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (id) DO UPDATE SET name = $2, agent_type = $3, config = $4, updated_at = $6",
            agent.id,
            agent.name,
            agent.agent_type,
            agent.config,
            agent.created_at,
            agent.updated_at
        )
        .execute(self.db_manager.get_pool())
        .await?;

        Ok(())
    }
    
    /// 根据ID获取智能体记录
    async fn get_agent_by_id(&self, id: &Uuid) -> Result<Option<AgentRecord>, RepositoryError> {
        let row = sqlx::query_as!(
            AgentRecord,
            "SELECT id, name, agent_type, config, created_at, updated_at FROM agents WHERE id = $1",
            id
        )
        .fetch_optional(self.db_manager.get_pool())
        .await?;

        Ok(row)
    }
    
    /// 保存对话记录
    async fn save_conversation(&self, conversation: &ConversationRecord) -> Result<(), RepositoryError> {
        sqlx::query!(
            "INSERT INTO conversations (id, title, participants, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)
             ON CONFLICT (id) DO UPDATE SET title = $2, participants = $3, updated_at = $5",
            conversation.id,
            conversation.title,
            &conversation.participants,
            conversation.created_at,
            conversation.updated_at
        )
        .execute(self.db_manager.get_pool())
        .await?;

        Ok(())
    }
    
    /// 根据ID获取对话记录
    async fn get_conversation_by_id(&self, id: &Uuid) -> Result<Option<ConversationRecord>, RepositoryError> {
        let row = sqlx::query_as!(
            ConversationRecord,
            "SELECT id, title, participants, created_at, updated_at FROM conversations WHERE id = $1",
            id
        )
        .fetch_optional(self.db_manager.get_pool())
        .await?;

        Ok(row)
    }
    
    /// 保存消息记录
    async fn save_message(&self, message: &MessageRecord) -> Result<(), RepositoryError> {
        sqlx::query!(
            "INSERT INTO messages (id, conversation_id, sender, recipient, content, message_type, timestamp, metadata) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            message.id,
            message.conversation_id,
            message.sender,
            message.recipient,
            message.content,
            message.message_type,
            message.timestamp,
            message.metadata
        )
        .execute(self.db_manager.get_pool())
        .await?;

        Ok(())
    }
    
    /// 根据对话ID获取消息记录列表
    async fn get_messages_by_conversation_id(&self, conversation_id: &Uuid) -> Result<Vec<MessageRecord>, RepositoryError> {
        let rows = sqlx::query_as!(
            MessageRecord,
            "SELECT id, conversation_id, sender, recipient, content, message_type, timestamp, metadata FROM messages WHERE conversation_id = $1 ORDER BY timestamp ASC",
            conversation_id
        )
        .fetch_all(self.db_manager.get_pool())
        .await?;

        Ok(rows)
    }
}
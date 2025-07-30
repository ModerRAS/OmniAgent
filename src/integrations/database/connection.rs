//! 数据库连接管理
//!
//! 该模块提供了数据库连接管理和池化功能。

use sqlx::{PgPool, PgPoolOptions};
use std::sync::Arc;
use thiserror::Error;

/// 数据库错误类型
#[derive(Debug, Error)]
pub enum DatabaseError {
    /// SQLx错误
    #[error("SQLx error: {0}")]
    Sqlx(#[from] sqlx::Error),
    
    /// 配置错误
    #[error("Configuration error: {0}")]
    Config(String),
}

/// 数据库管理器
#[derive(Debug, Clone)]
pub struct DatabaseManager {
    /// PostgreSQL连接池
    pool: Arc<PgPool>,
}

impl DatabaseManager {
    /// 创建新的数据库管理器
    ///
    /// # 参数
    /// * `database_url` - 数据库连接URL
    ///
    /// # 返回值
    /// 返回新的数据库管理器实例或错误
    pub async fn new(database_url: &str) -> Result<Self, DatabaseError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    /// 获取数据库连接池
    ///
    /// # 返回值
    /// 返回数据库连接池的引用
    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    
    /// 检查数据库连接
    ///
    /// # 返回值
    /// 返回连接检查结果
    pub async fn check_connection(&self) -> Result<bool, DatabaseError> {
        match sqlx::query("SELECT 1").execute(&*self.pool).await {
            Ok(_) => Ok(true),
            Err(e) => Err(DatabaseError::Sqlx(e)),
        }
    }
}
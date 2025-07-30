//! 数据库集成模块
//!
//! 该模块提供了数据库集成功能，包括连接管理、数据访问等。

pub mod connection;
pub mod models;
pub mod repository;

pub use connection::DatabaseManager;
pub use models::{AgentRecord, ConversationRecord, MessageRecord};
pub use repository::DataRepository;
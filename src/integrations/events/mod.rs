//! 事件驱动协调系统
//!
//! 该模块提供了事件驱动的协调系统，用于处理智能体间的异步通信和协调。

pub mod bus;
pub mod handler;
pub mod types;

pub use bus::EventBus;
pub use handler::EventHandler;
pub use types::{Event, EventType};
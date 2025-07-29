//! LLM服务模块 - 简化版

/// LLM错误
#[derive(Debug, thiserror::Error)]
pub enum LLMError {
    #[error("模拟错误: {0}")]
    MockError(String),
}

/// LLM服务
#[derive(Clone)]
pub struct LLMService {
    use_mock: bool,
}

impl LLMService {
    pub fn new(use_mock: bool) -> Self {
        Self { use_mock }
    }

    /// 处理消息
    pub async fn process_message(&self, message: &str, _context: &[String]) -> Result<String, LLMError> {
        if self.use_mock {
            // 返回模拟响应
            Ok(format!("模拟LLM响应: {}", message))
        } else {
            // 这里将实现实际的LLM调用逻辑
            Ok(format!("LLM处理: {}", message))
        }
    }
}
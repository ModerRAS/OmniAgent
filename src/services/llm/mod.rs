//! LLM服务模块 - 支持令牌计算优化

use std::collections::HashMap;

/// LLM错误
#[derive(Debug, thiserror::Error)]
pub enum LLMError {
    #[error("模拟错误: {0}")]
    MockError(String),
    #[error("令牌计算错误: {0}")]
    TokenError(String),
}

/// 令牌使用统计
#[derive(Debug, Clone)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
    pub cached_tokens: u32,
}

/// 缓存条目
#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub response: String,
    pub token_usage: TokenUsage,
    pub timestamp: std::time::SystemTime,
}

/// LLM服务
#[derive(Clone)]
pub struct LLMService {
    use_mock: bool,
    cache: std::sync::Arc<tokio::sync::RwLock<HashMap<String, CacheEntry>>>,
}

impl LLMService {
    pub fn new(use_mock: bool) -> Self {
        Self { 
            use_mock,
            cache: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    /// 处理消息并计算令牌使用
    pub async fn process_message(&self, message: &str, context: &[String]) -> Result<(String, TokenUsage), LLMError> {
        // 生成缓存键
        let cache_key = self.generate_cache_key(message, context);
        
        // 检查缓存
        if let Some(cached) = self.get_from_cache(&cache_key).await {
            return Ok((cached.response, cached.token_usage));
        }
        
        // 计算输入令牌数
        let prompt_tokens = self.calculate_tokens(message) + 
                           context.iter().map(|c| self.calculate_tokens(c)).sum::<u32>();
        
        // 处理消息
        let response = if self.use_mock {
            format!("模拟LLM响应: {}", message)
        } else {
            format!("LLM处理: {}", message)
        };
        
        // 计算输出令牌数
        let completion_tokens = self.calculate_tokens(&response);
        let total_tokens = prompt_tokens + completion_tokens;
        
        let token_usage = TokenUsage {
            prompt_tokens,
            completion_tokens,
            total_tokens,
            cached_tokens: 0, // 新请求，无缓存令牌
        };
        
        // 存储到缓存
        let cache_entry = CacheEntry {
            response: response.clone(),
            token_usage: token_usage.clone(),
            timestamp: std::time::SystemTime::now(),
        };
        self.store_in_cache(cache_key, cache_entry).await;
        
        Ok((response, token_usage))
    }
    
    /// 从缓存获取响应
    async fn get_from_cache(&self, key: &str) -> Option<CacheEntry> {
        let cache = self.cache.read().await;
        cache.get(key).cloned()
    }
    
    /// 存储到缓存
    async fn store_in_cache(&self, key: String, entry: CacheEntry) {
        let mut cache = self.cache.write().await;
        cache.insert(key, entry);
    }
    
    /// 生成缓存键
    fn generate_cache_key(&self, message: &str, context: &[String]) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        message.hash(&mut hasher);
        for ctx in context {
            ctx.hash(&mut hasher);
        }
        format!("{:x}", hasher.finish())
    }
    
    /// 简单的令牌计算（实际实现中可能需要更复杂的逻辑）
    fn calculate_tokens(&self, text: &str) -> u32 {
        // 简化实现：假设每个词大约1.3个令牌
        let words: Vec<&str> = text.split_whitespace().collect();
        (words.len() as f32 * 1.3).ceil() as u32
    }
    
    /// 清理过期缓存
    pub async fn cleanup_cache(&self, max_age_seconds: u64) -> Result<usize, LLMError> {
        let now = std::time::SystemTime::now();
        let mut cache = self.cache.write().await;
        let count = cache.len();
        
        cache.retain(|_, entry| {
            if let Ok(elapsed) = now.duration_since(entry.timestamp) {
                elapsed.as_secs() < max_age_seconds
            } else {
                false
            }
        });
        
        Ok(count - cache.len())
    }
    
    /// 获取缓存统计信息
    pub async fn get_cache_stats(&self) -> (usize, usize) {
        let cache = self.cache.read().await;
        (cache.len(), cache.values().map(|e| e.token_usage.cached_tokens as usize).sum())
    }
}
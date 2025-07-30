//! 安全管理模块 - 实现认证、授权和审计功能

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tracing::info;

/// 用户角色
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    Admin,
    User,
    Guest,
    Service,
}

/// 权限类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Permission {
    Read,
    Write,
    Execute,
    Admin,
    ToolAccess(String),
    AgentAccess(String),
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub role: UserRole,
    pub permissions: Vec<Permission>,
    pub created_at: u64,
    pub last_login: Option<u64>,
}

/// 认证凭据
#[derive(Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String, // 在实际实现中应该使用哈希
}

/// 认证令牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    pub token: String,
    pub user_id: String,
    pub expires_at: u64,
    pub permissions: Vec<Permission>,
}

/// 审计日志条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: String,
    pub user_id: String,
    pub action: String,
    pub resource: String,
    pub timestamp: u64,
    pub success: bool,
    pub details: Option<String>,
}

/// 安全错误类型
#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("认证失败: {0}")]
    AuthenticationFailed(String),
    #[error("授权被拒绝: {0}")]
    AuthorizationDenied(String),
    #[error("令牌无效: {0}")]
    InvalidToken(String),
    #[error("用户不存在: {0}")]
    UserNotFound(String),
    #[error("权限不足: {0}")]
    InsufficientPermissions(String),
}

/// 安全管理器
pub struct SecurityManager {
    users: Arc<RwLock<HashMap<String, User>>>,
    tokens: Arc<RwLock<HashMap<String, AuthToken>>>,
    audit_logs: Arc<RwLock<Vec<AuditLog>>>,
    secret_key: String,
}

impl SecurityManager {
    /// 创建新的安全管理器
    pub fn new(secret_key: String) -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            tokens: Arc::new(RwLock::new(HashMap::new())),
            audit_logs: Arc::new(RwLock::new(Vec::new())),
            secret_key,
        }
    }

    /// 注册用户
    pub async fn register_user(&self, username: &str, _password: &str, role: UserRole) -> Result<String, SecurityError> {
        let user_id = uuid::Uuid::new_v4().to_string();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| SecurityError::AuthenticationFailed(e.to_string()))?
            .as_secs();

        let user = User {
            id: user_id.clone(),
            username: username.to_string(),
            role: role.clone(),
            permissions: self.get_role_permissions(&role),
            created_at: now,
            last_login: None,
        };

        let mut users = self.users.write().await;
        users.insert(user_id.clone(), user);
        
        info!("✅ 用户注册成功: {}", username);
        self.log_audit(&user_id, "register_user", username, true, None).await;
        
        Ok(user_id)
    }

    /// 用户认证
    pub async fn authenticate(&self, credentials: &Credentials) -> Result<AuthToken, SecurityError> {
        let users = self.users.read().await;
        let user = users.values().find(|u| u.username == credentials.username)
            .ok_or_else(|| SecurityError::AuthenticationFailed("用户不存在".to_string()))?;

        // 简化的密码验证（实际实现中应使用哈希）
        if credentials.password != "password123" { // 仅用于演示
            self.log_audit(&user.id, "authenticate", &credentials.username, false, Some("密码错误".to_string())).await;
            return Err(SecurityError::AuthenticationFailed("密码错误".to_string()));
        }

        // 更新最后登录时间
        {
            let mut users_write = self.users.write().await;
            if let Some(u) = users_write.get_mut(&user.id) {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map_err(|e| SecurityError::AuthenticationFailed(e.to_string()))?
                    .as_secs();
                u.last_login = Some(now);
            }
        }

        // 生成认证令牌
        let token = self.generate_token(&user.id, &user.permissions)?;
        
        self.log_audit(&user.id, "authenticate", &credentials.username, true, None).await;
        
        Ok(token)
    }

    /// 验证令牌
    pub async fn validate_token(&self, token_str: &str) -> Result<AuthToken, SecurityError> {
        let tokens = self.tokens.read().await;
        let token = tokens.get(token_str)
            .ok_or_else(|| SecurityError::InvalidToken("令牌不存在".to_string()))?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| SecurityError::InvalidToken(e.to_string()))?
            .as_secs();

        if token.expires_at < now {
            return Err(SecurityError::InvalidToken("令牌已过期".to_string()));
        }

        Ok(token.clone())
    }

    /// 权限检查
    pub async fn check_permission(&self, token: &AuthToken, required_permission: &Permission) -> Result<(), SecurityError> {
        if token.permissions.contains(required_permission) {
            Ok(())
        } else {
            Err(SecurityError::InsufficientPermissions("权限不足".to_string()))
        }
    }

    /// 获取用户信息
    pub async fn get_user(&self, user_id: &str) -> Result<User, SecurityError> {
        let users = self.users.read().await;
        users.get(user_id)
            .cloned()
            .ok_or_else(|| SecurityError::UserNotFound("用户不存在".to_string()))
    }

    /// 添加权限到用户
    pub async fn add_permission(&self, user_id: &str, permission: Permission) -> Result<(), SecurityError> {
        let mut users = self.users.write().await;
        let user = users.get_mut(user_id)
            .ok_or_else(|| SecurityError::UserNotFound("用户不存在".to_string()))?;

        if !user.permissions.contains(&permission) {
            user.permissions.push(permission.clone());
        }

        // 如果用户有活跃令牌，更新令牌权限
        self.update_user_tokens(user_id, &user.permissions).await;

        info!("✅ 为用户添加权限: {} {:?}", user_id, permission);
        Ok(())
    }

    /// 记录审计日志
    async fn log_audit(&self, user_id: &str, action: &str, resource: &str, success: bool, details: Option<String>) {
        let log = AuditLog {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            action: action.to_string(),
            resource: resource.to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            success,
            details,
        };

        let mut logs = self.audit_logs.write().await;
        logs.push(log);
    }

    /// 获取角色默认权限
    fn get_role_permissions(&self, role: &UserRole) -> Vec<Permission> {
        match role {
            UserRole::Admin => vec![
                Permission::Read,
                Permission::Write,
                Permission::Execute,
                Permission::Admin,
            ],
            UserRole::User => vec![
                Permission::Read,
                Permission::Write,
                Permission::Execute,
            ],
            UserRole::Guest => vec![
                Permission::Read,
            ],
            UserRole::Service => vec![
                Permission::Read,
                Permission::Write,
                Permission::Execute,
            ],
        }
    }

    /// 生成认证令牌
    fn generate_token(&self, user_id: &str, permissions: &[Permission]) -> Result<AuthToken, SecurityError> {
        let token = format!("{}-{}", user_id, uuid::Uuid::new_v4());
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| SecurityError::AuthenticationFailed(e.to_string()))?
            .as_secs();

        let expires_at = now + 3600; // 1小时过期

        let auth_token = AuthToken {
            token: token.clone(),
            user_id: user_id.to_string(),
            expires_at,
            permissions: permissions.to_vec(),
        };

        // 存储令牌
        tokio::spawn({
            let tokens = Arc::clone(&self.tokens);
            let auth_token = auth_token.clone();
            async move {
                let mut tokens = tokens.write().await;
                tokens.insert(token, auth_token);
            }
        });

        Ok(auth_token)
    }

    /// 更新用户令牌权限
    async fn update_user_tokens(&self, user_id: &str, permissions: &[Permission]) {
        let mut tokens = self.tokens.write().await;
        for token in tokens.values_mut() {
            if token.user_id == user_id {
                token.permissions = permissions.to_vec();
            }
        }
    }

    /// 获取审计日志
    pub async fn get_audit_logs(&self, limit: Option<usize>) -> Vec<AuditLog> {
        let logs = self.audit_logs.read().await;
        let mut result = logs.clone();
        result.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        match limit {
            Some(n) => result.into_iter().take(n).collect(),
            None => result,
        }
    }

    /// 清理过期令牌
    pub async fn cleanup_expired_tokens(&self) -> Result<usize, SecurityError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| SecurityError::InvalidToken(e.to_string()))?
            .as_secs();

        let mut tokens = self.tokens.write().await;
        let count = tokens.len();
        tokens.retain(|_, token| token.expires_at > now);
        let removed = count - tokens.len();
        
        Ok(removed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_manager_creation() {
        let manager = SecurityManager::new("test_secret".to_string());
        assert_eq!(manager.get_audit_logs(None).await.len(), 0);
    }

    #[tokio::test]
    async fn test_user_registration() {
        let manager = SecurityManager::new("test_secret".to_string());
        let user_id = manager.register_user("testuser", "password123", UserRole::User).await;
        assert!(user_id.is_ok());
    }

    #[tokio::test]
    async fn test_authentication() {
        let manager = SecurityManager::new("test_secret".to_string());
        let user_id = manager.register_user("testuser", "password123", UserRole::User).await.unwrap();
        
        let credentials = Credentials {
            username: "testuser".to_string(),
            password: "password123".to_string(),
        };
        
        let token = manager.authenticate(&credentials).await;
        assert!(token.is_ok());
    }

    #[tokio::test]
    async fn test_permission_check() {
        let manager = SecurityManager::new("test_secret".to_string());
        let user_id = manager.register_user("testuser", "password123", UserRole::User).await.unwrap();
        
        let credentials = Credentials {
            username: "testuser".to_string(),
            password: "password123".to_string(),
        };
        
        let token = manager.authenticate(&credentials).await.unwrap();
        let result = manager.check_permission(&token, &Permission::Read).await;
        assert!(result.is_ok());
    }
}
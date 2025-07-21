use clap::Parser;
use omni_agent::{AppConfig, AgentBuilder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::sync::RwLock;
use tracing::{info, warn, error, Level};
use tracing_subscriber::FmtSubscriber;
use axum::{
    routing::{get, post},
    Router,
    extract::State,
    Json,
    response::Json as JsonResponse,
};
use serde_json::{json, Value};
use std::sync::Arc;

/// 智能体应用配置
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// 配置文件路径
    #[arg(short, long, default_value = "config.json")]
    config: PathBuf,
    
    /// 启用模拟模式
    #[arg(long)]
    mock: bool,
    
    /// 服务器端口
    #[arg(short, long)]
    port: Option<u16>,
    
    /// 日志级别
    #[arg(long, default_value = "info")]
    log_level: String,
}

/// 请求消息结构
#[derive(Debug, Serialize, Deserialize)]
struct UserRequest {
    message: String,
    context: Option<HashMap<String, Value>>,
}

/// 响应消息结构
#[derive(Debug, Serialize, Deserialize)]
struct AgentResponse {
    message: String,
    source: String,
    details: HashMap<String, Value>,
}

/// 智能体应用状态
#[derive(Clone)]
struct AppState {
    agent: Arc<RwLock<omni_agent::Agent>>,
    config: AppConfig,
}

/// 智能路由器 - 决定使用哪个工具/智能体
struct IntelligentRouter;

impl IntelligentRouter {
    /// 分析用户消息并决定最佳行动方案
    async fn route_message(
        &self,
        message: &str,
        agent: &omni_agent::Agent,
    ) -> Result<(String, String, HashMap<String, Value>), Box<dyn std::error::Error>> {
        info!("🔍 分析用户消息: {}", message);
        
        // 1. 检查 MCP 工具是否适用
        if let Some((tool_name, tool_result)) = self.try_mcp_tools(message, agent).await? {
            info!("🛠️  使用 MCP 工具: {}", tool_name);
            return Ok((tool_result, "mcp_tool".to_string(), 
                      HashMap::from([("tool".to_string(), json!(tool_name))])));
        }
        
        // 2. 检查 A2A 智能体是否适用
        if let Some((agent_name, agent_result)) = self.try_a2a_agents(message, agent).await? {
            info!("🤝 使用 A2A 智能体: {}", agent_name);
            return Ok((agent_result, "a2a_agent".to_string(), 
                      HashMap::from([("agent".to_string(), json!(agent_name))])));
        }
        
        // 3. 使用本地 LLM
        info!("🧠 使用本地 LLM 回答");
        let llm_response = self.use_local_llm(message, agent).await?;
        Ok((llm_response, "local_llm".to_string(), HashMap::new()))
    }
    
    /// 尝试使用 MCP 工具解决请求
    async fn try_mcp_tools(
        &self,
        message: &str,
        _agent: &omni_agent::Agent,
    ) -> Result<Option<(String, String)>, Box<dyn std::error::Error>> {
        // 简单的关键词匹配策略
        let keywords = ["文件", "读取", "写入", "计算", "搜索", "查询"];
        
        for keyword in keywords.iter() {
            if message.contains(keyword) {
                // 这里应该实现实际的 MCP 工具调用逻辑
                // 目前返回模拟结果
                return Ok(Some((
                    format!("文件工具"),
                    format!("使用 MCP 工具处理了包含 '{}' 的请求: {}", keyword, message)
                )));
            }
        }
        
        Ok(None)
    }
    
    /// 尝试使用 A2A 智能体解决请求
    async fn try_a2a_agents(
        &self,
        message: &str,
        _agent: &omni_agent::Agent,
    ) -> Result<Option<(String, String)>, Box<dyn std::error::Error>> {
        // 简单的关键词匹配策略
        let keywords = ["天气", "时间", "新闻", "翻译", "定义", "查询"];
        
        for keyword in keywords.iter() {
            if message.contains(keyword) {
                // 这里应该实现实际的 A2A 智能体调用逻辑
                // 目前返回模拟结果
                return Ok(Some((
                    format!("天气智能体"),
                    format!("使用 A2A 智能体处理了包含 '{}' 的请求: {}", keyword, message)
                )));
            }
        }
        
        Ok(None)
    }
    
    /// 使用本地 LLM 回答
    async fn use_local_llm(
        &self,
        message: &str,
        agent: &omni_agent::Agent,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let response = agent
            .llm
            .write()
            .await
            .process_message(message, &[])
            .await?;
        
        match response.content {
            omni_agent::protocol::message::MessageContent::Text { text } => Ok(text),
            _ => Ok("无法处理消息格式".to_string()),
        }
    }
}

/// 创建默认配置文件
async fn create_default_config(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let default_config = json!({
        "server": {
            "port": 8080,
            "host": "127.0.0.1",
            "cors_origins": ["*"]
        },
        "llm": {
            "provider": "google",
            "model": "gemini-pro",
            "api_key": "",
            "base_url": null,
            "temperature": 0.7,
            "max_tokens": 1000,
            "use_mock": true
        },
        "mcp": {
            "servers": {
                "file-tools": {
                    "name": "文件工具",
                    "description": "文件操作工具",
                    "url": "http://localhost:3000",
                    "timeout": 30,
                    "retry_attempts": 3,
                    "enabled": true
                },
                "calculator": {
                    "name": "计算器",
                    "description": "数学计算工具",
                    "url": "http://localhost:3001",
                    "timeout": 30,
                    "retry_attempts": 3,
                    "enabled": true
                }
            },
            "enabled": true
        },
        "a2a": {
            "servers": {
                "weather-agent": {
                    "name": "天气智能体",
                    "description": "天气查询智能体",
                    "url": "http://localhost:8081",
                    "timeout": 30,
                    "retry_attempts": 3,
                    "enabled": true
                },
                "news-agent": {
                    "name": "新闻智能体",
                    "description": "新闻获取智能体",
                    "url": "http://localhost:8082",
                    "timeout": 30,
                    "retry_attempts": 3,
                    "enabled": true
                }
            },
            "enabled": true,
            "allow_external": true
        },
        "logging": {
            "level": "info",
            "format": "json",
            "file": null
        }
    });
    
    tokio::fs::write(path, serde_json::to_string_pretty(&default_config)?).await?;
    info!("✅ 已创建默认配置文件: {}", path.display());
    Ok(())
}

/// 处理聊天请求
async fn chat_handler(
    State(state): State<AppState>,
    Json(request): Json<UserRequest>,
) -> JsonResponse<AgentResponse> {
    let router = IntelligentRouter;
    
    let agent = state.agent.read().await;
    match router.route_message(&request.message, &agent).await {
        Ok((response, source, details)) => {
            JsonResponse(AgentResponse {
                message: response,
                source,
                details,
            })
        }
        Err(e) => {
            error!("❌ 处理消息失败: {}", e);
            JsonResponse(AgentResponse {
                message: format!("处理失败: {}", e),
                source: "error".to_string(),
                details: HashMap::new(),
            })
        }
    }
}

/// 健康检查端点
async fn health_handler() -> JsonResponse<Value> {
    JsonResponse(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// 获取智能体信息
async fn info_handler(State(state): State<AppState>) -> JsonResponse<Value> {
    let agent = state.agent.read().await;
    JsonResponse(json!({
        "name": agent.config.name,
        "description": agent.config.description,
        "version": agent.config.version,
        "mcp_clients": agent.mcp_clients.len(),
        "a2a_clients": agent.a2a_clients.len(),
        "llm_provider": state.config.llm.provider,
        "llm_model": state.config.llm.model
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 解析命令行参数
    let cli = Cli::parse();
    
    // 初始化日志
    let log_level = match cli.log_level.as_str() {
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };
    
    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    
    info!("🚀 启动智能体应用...");
    
    // 检查配置文件
    if !cli.config.exists() {
        warn!("⚠️  配置文件不存在，创建默认配置...");
        create_default_config(&cli.config).await?;
    }
    
    // 加载配置
    let mut config = AppConfig::load_from_file(cli.config.to_str().unwrap())?;
    config.override_with_env();
    
    // 应用命令行参数
    if cli.mock {
        config.llm.use_mock = true;
        info!("🎭 启用模拟模式");
    }
    
    if let Some(port) = cli.port {
        config.server.port = port;
        info!("🌐 使用端口: {}", port);
    }
    
    info!("📋 配置加载完成:");
    info!("   LLM 提供商: {}", config.llm.provider);
    info!("   模型: {}", config.llm.model);
    info!("   MCP 服务器: {}", config.mcp.servers.len());
    info!("   A2A 智能体: {}", config.a2a.servers.len());
    info!("   模拟模式: {}", config.llm.use_mock);
    
    // 创建智能体
    let mut agent_builder = AgentBuilder::new("omni-agent", "全能智能体助手")
        .version("1.0.0");
    
    // 添加 MCP 服务器
    for (name, server) in &config.mcp.servers {
        if server.enabled {
            agent_builder = agent_builder.add_mcp(name, &server.url);
        }
    }
    
    // 添加 A2A 智能体
    for (name, agent) in &config.a2a.servers {
        if agent.enabled {
            agent_builder = agent_builder.add_a2a(name, &agent.url);
        }
    }
    
    let agent = agent_builder.build().await?;
    info!("✅ 智能体创建完成");
    
    let port = config.server.port;
    
    // 创建应用状态
    let state = AppState {
        agent: Arc::new(RwLock::new(agent)),
        config,
    };
    
    // 创建路由
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/info", get(info_handler))
        .route("/chat", post(chat_handler))
        .with_state(state);
    
    let addr = format!("127.0.0.1:{}", port);
    info!("🌐 服务器启动于 http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
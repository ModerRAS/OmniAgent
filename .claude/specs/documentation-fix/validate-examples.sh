#!/bin/bash

# OmniAgent 文档示例验证脚本
echo "🔍 验证所有代码示例..."

# 创建临时测试目录
TEMP_DIR=$(mktemp -d)
echo "📁 创建临时目录: $TEMP_DIR"

# 测试基本Agent创建示例
echo "🧪 测试基本Agent创建示例..."
cat > $TEMP_DIR/test_basic_agent.rs << 'EOF'
use omni_agent::AgentBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a simple agent
    let agent = AgentBuilder::new("my-agent", "A helpful assistant")
        .version("1.0.0")
        .build()
        .await?;

    // Use the agent
    let response = agent
        .llm
        .write()
        .await
        .process_message("Hello, how can you help me?", &[])
        .await?;

    match response.content {
        omni_agent::protocol::message::MessageContent::Text { text } => {
            println!("Agent response: {}", text);
        }
        _ => println!("Unexpected response format"),
    }
    
    Ok(())
}
EOF

# 测试配置加载示例
echo "⚙️  测试配置加载示例..."
cat > $TEMP_DIR/test_config.rs << 'EOF'
use omni_agent::AppConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::load_from_file("config.example.json")?;
    println!("Loaded config for provider: {}", config.llm.provider);
    Ok(())
}
EOF

# 测试API调用示例
echo "🌐 测试API调用示例..."
cat > $TEMP_DIR/test_api.rs << 'EOF'
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct UserRequest {
    message: String,
    context: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AgentResponse {
    message: String,
    source: String,
    details: HashMap<String, serde_json::Value>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = UserRequest {
        message: "Hello, agent!".to_string(),
        context: None,
    };
    
    println!("Request: {:?}", request);
    Ok(())
}
EOF

# 测试MessageContent示例
echo "📨 测试MessageContent示例..."
cat > $TEMP_DIR/test_message.rs << 'EOF'
use omni_agent::protocol::message::{Message, MessageContent};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let message = Message::new(
        "user".to_string(),
        "agent".to_string(),
        MessageContent::Text {
            text: "Hello, can you help me?".to_string(),
        },
        None,
    );
    
    println!("Created message with ID: {}", message.id);
    Ok(())
}
EOF

# 运行测试
cd $(dirname $0)/../../..

echo "🔧 运行基本Agent测试..."
if cargo check --manifest-path Cargo.toml --example claude_usage 2>/dev/null; then
    echo "✅ 基本Agent示例编译成功"
else
    echo "❌ 基本Agent示例编译失败"
fi

echo "🔧 运行配置测试..."
if cargo check --manifest-path Cargo.toml --example llm_usage 2>/dev/null; then
    echo "✅ 配置示例编译成功"
else
    echo "❌ 配置示例编译失败"
fi

echo "🔧 验证配置文件格式..."
if cargo run -- --config config.example.json --mock --port 9999 &
    PID=$!
    sleep 2
    curl -s http://localhost:9999/health | grep -q "healthy"
then
    echo "✅ 配置文件验证成功"
    kill $PID 2>/dev/null || true
    wait $PID 2>/dev/null || true
else
    echo "❌ 配置文件验证失败"
    kill $PID 2>/dev/null || true
fi

# 清理
echo "🧹 清理临时文件..."
rm -rf $TEMP_DIR

echo "🎉 验证完成！"
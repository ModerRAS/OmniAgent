use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use axum::{
    routing::post,
    Router,
    Json,
    extract::State,
};
use axum::serve;
use tokio::net::TcpListener;
use serde::{Deserialize, Serialize};

// Mock Claude Server
#[derive(Debug, Serialize, Deserialize)]
struct MockClaudeRequest {
    model: String,
    messages: Vec<MockClaudeMessage>,
    max_tokens: u32,
    temperature: Option<f32>,
    system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MockClaudeMessage {
    role: String,
    content: Vec<MockClaudeContent>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MockClaudeContent {
    #[serde(rename = "type")]
    type_: String,
    text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MockClaudeResponse {
    id: String,
    content: Vec<MockClaudeResponseContent>,
    model: String,
    role: String,
    stop_reason: Option<String>,
    usage: MockUsage,
}

#[derive(Debug, Serialize, Deserialize)]
struct MockClaudeResponseContent {
    #[serde(rename = "type")]
    type_: String,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MockUsage {
    input_tokens: u32,
    output_tokens: u32,
}

#[derive(Clone)]
struct MockClaudeState {
    responses: Arc<RwLock<HashMap<String, String>>>,
}

async fn mock_claude_handler(
    State(state): State<MockClaudeState>,
    Json(request): Json<MockClaudeRequest>,
) -> Json<MockClaudeResponse> {
    let response_text = state.responses.read().await
        .get(&request.messages.last().unwrap().content[0].text.clone().unwrap_or_default())
        .cloned()
        .unwrap_or_else(|| "This is a mock Claude response".to_string());

    Json(MockClaudeResponse {
        id: "mock-123".to_string(),
        content: vec![MockClaudeResponseContent {
            type_: "text".to_string(),
            text: response_text,
        }],
        model: request.model,
        role: "assistant".to_string(),
        stop_reason: Some("end_turn".to_string()),
        usage: MockUsage {
            input_tokens: 10,
            output_tokens: 20,
        },
    })
}

// Mock OpenAI Server
#[derive(Debug, Serialize, Deserialize)]
struct MockOpenAIRequest {
    model: String,
    messages: Vec<MockOpenAIMessage>,
    temperature: Option<f64>,
    max_tokens: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MockOpenAIMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MockOpenAIResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<MockOpenAIChoice>,
    usage: MockOpenAIUsage,
}

#[derive(Debug, Serialize, Deserialize)]
struct MockOpenAIChoice {
    index: i32,
    message: MockOpenAIMessage,
    finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MockOpenAIUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Clone)]
struct MockOpenAIState {
    responses: Arc<RwLock<HashMap<String, String>>>,
}

async fn mock_openai_handler(
    State(state): State<MockOpenAIState>,
    Json(request): Json<MockOpenAIRequest>,
) -> Json<MockOpenAIResponse> {
    let last_message = request.messages.last().unwrap();
    let response_text = state.responses.read().await
        .get(&last_message.content)
        .cloned()
        .unwrap_or_else(|| "This is a mock OpenAI response".to_string());

    Json(MockOpenAIResponse {
        id: "mock-456".to_string(),
        object: "chat.completion".to_string(),
        created: chrono::Utc::now().timestamp(),
        model: request.model,
        choices: vec![MockOpenAIChoice {
            index: 0,
            message: MockOpenAIMessage {
                role: "assistant".to_string(),
                content: response_text,
            },
            finish_reason: "stop".to_string(),
        }],
        usage: MockOpenAIUsage {
            prompt_tokens: 10,
            completion_tokens: 20,
            total_tokens: 30,
        },
    })
}

pub async fn start_mock_claude_server(port: u16) -> String {
    let state = MockClaudeState {
        responses: Arc::new(RwLock::new(HashMap::new())),
    };
    
    // 添加一些测试响应
    state.responses.write().await.insert(
        "Hello".to_string(),
        "Hello! This is a mock Claude response.".to_string(),
    );

    let app = Router::new()
        .route("/v1/messages", post(mock_claude_handler))
        .with_state(state);

    let url = format!("http://localhost:{}", port);
    tokio::spawn(async move {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
        serve(listener, app).await.unwrap();
    });

    url
}

pub async fn start_mock_google_server(port: u16) -> String {
    let state = MockOpenAIState {
        responses: Arc::new(RwLock::new(HashMap::new())),
    };
    
    // 添加一些测试响应
    state.responses.write().await.insert(
        "Hello".to_string(),
        "Hello! This is a mock Google Gemini response.".to_string(),
    );

    let app = Router::new()
        .route("/v1beta/models/gemini-pro:generateContent", post(mock_google_handler))
        .with_state(state);

    let url = format!("http://localhost:{}", port);
    tokio::spawn(async move {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
        serve(listener, app).await.unwrap();
    });

    url
}

async fn mock_google_handler(
    State(state): State<MockOpenAIState>,
    Json(request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let request_text = request["contents"][0]["parts"][0]["text"]
        .as_str()
        .unwrap_or("Hello");

    let response_text = state.responses.read().await
        .get(request_text)
        .cloned()
        .unwrap_or_else(|| "This is a mock Google Gemini response".to_string());

    Json(serde_json::json!({
        "candidates": [{
            "content": {
                "parts": [{"text": response_text}],
                "role": "model"
            },
            "finishReason": "STOP",
            "index": 0
        }],
        "usageMetadata": {
            "promptTokenCount": 10,
            "candidatesTokenCount": 20,
            "totalTokenCount": 30
        },
        "modelVersion": "gemini-pro"
    }))
}

pub async fn start_mock_openai_server(port: u16) -> String {
    let state = MockOpenAIState {
        responses: Arc::new(RwLock::new(HashMap::new())),
    };
    
    // 添加一些测试响应
    state.responses.write().await.insert(
        "Hello".to_string(),
        "Hello! This is a mock OpenAI response.".to_string(),
    );

    let app = Router::new()
        .route("/v1/chat/completions", post(mock_openai_handler))
        .with_state(state);

    let url = format!("http://localhost:{}", port);
    tokio::spawn(async move {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
        serve(listener, app).await.unwrap();
    });

    url
}
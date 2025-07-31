//! 智能路由器单元测试

use omni_agent::core::router::{IntelligentRouter, RouteDecision, RouteTarget};
use tokio;

#[tokio::test]
async fn test_route_message_to_local_llm() {
    let router = IntelligentRouter::new();
    let decision = router.route_message("普通问题").await;
    
    assert_eq!(decision.target, RouteTarget::LocalLLM);
    assert!(decision.confidence > 0.0);
    assert!(!decision.reasoning.is_empty());
}

#[tokio::test]
async fn test_route_message_to_a2a_agent() {
    let router = IntelligentRouter::new();
    let decision = router.route_message("查询天气").await;
    
    match decision.target {
        RouteTarget::A2AAgent(agent_name) => {
            assert_eq!(agent_name, "info_agent");
        }
        _ => panic!("Expected A2AAgent route target"),
    }
    assert!(decision.confidence > 0.0);
}

#[tokio::test]
async fn test_route_message_to_mcp_tool() {
    let router = IntelligentRouter::new();
    let decision = router.route_message("处理文件").await;
    
    match decision.target {
        RouteTarget::MCPTool(tool_name) => {
            assert_eq!(tool_name, "file_processor");
        }
        _ => panic!("Expected MCPTool route target"),
    }
    assert!(decision.confidence > 0.0);
}

#[tokio::test]
async fn test_route_decision_fields() {
    let router = IntelligentRouter::new();
    let decision = router.route_message("测试消息").await;
    
    // 验证决策结构体字段
    assert!(!decision.id.is_nil());
    assert!(!decision.message.is_empty());
    assert!(decision.confidence >= 0.0 && decision.confidence <= 1.0);
    assert!(!decision.reasoning.is_empty());
    assert!(decision.timestamp.timestamp() > 0);
}
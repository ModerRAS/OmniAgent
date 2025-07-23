use omni_agent::{agent::Agent, agent::AgentConfig, protocol::agent_card::AgentCard};

#[tokio::test]
async fn test_agent_card_creation() {
    let agent_config = AgentConfig {
        name: "TestAgent".to_string(),
        description: "A test agent for unit testing".to_string(),
        version: "1.0.0".to_string(),
    };
    
    let agent = Agent::new(agent_config);
    let card = agent.get_agent_card("http://localhost:8080".to_string()).await;
    
    assert_eq!(card.name, "TestAgent");
    assert_eq!(card.description, "A test agent for unit testing");
    assert_eq!(card.version, "1.0.0");
    assert_eq!(card.url, "http://localhost:8080/");
    assert!(!card.skills.is_empty());
}

#[tokio::test]
async fn test_agent_card_with_capabilities() {
    let agent_config = AgentConfig {
        name: "CapabilityAgent".to_string(),
        description: "Agent with capabilities".to_string(),
        version: "2.0.0".to_string(),
    };
    
    let agent = Agent::new(agent_config);
    let card = agent.get_agent_card("https://example.com".to_string()).await;
    
    // Check that capabilities are properly converted to skills
    assert!(!card.skills.is_empty());
    
    // Check that all required fields are present
    assert!(!card.name.is_empty());
    assert!(!card.description.is_empty());
    assert!(!card.version.is_empty());
    assert!(!card.url.is_empty());
    assert!(!card.default_input_modes.is_empty());
    assert!(!card.default_output_modes.is_empty());
    
    // Check A2A specification compliance
    assert!(card.capabilities.push_notifications.is_some());
    assert!(card.capabilities.state_transition_history.is_some());
    assert!(card.capabilities.streaming.is_some());
}

#[tokio::test]
async fn test_agent_card_serialization() {
    let agent_config = AgentConfig {
        name: "SerializationAgent".to_string(),
        description: "Agent for serialization testing".to_string(),
        version: "3.0.0".to_string(),
    };
    
    let agent = Agent::new(agent_config);
    let card = agent.get_agent_card("http://localhost:3000".to_string()).await;
    
    // Test JSON serialization
    let json = serde_json::to_string(&card).unwrap();
    assert!(!json.is_empty());
    
    // Test JSON deserialization
    let deserialized: AgentCard = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.name, card.name);
    assert_eq!(deserialized.description, card.description);
    assert_eq!(deserialized.version, card.version);
    assert_eq!(deserialized.url, card.url);
}

#[tokio::test]
async fn test_agent_card_skill_generation() {
    let agent_config = AgentConfig {
        name: "SkillAgent".to_string(),
        description: "Agent for skill testing".to_string(),
        version: "4.0.0".to_string(),
    };
    
    let agent = Agent::new(agent_config);
    let card = agent.get_agent_card("http://localhost:4000".to_string()).await;
    
    // Check that skills are generated correctly from capabilities
    for skill in &card.skills {
        assert!(!skill.id.is_empty());
        assert!(!skill.name.is_empty());
        assert!(!skill.description.is_empty());
        assert!(!skill.tags.is_empty());
    }
}
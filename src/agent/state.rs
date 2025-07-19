use std::collections::VecDeque;
use uuid::Uuid;

use crate::protocol::message::Message;

#[derive(Debug, Clone, PartialEq)]
pub enum AgentState {
    Idle,
    Processing,
    WaitingForTool,
    WaitingForAgent,
    Error(String),
}

#[derive(Debug)]
pub struct StateMachine {
    pub state: AgentState,
    pub context: VecDeque<Message>,
    pub max_context_size: usize,
}

impl StateMachine {
    pub fn new(max_context_size: usize) -> Self {
        Self {
            state: AgentState::Idle,
            context: VecDeque::with_capacity(max_context_size),
            max_context_size,
        }
    }

    pub fn transition(&mut self, new_state: AgentState) {
        tracing::info!("State transition: {:?} -> {:?}", self.state, new_state);
        self.state = new_state;
    }

    pub fn add_message(&mut self, message: Message) {
        if self.context.len() >= self.max_context_size {
            self.context.pop_front();
        }
        self.context.push_back(message);
    }

    pub fn get_context(&self) -> Vec<Message> {
        self.context.iter().cloned().collect()
    }

    pub fn get_last_message(&self) -> Option<&Message> {
        self.context.back()
    }

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn is_ready(&self) -> bool {
        matches!(self.state, AgentState::Idle)
    }

    pub fn get_state(&self) -> &AgentState {
        &self.state
    }

    pub fn get_session_id(&self) -> Uuid {
        // Use the ID of the first message or generate a new one
        self.context
            .front()
            .map(|msg| msg.id)
            .unwrap_or_else(Uuid::new_v4)
    }
}
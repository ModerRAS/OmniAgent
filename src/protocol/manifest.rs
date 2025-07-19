use crate::a2a::client::A2AManifest;
use crate::mcp::client::MCPManifest;

#[derive(Debug, Clone)]
pub enum Manifest {
    MCP(MCPManifest),
    A2A(A2AManifest),
}

impl Manifest {
    pub fn name(&self) -> &str {
        match self {
            Manifest::MCP(m) => &m.name,
            Manifest::A2A(m) => &m.name,
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Manifest::MCP(m) => &m.description,
            Manifest::A2A(m) => &m.description,
        }
    }

    pub fn capabilities(&self) -> &Vec<String> {
        match self {
            Manifest::MCP(m) => &m.capabilities,
            Manifest::A2A(m) => &m.capabilities,
        }
    }
}
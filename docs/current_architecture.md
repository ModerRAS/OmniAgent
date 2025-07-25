# Current OmniAgent Architecture

## Overview

The current implementation of OmniAgent provides a basic framework for building agents that can act as A2A servers and connect to external MCP/A2A services. However, the integration between components is not fully realized.

## Architecture Diagram

```mermaid
graph TD
    subgraph "OmniAgent Application"
        A[A2A Server<br/>HTTP REST API] --> B[Main Application]
        C[MCP Client<br/>HTTP Client] --> B
        D[A2A Client<br/>HTTP Client] --> B
        B[Main Application<br/>Agent Core] --> E[LLM Service]
        B --> F[Configuration Manager]
    end
    
    subgraph "External Services"
        G[MCP Server] <-- HTTP --> C
        H[A2A Server] <-- HTTP --> D
        I[LLM Providers<br/>OpenAI/Claude/Google] <-- HTTP --> E
    end
    
    J[User/Client] <-- HTTP --> A

    style A fill:#cde4ff,stroke:#6495ED,stroke-width:2px
    style B fill:#cde4ff,stroke:#6495ED,stroke-width:2px
    style C fill:#cde4ff,stroke:#6495ED,stroke-width:2px
    style D fill:#cde4ff,stroke:#6495ED,stroke-width:2px
    style E fill:#cde4ff,stroke:#6495ED,stroke-width:2px
    style F fill:#cde4ff,stroke:#6495ED,stroke-width:2px
    style G fill:#ffe4b5,stroke:#ffa500,stroke-width:2px
    style H fill:#ffe4b5,stroke:#ffa500,stroke-width:2px
    style I fill:#ffe4b5,stroke:#ffa500,stroke-width:2px
    style J fill:#98fb98,stroke:#32cd32,stroke-width:2px
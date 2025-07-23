# OmniAgent Documentation Fix Requirements

## Introduction

This specification addresses critical documentation errors and inconsistencies in the OmniAgent project. The project contains multiple documentation files with outdated examples, incorrect API usage patterns, inconsistent naming conventions, and mismatched configuration structures. These issues need to be resolved to provide developers with accurate guidance and correct project understanding.

The documentation fixes will ensure that all code examples compile and run correctly, configuration examples match actual implementation, API documentation aligns with server endpoints, and naming conventions are consistent across all documentation files.

## Requirements

### 1. Code Example Accuracy

**User Story**: As a developer using OmniAgent, I want all code examples in the documentation to be accurate and compile without errors, so that I can successfully implement and integrate the framework without encountering compilation or runtime issues.

**Acceptance Criteria**:
1.1 WHEN a developer follows code examples in README.md, THEN all examples MUST successfully compile and run without errors
1.2 WHEN a developer uses provided API patterns, THEN response handling MUST match actual MessageContent enum structure
1.3 WHEN a developer references configuration examples, THEN JSON structure MUST match actual AppConfig struct definition
1.4 IF code examples show response.content usage, THEN they MUST correctly handle MessageContent::Text variant with proper pattern matching
1.5 WHEN examples use external dependencies, THEN Cargo.toml examples MUST include all required dependencies with correct versions

### 2. Naming Convention Consistency

**User Story**: As a developer reading documentation, I want consistent naming conventions across all documentation files, so that I can understand the project structure without confusion.

**Acceptance Criteria**:
2.1 WHEN documentation references the project name, THEN it MUST consistently use "OmniAgent" in titles and "omni-agent" in code/file references
2.2 WHEN configuration examples are provided, THEN they MUST use correct field names that match actual struct definitions
2.3 WHEN API endpoints are documented, THEN they MUST match actual server implementation routes
2.4 IF multiple language versions exist, THEN naming conventions MUST be consistent across all language variants
2.5 WHEN referencing module names, THEN they MUST match actual directory and file names in src/

### 3. Configuration File Accuracy

**User Story**: As a developer configuring OmniAgent, I want accurate configuration file examples, so that I can correctly set up the application without trial and error.

**Acceptance Criteria**:
3.1 WHEN configuration examples show MCP servers, THEN they MUST use standard MCP configuration format: `{"mcpServers": {"server-name": {"command": "uvx", "args": ["mcp-server-name"], "env": {}, "disabled": false, "autoApprove": []}}}`
3.2 WHEN configuration examples show A2A servers, THEN they MUST use HashMap<String, A2AServerConfig> structure format
3.3 WHEN environment variable documentation is provided, THEN it MUST match actual override_with_env() implementation
3.4 IF default values are mentioned, THEN they MUST match AppConfig's Default trait implementation
3.5 WHEN configuration file paths are referenced, THEN they MUST match actual file locations used by the application

### 4. HTTP API Documentation Accuracy

**User Story**: As a developer using the HTTP API, I want accurate endpoint documentation, so that I can successfully call the running server APIs.

**Acceptance Criteria**:
4.1 WHEN API endpoint documentation shows available routes, THEN they MUST match actual Router configuration in main.rs
4.2 WHEN request/response examples are provided, THEN they MUST use correct UserRequest and AgentResponse structures
4.3 WHEN curl examples are shown, THEN they MUST target correct endpoints (/health, /info, /chat)
4.4 IF message format examples are provided, THEN they MUST match actual message handling implementation
4.5 WHEN HTTP status codes are documented, THEN they MUST match actual server responses

### 5. LLM Provider Usage Examples

**User Story**: As a developer using LLM providers, I want accurate provider usage examples, so that I can correctly integrate different LLM services.

**Acceptance Criteria**:
5.1 WHEN LLM provider examples are shown, THEN they MUST use correct provider configuration structures
5.2 WHEN API key configuration is documented, THEN it MUST match actual environment variable handling
5.3 WHEN mock mode usage is explained, THEN it MUST accurately reflect use_mock configuration option
5.4 IF specific provider examples are given, THEN they MUST use correct model names and API patterns
5.5 WHEN provider initialization examples are provided, THEN they MUST match actual LLMManager implementation

### 6. Standard MCP Protocol Support

**User Story**: As a developer using standard MCP protocol, I want the project to use standard MCP configuration format, so that I can seamlessly integrate with other MCP-compatible tools and services.

**Acceptance Criteria**:
6.1 WHEN configuring MCP servers, THEN the system MUST support standard MCP configuration format: `{"mcpServers": {"server-name": {"command": "uvx", "args": ["mcp-server-name"], "env": {}, "disabled": false, "autoApprove": []}}}`
6.2 WHEN loading MCP configuration, THEN the system MUST be able to parse command, args, env, disabled, and autoApprove fields
6.3 WHEN documentation shows MCP configuration examples, THEN they MUST use standard format instead of custom HashMap structure
6.4 IF existing code uses non-standard format, THEN it MUST be refactored to support standard MCP configuration format
6.5 WHEN MCP servers are initialized, THEN configuration validation MUST ensure all required fields are present

### 7. Project Structure Documentation

**User Story**: As a developer reading project documentation, I want accurate project structure information, so that I can understand the codebase organization and locate relevant files.

**Acceptance Criteria**:
7.1 WHEN project structure is documented, THEN it MUST reflect actual src/ directory organization
7.2 WHEN module descriptions are provided, THEN they MUST accurately describe implemented functionality
7.3 WHEN file paths are referenced, THEN they MUST exist in actual project structure
7.4 IF build/run commands are documented, THEN they MUST be compatible with current Cargo.toml configuration
7.5 WHEN dependency information is provided, THEN it MUST match actual Cargo.toml dependencies

## Technical Constraints

- All documentation changes must maintain backward compatibility with existing working configurations
- Documentation updates should not break existing valid configurations
- Examples should work with the current version of the codebase
- All code examples must be tested to ensure they compile and run correctly
- Configuration examples must validate against actual struct definitions
- API documentation must match current server implementation exactly

## Success Criteria

- All code examples in documentation compile without errors
- Configuration examples match actual AppConfig structure exactly
- API endpoint documentation matches server routes exactly
- Naming conventions are consistent across all documentation
- Standard MCP configuration format is fully supported
- All documentation files are updated simultaneously to maintain consistency
- No breaking changes are introduced to existing valid configurations
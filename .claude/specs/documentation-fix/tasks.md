# OmniAgent Documentation Fix Implementation Plan

## 1. Audit and Inventory
- [ ] 1.1 Scan project directory for all documentation files (.md, .json, .txt)
- [ ] 1.2 Create inventory of all code examples in documentation
- [ ] 1.3 Extract actual AppConfig structure from src/config.rs
- [ ] 1.4 Extract actual API routes from src/server/mod.rs and main.rs
- [ ] 1.5 Extract MessageContent enum from src/protocol/message.rs
- [ ] 1.6 Extract LLM provider configurations from src/llm/
- [ ] 1.7 Document current MCP configuration format vs standard format
- [ ] 1.8 Create mapping of all naming inconsistencies found

## 2. Critical Code Example Fixes
- [ ] 2.1 Fix README.md code examples to use correct MessageContent handling
- [ ] 2.2 Update configuration examples to match actual AppConfig structure
- [ ] 2.3 Fix API usage examples to use correct UserRequest/AgentResponse structures
- [ ] 2.4 Update LLM provider examples with correct initialization patterns
- [ ] 2.5 Ensure all code examples include necessary Cargo.toml dependencies
- [ ] 2.6 Test all code examples for compilation success

## 3. Configuration Structure Updates
- [ ] 3.1 Update config.example.json to match actual AppConfig structure
- [ ] 3.2 Create standard MCP configuration template file
- [ ] 3.3 Update A2A server configuration examples to use HashMap format
- [ ] 3.4 Document all environment variables with correct override behavior
- [ ] 3.5 Ensure all default values match AppConfig Default trait implementation
- [ ] 3.6 Create configuration validation script

## 4. API Documentation Corrections
- [ ] 4.1 Update all API endpoint documentation to match actual routes (/health, /info, /chat)
- [ ] 4.2 Fix request/response examples to use correct JSON structures
- [ ] 4.3 Update curl examples with correct endpoints and headers
- [ ] 4.4 Document correct HTTP status codes for each endpoint
- [ ] 4.5 Update message format documentation to match implementation
- [ ] 4.6 Create API testing script using documented examples

## 5. Naming Convention Standardization
- [ ] 5.1 Replace all inconsistent project name references with "OmniAgent"/"omni-agent"
- [ ] 5.2 Update configuration field names to match actual struct fields
- [ ] 5.3 Standardize module and file path references
- [ ] 5.4 Ensure consistent naming across all documentation files
- [ ] 5.5 Update code examples to use consistent variable naming
- [ ] 5.6 Create naming convention reference document

## 6. MCP Format Standardization
- [ ] 6.1 Update all MCP configuration examples to use standard format
- [ ] 6.2 Create migration guide from old to new MCP format
- [ ] 6.3 Update MCP server initialization documentation
- [ ] 6.4 Document standard MCP fields (command, args, env, disabled, autoApprove)
- [ ] 6.5 Create MCP configuration validation script
- [ ] 6.6 Update any code examples using non-standard MCP format

## 7. Project Structure Documentation
- [ ] 7.1 Update project structure diagram to reflect actual src/ directory
- [ ] 7.2 Correct module descriptions to match actual functionality
- [ ] 7.3 Verify all file paths referenced in documentation exist
- [ ] 7.4 Update build and run commands to match current Cargo.toml
- [ ] 7.5 Document correct dependency versions from Cargo.toml
- [ ] 7.6 Create project structure validation script

## 8. Validation and Testing
- [ ] 8.1 Create automated script to test all code examples
- [ ] 8.2 Implement configuration validation against actual structs
- [ ] 8.3 Test all API endpoints with documented examples
- [ ] 8.4 Validate MCP configuration format compliance
- [ ] 8.5 Run naming consistency checks across all files
- [ ] 8.6 Create integration test for complete documentation accuracy

## 9. Final Review and Documentation
- [ ] 9.1 Perform complete documentation walkthrough
- [ ] 9.2 Create documentation maintenance checklist
- [ ] 9.3 Document any breaking changes for existing users
- [ ] 9.4 Create contribution guidelines for documentation updates
- [ ] 9.5 Set up automated documentation validation in CI
- [ ] 9.6 Generate final validation report
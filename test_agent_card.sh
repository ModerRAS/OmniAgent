#!/bin/bash

# Agent Card Test Script for OmniAgent A2A Server
# This script tests the agent card functionality and validates A2A compliance

set -e

# Configuration
SERVER_PORT=${PORT:-8080}
SERVER_URL="http://localhost:${SERVER_PORT}"
CONFIG_FILE="${OMNI_AGENT_CONFIG:-config.json}"
MAX_WAIT_TIME=30

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check dependencies
print_info "Checking dependencies..."
command -v curl >/dev/null 2>&1 || { print_error "curl is required but not installed"; exit 1; }
command -v jq >/dev/null 2>&1 || { print_warning "jq not found - JSON output will not be formatted"; }

# Create test configuration with placeholder for models
create_test_config() {
    print_info "Creating test configuration..."
    cat > "${CONFIG_FILE}" << EOF
{
  "server": {
    "port": ${SERVER_PORT},
    "host": "localhost"
  },
  "llm": {
    "mock": true,
    "providers": {
      "openai": {
        "enabled": false,
        "api_key": "",
        "model": "gpt-3.5-turbo"
      },
      "claude": {
        "enabled": false,
        "api_key": "",
        "model": "claude-3-haiku"
      },
      "google": {
        "enabled": false,
        "api_key": "",
        "model": "gemini-pro"
      }
    }
  },
  "mcp": {
    "enabled": true,
    "servers": []
  },
  "a2a": {
    "enabled": true,
    "agents": []
  }
}
EOF
    print_success "Test configuration created at ${CONFIG_FILE}"
}

# Start the server
start_server() {
    print_info "Starting OmniAgent A2A server..."
    
    # Kill any existing process
    pkill -f "omni-agent" || true
    sleep 2
    
    # Build and start
    cargo build --release --quiet
    cargo run --release -- --config "${CONFIG_FILE}" > server.log 2>&1 &
    SERVER_PID=$!
    
    print_info "Server started with PID: ${SERVER_PID}"
    
    # Wait for server to be ready
    print_info "Waiting for server to start..."
    local wait_count=0
    while [ $wait_count -lt $MAX_WAIT_TIME ]; do
        if curl -s "${SERVER_URL}/health" >/dev/null 2>&1; then
            print_success "Server is ready"
            return 0
        fi
        sleep 1
        ((wait_count++))
    done
    
    print_error "Server failed to start within ${MAX_WAIT_TIME} seconds"
    cat server.log
    return 1
}

# Test functions
test_agent_card_structure() {
    print_info "Testing agent card structure..."
    
    local response=$(curl -s -w "\n%{http_code}" "${SERVER_URL}/agent.json")
    local http_code=$(echo "$response" | tail -n1)
    local json_data=$(echo "$response" | head -n-1)
    
    if [ "$http_code" != "200" ]; then
        print_error "Agent card endpoint returned HTTP $http_code"
        return 1
    fi
    
    # Validate required fields
    local required_fields=("name" "description" "version" "url" "capabilities" "skills")
    for field in "${required_fields[@]}"; do
        if ! echo "$json_data" | jq -e ".$field" >/dev/null 2>&1; then
            print_error "Missing required field: $field"
            return 1
        fi
    done
    
    # Validate capabilities structure
    if ! echo "$json_data" | jq -e ".capabilities.streaming" >/dev/null 2>&1; then
        print_error "Missing capabilities.streaming"
        return 1
    fi
    
    # Validate skills array
    local skills_count=$(echo "$json_data" | jq '.skills | length')
    if [ "$skills_count" -lt 1 ]; then
        print_error "Agent card should have at least one skill"
        return 1
    fi
    
    print_success "Agent card structure is valid"
    
    # Display formatted output
    if command -v jq >/dev/null 2>&1; then
        echo "$json_data" | jq '.'
    else
        echo "$json_data"
    fi
    
    return 0
}

test_agent_card_content() {
    print_info "Testing agent card content..."
    
    local json_data=$(curl -s "${SERVER_URL}/agent.json")
    
    # Test specific content
    local name=$(echo "$json_data" | jq -r '.name')
    local description=$(echo "$json_data" | jq -r '.description')
    local version=$(echo "$json_data" | jq -r '.version')
    local url=$(echo "$json_data" | jq -r '.url')
    
    if [ "$name" != "OmniAgent" ]; then
        print_error "Expected name 'OmniAgent', got '$name'"
        return 1
    fi
    
    if [ -z "$description" ]; then
        print_error "Description should not be empty"
        return 1
    fi
    
    if [ -z "$version" ]; then
        print_error "Version should not be empty"
        return 1
    fi
    
    if [ "$url" != "http://localhost:${SERVER_PORT}" ]; then
        print_warning "URL mismatch: expected 'http://localhost:${SERVER_PORT}', got '$url'"
    fi
    
    print_success "Agent card content is valid"
    return 0
}

test_all_endpoints() {
    print_info "Testing all A2A endpoints..."
    
    local endpoints=(
        "/health"
        "/manifest"
        "/agent.json"
        "/"
    )
    
    for endpoint in "${endpoints[@]}"; do
        local response=$(curl -s -w "\n%{http_code}" "${SERVER_URL}${endpoint}")
        local http_code=$(echo "$response" | tail -n1)
        
        if [ "$http_code" == "200" ]; then
            print_success "✓ ${endpoint}"
        else
            print_error "✗ ${endpoint} (HTTP $http_code)"
            return 1
        fi
    done
    
    return 0
}

test_message_handling() {
    print_info "Testing message handling..."
    
    local test_message='{"sender": "test-client", "recipient": "OmniAgent", "content": {"type": "text", "text": "Hello OmniAgent"}}'
    local response=$(curl -s -X POST "${SERVER_URL}/messages" \
        -H "Content-Type: application/json" \
        -d "$test_message" \
        -w "\n%{http_code}")
    
    local http_code=$(echo "$response" | tail -n1)
    local json_data=$(echo "$response" | head -n-1)
    
    if [ "$http_code" != "200" ]; then
        print_error "Message endpoint returned HTTP $http_code"
        return 1
    fi
    
    if ! echo "$json_data" | jq -e '.content.text' >/dev/null 2>&1; then
        print_error "Invalid message response format"
        return 1
    fi
    
    print_success "Message handling works correctly"
    return 0
}

# Main test runner
main() {
    print_info "Starting OmniAgent Agent Card Tests..."
    echo "=========================================="
    
    create_test_config
    
    if ! start_server; then
        print_error "Failed to start server"
        exit 1
    fi
    
    sleep 2  # Give server time to fully initialize
    
    local tests_passed=0
    local tests_total=0
    
    # Run tests
    local tests=(
        "test_all_endpoints"
        "test_agent_card_structure"
        "test_agent_card_content"
        "test_message_handling"
    )
    
    for test in "${tests[@]}"; do
        ((tests_total++))
        if $test; then
            ((tests_passed++))
        else
            print_error "Test failed: $test"
        fi
        echo "------------------------------------------"
    done
    
    # Cleanup
    print_info "Cleaning up..."
    if [ -n "$SERVER_PID" ]; then
        kill $SERVER_PID 2>/dev/null || true
    fi
    
    # Results
    echo "=========================================="
    print_info "Test Results:"
    print_info "Tests passed: $tests_passed/$tests_total"
    
    if [ $tests_passed -eq $tests_total ]; then
        print_success "All tests passed! 🎉"
        exit 0
    else
        print_error "Some tests failed. Check server.log for details."
        exit 1
    fi
}

# Handle script interruption
trap 'print_error "Test interrupted"; if [ -n "$SERVER_PID" ]; then kill $SERVER_PID 2>/dev/null || true; fi; exit 1' INT TERM

# Run main function
main "$@"
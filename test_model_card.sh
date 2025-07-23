#!/bin/bash

# Model Card Test Script for OmniAgent A2A Server
# Tests the agent card functionality and validates A2A compliance

set -e

# Configuration
SERVER_PORT=8080
SERVER_URL="http://localhost:${SERVER_PORT}"
MAX_WAIT_TIME=30

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
print_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
print_error() { echo -e "${RED}[ERROR]${NC} $1"; }
print_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }

# Check dependencies
check_dependencies() {
    print_info "Checking dependencies..."
    command -v curl >/dev/null 2>&1 || { print_error "curl is required"; exit 1; }
    command -v jq >/dev/null 2>&1 || { print_warning "jq not found - JSON will not be formatted"; }
}

# Create test configuration
create_test_config() {
    print_info "Creating test configuration..."
    cat > test_config.json << EOF
{
  "server": {
    "port": ${SERVER_PORT},
    "host": "127.0.0.1",
    "cors_origins": ["*"]
  },
  "llm": {
    "provider": "mock",
    "model": "gpt-3.5-turbo",
    "api_key": "test-key",
    "base_url": null,
    "temperature": 0.7,
    "max_tokens": 1000,
    "use_mock": true
  },
  "mcp": {
    "enabled": true,
    "servers": {}
  },
  "a2a": {
    "enabled": true,
    "servers": {},
    "allow_external": true
  },
  "logging": {
    "level": "info",
    "format": "json",
    "file": null
  }
}
EOF
    print_success "Test configuration created"
}

# Start server
start_server() {
    print_info "Starting A2A server..."
    
    # Kill any existing server
    pkill -f omni-agent || true
    sleep 2
    
    # Build and start
    cargo build --release --quiet
    RUST_LOG=info cargo run --release -- --config test_config.json > a2a_test.log 2>&1 &
echo $! > server.pid
    
    SERVER_PID=$(cat server.pid)
    print_info "Server started with PID: $SERVER_PID"
    
    # Wait for server
    for i in {1..$MAX_WAIT_TIME}; do
        if curl -s "${SERVER_URL}/health" >/dev/null 2>&1; then
            print_success "Server is ready!"
            return 0
        fi
        sleep 1
    done
    
    print_error "Server failed to start"
    cat a2a_test.log
    return 1
}

# Test agent card structure
test_agent_card_structure() {
    print_info "Testing agent card structure..."
    
    local response=$(curl -s -w "\n%{http_code}" "${SERVER_URL}/agent.json")
    local http_code=$(echo "$response" | tail -n1)
    local json_data=$(echo "$response" | head -n-1)
    
    if [ "$http_code" != "200" ]; then
        print_error "HTTP $http_code from /agent.json"
        return 1
    fi
    
    # Check required fields
    local required_fields=("name" "description" "version" "url" "capabilities" "skills" "defaultInputModes" "defaultOutputModes")
    for field in "${required_fields[@]}"; do
        if ! echo "$json_data" | jq -e ".$field" >/dev/null 2>&1; then
            print_error "Missing required field: $field"
            return 1
        fi
    done
    
    print_success "Agent card structure valid"
    return 0
}

# Test agent card content
test_agent_card_content() {
    print_info "Testing agent card content..."
    
    local json_data=$(curl -s "${SERVER_URL}/agent.json")
    
    # Validate specific content
    local name=$(echo "$json_data" | jq -r '.name')
    local description=$(echo "$json_data" | jq -r '.description')
    local version=$(echo "$json_data" | jq -r '.version')
    local url=$(echo "$json_data" | jq -r '.url')
    
    [ "$name" != "null" ] && [ -n "$name" ] || { print_error "Name is empty"; return 1; }
    [ "$description" != "null" ] && [ -n "$description" ] || { print_error "Description is empty"; return 1; }
    [ "$version" != "null" ] && [ -n "$version" ] || { print_error "Version is empty"; return 1; }
    [ "$url" != "null" ] && [ -n "$url" ] || { print_error "URL is empty"; return 1; }
    
    print_success "Agent card content valid"
    return 0
}

# Test all endpoints
test_all_endpoints() {
    print_info "Testing all A2A endpoints..."
    
    local endpoints=(
        "/health"
        "/info"
        "/agent.json"
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

# Test agent card validation
test_agent_card_validation() {
    print_info "Validating agent card against A2A spec..."
    
    local json_data=$(curl -s "${SERVER_URL}/agent.json")
    
    # Test JSON schema validation
    python3 validate_agent_card.py --url "${SERVER_URL}" --verbose
    
    return $?
}

# Display agent card
display_agent_card() {
    print_info "Displaying agent card..."
    curl -s "${SERVER_URL}/agent.json" | {
        if command -v jq >/dev/null 2>&1; then
            jq '.'
        else
            cat
        fi
    }
}

# Cleanup
cleanup() {
    print_info "Cleaning up..."
    if [ -f server.pid ]; then
        local pid=$(cat server.pid)
        kill $pid 2>/dev/null || true
        rm -f server.pid
    fi
    rm -f test_config.json
    rm -f a2a_test.log
}

# Main test function
main() {
    print_info "Starting Model Card Tests..."
    echo "=========================================="
    
    # Set trap for cleanup
    trap cleanup EXIT
    
    check_dependencies
    create_test_config
    
    if ! start_server; then
        print_error "Failed to start server"
        exit 1
    fi
    
    local tests_passed=0
    local tests_total=4
    
    # Run tests
    test_agent_card_structure && ((tests_passed++)) || true
    test_agent_card_content && ((tests_passed++)) || true
    test_all_endpoints && ((tests_passed++)) || true
    test_agent_card_validation && ((tests_passed++)) || true
    
    # Display results
    echo "=========================================="
    print_info "Test Results: $tests_passed/$tests_total tests passed"
    
    if [ $tests_passed -eq $tests_total ]; then
        print_success "🎉 All tests passed!"
        display_agent_card
        exit 0
    else
        print_error "❌ Some tests failed"
        exit 1
    fi
}

# Run main function
main "$@"
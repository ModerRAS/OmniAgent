#!/bin/bash

# A2A Server Runner for OmniAgent
# This script starts the A2A server with proper agent card support

set -e

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

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Kill any existing server
pkill -f omni-agent || true
sleep 2

print_info "Starting A2A Server..."

# Build the project
print_info "Building project..."
cargo build --release --quiet

# Start the A2A server with mock mode
print_info "Starting A2A server on port 8080..."
RUST_LOG=info cargo run --release -- --mock --port 8080 > a2a_server.log 2>&1 &
echo $! > a2a_server.pid

SERVER_PID=$(cat a2a_server.pid)
print_info "Server started with PID: $SERVER_PID"

# Wait for server to be ready
print_info "Waiting for server to start..."
for i in {1..30}; do
    if curl -s http://localhost:8080/health >/dev/null 2>&1; then
        print_success "A2A Server is ready!"
        break
    fi
    if [ $i -eq 30 ]; then
        print_error "Server failed to start within 30 seconds"
        cat a2a_server.log
        exit 1
    fi
    sleep 1
done

print_success "✅ A2A Server is running on http://localhost:8080"
print_info "Available endpoints:"
print_info "  - GET /health - Health check"
print_info "  - GET /info - Agent information"
print_info "  - GET /agent.json - Agent Card (A2A specification)"
print_info "  - GET /manifest - Agent capabilities"
print_info "  - POST /messages - Send A2A messages"
print_info "  - GET /messages/:id - Get message by ID"

print_info ""
print_info "Testing agent card..."
curl -s http://localhost:8080/agent.json | jq '.'

print_info ""
print_info "To stop the server, run: kill $SERVER_PID"
echo "Server PID: $SERVER_PID"
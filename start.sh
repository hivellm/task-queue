#!/bin/bash

# Task Queue Start Script
# This script starts both HTTP and MCP servers

set -e

echo "🚀 Starting Task Queue with MCP..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to check if port is in use
check_port() {
    local port=$1
    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null ; then
        echo -e "${RED}❌ Port $port is already in use${NC}"
        return 0
    else
        echo -e "${GREEN}✅ Port $port is free${NC}"
        return 1
    fi
}

# Function to kill process using port
kill_process_on_port() {
    local port=$1
    local pid=$(lsof -ti:$port)
    if [ ! -z "$pid" ]; then
        echo -e "${YELLOW}⚠️  Killing process $pid using port $port${NC}"
        kill -9 $pid 2>/dev/null || true
        sleep 2
    fi
}

# Function to wait for port to be free
wait_for_port() {
    local port=$1
    local max_attempts=10
    local attempt=1

    while [ $attempt -le $max_attempts ]; do
        if ! check_port $port; then
            echo -e "${GREEN}✅ Port $port is now free${NC}"
            return 0
        fi

        echo -e "${YELLOW}⏳ Waiting for port $port to be free (attempt $attempt/$max_attempts)...${NC}"
        kill_process_on_port $port
        sleep 1
        ((attempt++))
    done

    echo -e "${RED}❌ Failed to free port $port after $max_attempts attempts${NC}"
    return 1
}

# Kill any existing task-queue processes
echo -e "${BLUE}🔍 Checking for existing task-queue processes...${NC}"
if pgrep -f "task-queue" >/dev/null; then
    echo -e "${YELLOW}⚠️  Killing existing task-queue processes...${NC}"
    pkill -f "task-queue" 2>/dev/null || true
    sleep 2
fi

# Check and free ports
echo -e "${BLUE}🔍 Checking ports 16080 and 16081...${NC}"
wait_for_port 16080
wait_for_port 16081

# Create necessary directories
echo -e "${BLUE}📁 Creating necessary directories...${NC}"
mkdir -p data logs

# Create config file if it doesn't exist
if [ ! -f "config.yml" ]; then
    echo -e "${BLUE}📝 Creating config.yml...${NC}"
    cat > config.yml << 'EOF'
server:
  host: "0.0.0.0"
  port: 16080
  grpc_port: 16081
  mcp_port: 16082

storage:
  database_path: "./data/task-queue.db"
  backup_interval: "1h"
  retention_days: 30

vectorizer:
  endpoint: "http://localhost:15002"
  collection: "task-interactions"
  auto_index: false

execution:
  max_concurrent_tasks: 10
  default_timeout: "5m"
  retry_attempts: 3
  retry_delay: "1s"

monitoring:
  metrics_enabled: true
  metrics_port: 9090
  health_check_interval: "30s"
EOF
fi

# Build the project
echo -e "${BLUE}🔨 Building project...${NC}"
if ! cargo build --release; then
    echo -e "${RED}❌ Build failed${NC}"
    exit 1
fi

# Verify binary exists
if [ ! -f "./target/release/task-queue" ]; then
    echo -e "${RED}❌ Binary not found at ./target/release/task-queue${NC}"
    exit 1
fi

# Start the server
echo -e "${GREEN}🚀 Starting Task Queue server with MCP integration...${NC}"
echo -e "${BLUE}📋 HTTP API will be available at: http://localhost:16080${NC}"
echo -e "${BLUE}📋 MCP endpoint will be available at: http://localhost:16080/mcp${NC}"
echo ""

# Run the server and capture output
./target/release/task-queue 2>&1 &
SERVER_PID=$!

echo -e "${BLUE}📋 Server PID: $SERVER_PID${NC}"

# Wait a bit for servers to start
sleep 3

# Check if servers are running
if kill -0 $SERVER_PID 2>/dev/null; then
    echo -e "${GREEN}✅ Task Queue server is running (PID: $SERVER_PID)${NC}"

    # Check if ports are actually listening
    if lsof -Pi :16080 -sTCP:LISTEN -t >/dev/null; then
        echo -e "${GREEN}✅ HTTP API is listening on port 16080${NC}"
    else
        echo -e "${RED}❌ HTTP API is NOT listening on port 16080${NC}"
    fi

    if lsof -Pi :16081 -sTCP:LISTEN -t >/dev/null; then
        echo -e "${GREEN}✅ MCP WebSocket is listening on port 16081${NC}"
    else
        echo -e "${RED}❌ MCP WebSocket is NOT listening on port 16081${NC}"
        echo -e "${YELLOW}⚠️  Checking what is using port 16081...${NC}"
        lsof -i :16081 || echo "Nothing found using port 16081"
    fi

    echo -e "${BLUE}📋 Press Ctrl+C to stop the server${NC}"
    wait $SERVER_PID
else
    echo -e "${RED}❌ Server failed to start${NC}"
    exit 1
fi

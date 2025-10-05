#!/bin/bash

# Task Queue Stop Script
# This script stops both HTTP and MCP servers

set -e

echo "🛑 Stopping Task Queue with MCP..."

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
        echo -e "${RED}❌ Port $port is still in use${NC}"
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

# Function to stop task-queue processes gracefully
stop_task_queue_processes() {
    echo -e "${BLUE}🔍 Looking for task-queue processes...${NC}"
    
    # Find all task-queue processes
    local pids=$(pgrep -f "task-queue" 2>/dev/null || true)
    
    if [ -z "$pids" ]; then
        echo -e "${YELLOW}⚠️  No task-queue processes found${NC}"
        return 0
    fi
    
    echo -e "${BLUE}📋 Found task-queue processes: $pids${NC}"
    
    # Try graceful shutdown first
    echo -e "${YELLOW}⚠️  Attempting graceful shutdown...${NC}"
    for pid in $pids; do
        if kill -0 $pid 2>/dev/null; then
            echo -e "${BLUE}📋 Sending SIGTERM to process $pid${NC}"
            kill -TERM $pid 2>/dev/null || true
        fi
    done
    
    # Wait a bit for graceful shutdown
    sleep 3
    
    # Check if processes are still running
    local remaining_pids=$(pgrep -f "task-queue" 2>/dev/null || true)
    if [ ! -z "$remaining_pids" ]; then
        echo -e "${YELLOW}⚠️  Some processes still running, forcing shutdown...${NC}"
        for pid in $remaining_pids; do
            if kill -0 $pid 2>/dev/null; then
                echo -e "${BLUE}📋 Sending SIGKILL to process $pid${NC}"
                kill -9 $pid 2>/dev/null || true
            fi
        done
        sleep 2
    fi
    
    # Final check
    local final_pids=$(pgrep -f "task-queue" 2>/dev/null || true)
    if [ -z "$final_pids" ]; then
        echo -e "${GREEN}✅ All task-queue processes stopped${NC}"
        return 0
    else
        echo -e "${RED}❌ Some processes could not be stopped: $final_pids${NC}"
        return 1
    fi
}

# Function to stop processes using specific ports
stop_port_processes() {
    local port=$1
    local pids=$(lsof -ti:$port 2>/dev/null || true)
    
    if [ -z "$pids" ]; then
        echo -e "${GREEN}✅ No processes using port $port${NC}"
        return 0
    fi
    
    echo -e "${BLUE}📋 Found processes using port $port: $pids${NC}"
    
    for pid in $pids; do
        if kill -0 $pid 2>/dev/null; then
            echo -e "${YELLOW}⚠️  Stopping process $pid on port $port${NC}"
            kill -TERM $pid 2>/dev/null || true
            sleep 1
            if kill -0 $pid 2>/dev/null; then
                echo -e "${YELLOW}⚠️  Force killing process $pid on port $port${NC}"
                kill -9 $pid 2>/dev/null || true
            fi
        fi
    done
    
    sleep 1
    if check_port $port; then
        echo -e "${RED}❌ Port $port is still in use${NC}"
        return 1
    else
        echo -e "${GREEN}✅ Port $port is now free${NC}"
        return 0
    fi
}

# Main stop sequence
echo -e "${BLUE}🛑 Stopping Task Queue services...${NC}"

# Stop task-queue processes
stop_task_queue_processes

# Stop processes on specific ports
echo -e "${BLUE}🔍 Checking and stopping processes on ports 16080 and 16081...${NC}"
stop_port_processes 16080
stop_port_processes 16081

# Final verification
echo -e "${BLUE}🔍 Final verification...${NC}"

# Check if any task-queue processes are still running
if pgrep -f "task-queue" >/dev/null; then
    echo -e "${RED}❌ Some task-queue processes are still running${NC}"
    echo -e "${BLUE}📋 Remaining processes:$(pgrep -f "task-queue")${NC}"
    exit 1
else
    echo -e "${GREEN}✅ No task-queue processes running${NC}"
fi

# Check ports
if lsof -Pi :16080 -sTCP:LISTEN -t >/dev/null; then
    echo -e "${RED}❌ Port 16080 is still in use${NC}"
    lsof -i :16080
    exit 1
else
    echo -e "${GREEN}✅ Port 16080 is free${NC}"
fi

if lsof -Pi :16081 -sTCP:LISTEN -t >/dev/null; then
    echo -e "${RED}❌ Port 16081 is still in use${NC}"
    lsof -i :16081
    exit 1
else
    echo -e "${GREEN}✅ Port 16081 is free${NC}"
fi

echo -e "${GREEN}🎉 Task Queue services stopped successfully${NC}"

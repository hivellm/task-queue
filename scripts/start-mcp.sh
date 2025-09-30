#!/bin/bash

# Task Queue MCP Server Startup Script

echo "🚀 Starting Task Queue MCP Server..."

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust first."
    exit 1
fi

# Build the project
echo "📦 Building Task Queue MCP Server..."
cargo build --release --bin task-queue-mcp-server

if [ $? -ne 0 ]; then
    echo "❌ Build failed. Please check the errors above."
    exit 1
fi

# Start the server
echo "✅ Starting MCP server on http://127.0.0.1:15004/sse"
echo "🔗 Connect your MCP client to the above URL"
echo "📊 Dashboard available at: http://127.0.0.1:15004/"

# Run the server
cargo run --release --bin task-queue-mcp-server -- --host 127.0.0.1 --port 15004

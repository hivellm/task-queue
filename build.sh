#!/bin/bash

echo "🚀 Building Task Queue Dashboard and Server..."

# Build the dashboard with Vite
echo "📦 Building dashboard with Vite..."
cd dashboard
npm install
npm run build
cd ..

# Build the Rust server
echo "🔨 Building Rust server..."
cargo build --release

echo "✅ Build complete!"
echo "📁 Dashboard build: dashboard/dist/"
echo "📁 Server binary: target/release/task-queue.exe"

# Multi-stage build for Task Queue
FROM rust:1.75-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo files first for better caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release && rm -rf src

# Copy source code
COPY src ./src
COPY cli ./cli

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd -r taskqueue && useradd -r -g taskqueue taskqueue

# Set working directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/task-queue /app/task-queue
COPY --from=builder /app/target/release/task-queue-mcp-server /app/task-queue-mcp-server

# Copy configuration files
COPY config.yml ./
COPY dashboard ./dashboard

# Create data directory
RUN mkdir -p /app/data && chown -R taskqueue:taskqueue /app

# Switch to non-root user
USER taskqueue

# Expose ports
EXPOSE 16080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:16080/health || exit 1

# Default command
CMD ["./task-queue"]

# Task Queue - Copilot Instructions

## 🤖 **AI Assistant Guidelines for Task Queue Development**

### **Project Overview**
Task Queue is a high-performance task management system built in Rust, designed for complex workflow orchestration with AI model consensus. It features integrated multi-interface architecture (Web Dashboard + REST API + MCP + WebSocket + CLI), and comprehensive project/task tracking to prevent duplication.

## 🏗️ **Architecture Principles**

### **Integrated Multi-Interface Design**
- **SINGLE SERVER**: Unified server handles Web Dashboard, REST API, MCP, and WebSocket
- **MULTI-INTERFACE**: All interfaces share the same business logic and data layer
- **NO GRPC LAYER**: Unlike Vectorizer, Task Queue doesn't use GRPC - everything is integrated
- **RULE**: All new features must be available across all interfaces (Dashboard, REST, MCP, WebSocket)

### **Workflow Management**
- **MANDATORY PHASES**: Planning → Implementation → TestCreation → Testing → AIReview → Finalized
- **STRICT ENFORCEMENT**: Never skip phases or violate workflow requirements
- **DOCUMENTATION FIRST**: Technical documentation required before implementation

### **Project Tracking System**
- **AUTOMATIC**: `.tasks` file created for every project to track task IDs
- **VERIFICATION**: Always check `.tasks` file before creating new tasks
- **PREVENTION**: Avoid duplicate tasks by consulting existing `.tasks` files

## 🧪 **Development Standards**

### **Testing Requirements**
- **EXECUTION MANDATORY**: All tests must be actually run using `cargo test`
- **COVERAGE MINIMUM**: 85% code coverage required for all modules
- **INTEGRATION TESTING**: Test all interfaces (Dashboard, REST, MCP, WebSocket) together
- **VERIFICATION**: Test results must be checked and failures addressed

### **Code Quality**
- **CLIPPY**: All Clippy warnings must be resolved
- **FORMATTING**: Code must be formatted with `rustfmt`
- **DOCUMENTATION**: All public APIs must have comprehensive documentation
- **ERROR HANDLING**: Robust error handling with structured error types

## 🔧 **Implementation Guidelines**

### **Task Creation Protocol**
```rust
// BEFORE creating any task, ALWAYS check existing tasks:
let tasks_file = fs::read_to_string(".tasks").await?;
if tasks_file.contains("task_name") {
    // Task already exists - DO NOT create duplicate
    return Err("Task already exists".into());
}
```

### **Project Creation Requirements**
- **CHECK EXISTING**: Verify no duplicate projects exist
- **CREATE TRACKING**: Automatically generate `.tasks` file
- **DOCUMENTATION**: Include project metadata in tracking file

### **Phase Transition Rules**
- **PLANNING**: Complete technical documentation first
- **IMPLEMENTATION**: Code according to documentation
- **TESTCREATION**: Create comprehensive test suite
- **TESTING**: Execute all tests and verify results
- **AIREVIEW**: Get 3 AI model approvals before completion

## 📁 **File Organization Standards**

### **Source Structure**
```
src/
├── auth/           # Authentication & authorization modules
├── core.rs         # Core business logic and data models
├── server.rs       # Integrated multi-interface server (REST + MCP + WebSocket)
├── mcp.rs          # MCP protocol implementation (SSE transport)
├── storage.rs      # Persistent storage layer (sled database)
├── vectorizer.rs   # Vectorizer integration (optional)
├── websocket.rs    # Real-time WebSocket communication
├── metrics.rs      # Prometheus metrics collection
├── logging.rs      # Structured logging configuration
├── error.rs        # Error types and handling
├── lib.rs          # Library exports and module organization
└── main.rs         # Server entry point
```

### **Configuration Files**
- **config.yml**: Main configuration file
- **.tasks**: Auto-generated task tracking (added to .gitignore)
- **Cargo.toml**: Rust dependencies and build configuration

### **Documentation Structure**
```
docs/
├── API_DOCUMENTATION.md          # REST API specification
├── DEVELOPMENT_WORKFLOW.md       # Development process
├── CRITICAL_TESTING_REQUIREMENTS.md  # Testing guidelines
└── implementations/              # Technical specifications
```

## 🔐 **Security & Authentication**

### **API Key Management**
- **VALIDATION**: All external requests require valid API keys
- **PERMISSIONS**: Role-based access control for operations
- **STORAGE**: Secure API key storage with encryption

### **Rate Limiting**
- **IMPLEMENTATION**: Request rate limiting on all endpoints
- **CONFIGURATION**: Configurable limits per user/endpoint
- **MONITORING**: Rate limit violations logged and monitored

## 📊 **Monitoring & Observability**

### **Metrics Collection**
- **PROMETHEUS**: All operations instrumented with metrics
- **ENDPOINTS**: `/metrics` endpoint for monitoring systems
- **DASHBOARD**: Grafana integration for visualization

### **Logging Standards**
- **STRUCTURED**: Use tracing crate for structured logging
- **LEVELS**: ERROR, WARN, INFO, DEBUG, TRACE levels
- **FILES**: Logs written to `logs/task-queue.log`

## 🚀 **Deployment & Production**

### **Container Strategy**
- **MULTI-STAGE**: Development and production Docker builds
- **COMPOSE**: Docker Compose for local development
- **KUBERNETES**: Production deployment manifests

### **Configuration Management**
- **VALIDATION**: Configuration validated on startup
- **ENVIRONMENT**: Environment variable support for secrets
- **DEFAULTS**: Sensible defaults for all configuration options

## 🔗 **Integration Patterns**

### **Vectorizer Integration**
- **OPTIONAL**: Vectorizer integration is non-blocking
- **GRACEFUL DEGRADATION**: System works without vectorizer
- **ASYNC OPERATIONS**: All vectorizer calls are asynchronous

### **External Services**
- **RETRY LOGIC**: Automatic retry for failed external calls
- **TIMEOUTS**: Configurable timeouts for all operations
- **CIRCUIT BREAKER**: Fault tolerance with circuit breaker pattern

## 🐛 **Bug Fixing Protocol**

### **Issue Resolution**
- **PRIORITY**: Critical bugs fixed immediately
- **TESTING**: All fixes include regression tests
- **DOCUMENTATION**: Bug fixes documented in CHANGELOG.md

### **Root Cause Analysis**
- **SYSTEMATIC**: Follow systematic debugging approach
- **LOGS**: Comprehensive logging for issue diagnosis
- **REPRODUCTION**: Create test cases to reproduce issues

## 📈 **Performance Optimization**

### **Latency Targets**
- **TASK SUBMISSION**: < 100ms end-to-end latency
- **TASK RETRIEVAL**: < 50ms response time
- **STATUS UPDATES**: < 10ms for status changes

### **Scalability Goals**
- **CONCURRENT TASKS**: Support 1000+ concurrent tasks
- **MEMORY EFFICIENCY**: Bounded memory usage with queues
- **STORAGE PERFORMANCE**: Fast task retrieval from persistent storage

## 🎯 **Feature Development Checklist**

### **New Feature Implementation**
- [ ] Check `.tasks` file for existing similar tasks
- [ ] Create technical documentation first
- [ ] Implement core business logic in server.rs
- [ ] Add REST API endpoint with proper serialization
- [ ] Add MCP tool support with input/output schemas
- [ ] Add WebSocket real-time updates if needed
- [ ] Update Vue.js dashboard if UI changes required
- [ ] Write comprehensive tests
- [ ] Update documentation
- [ ] Add metrics and monitoring

### **API Endpoint Addition**
- [ ] Implement business logic in server.rs first
- [ ] Add REST API route with proper serialization
- [ ] Add MCP tool with input/output schemas
- [ ] Add WebSocket events for real-time updates
- [ ] Update dashboard UI if needed
- [ ] Write integration tests for all interfaces
- [ ] Update API documentation

### **Database Schema Changes**
- [ ] Define migration strategy
- [ ] Update data models
- [ ] Implement backward compatibility
- [ ] Add validation and constraints
- [ ] Write migration tests
- [ ] Update schema documentation

## ⚡ **Quick Reference Commands**

### **Development Workflow**
```bash
# Start development server
cargo run

# Alternative with custom config
cargo run -- --host 127.0.0.1 --port 16080

# Run tests
cargo test

# Check code quality
cargo clippy
cargo fmt --check

# Build for production
cargo build --release
```

### **Project Management**
```bash
# Check existing tasks before creating new ones
cat .tasks

# View project status (health check)
curl http://localhost:16080/health

# Access web dashboard
curl http://localhost:16080/

# MCP SSE endpoint (for MCP clients)
curl http://localhost:16080/mcp/sse

# CLI commands (from cli/ directory)
cd cli && cargo run -- tasks list
```

### **Deployment**
```bash
# Build Docker image
docker build -t task-queue .

# Run with Docker Compose
docker-compose up

# Deploy to Kubernetes
kubectl apply -f devops/k8s/
```

## 🚨 **Critical Rules - NEVER Violate**

1. **ALWAYS check `.tasks` file before creating tasks**
2. **NEVER skip workflow phases**
3. **ALWAYS implement features across all interfaces (Dashboard, REST, MCP, WebSocket)**
4. **NEVER create duplicate projects/tasks**
5. **ALWAYS run tests and verify results**
6. **NEVER commit without code review**
7. **ALWAYS update documentation**
8. **NEVER break backward compatibility without migration**
9. **ALWAYS use structured logging**
10. **NEVER expose sensitive data in logs**

## 🎯 **Success Metrics**

- **Test Coverage**: > 85% code coverage
- **Performance**: < 100ms task submission latency
- **Reliability**: 99.9% uptime in production
- **Security**: Zero security vulnerabilities
- **Maintainability**: < 30 min average bug fix time
- **Scalability**: Support 10,000+ concurrent tasks

## 📞 **Getting Help**

When in doubt:
1. Check existing `.tasks` file for similar work
2. Review project documentation in `docs/`
3. Consult team workflow guidelines
4. Ask for clarification on unclear requirements

Remember: **Quality over speed, correctness over convenience, documentation over implementation.**

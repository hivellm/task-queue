# 🚨 CRITICAL TESTING REQUIREMENTS - TASK QUEUE

## **MANDATORY TEST EXECUTION PROTOCOL**

### **⚠️ BEFORE ANY CODE CHANGES:**

1. **Read the testing requirements in each module**
2. **Execute tests using the provided commands**
3. **Verify ALL tests pass before proceeding**
4. **Fix any failing tests immediately**

---

## **MODULE-SPECIFIC TEST COMMANDS**

### **1. Structured Logging (`src/logging.rs`)**
```bash
# Run all logging tests
cargo test --lib logging

# Run with verbose output
cargo test --lib logging -- --nocapture

# Run specific test
cargo test --lib logging test_log_level_conversion

# Run with coverage
cargo test --lib logging -- --nocapture --test-threads=1
```

### **2. Cache System (`src/cache.rs`)**
```bash
# Run all cache tests
cargo test --lib cache

# Run with verbose output
cargo test --lib cache -- --nocapture

# Run specific test
cargo test --lib cache test_in_memory_cache_put_get

# Run with coverage
cargo test --lib cache -- --nocapture --test-threads=1
```

### **3. Rate Limiting (`src/rate_limiting.rs`)**
```bash
# Run all rate limiting tests
cargo test --lib rate_limiting

# Run with verbose output
cargo test --lib rate_limiting -- --nocapture

# Run specific test
cargo test --lib rate_limiting test_token_bucket_rate_limiting

# Run with coverage
cargo test --lib rate_limiting -- --nocapture --test-threads=1
```

### **4. WebSocket Support (`src/websocket.rs`)**
```bash
# Run all WebSocket tests
cargo test --lib websocket

# Run with verbose output
cargo test --lib websocket -- --nocapture

# Run specific test
cargo test --lib websocket test_websocket_manager_creation

# Run with coverage
cargo test --lib websocket -- --nocapture --test-threads=1
```

---

## **AUTOMATED TEST SCRIPTS**

### **Linux/macOS:**
```bash
# Run all tests
./scripts/run-tests.sh

# Run specific module
./scripts/run-tests.sh logging

# Run specific test
./scripts/run-tests.sh cache test_in_memory_cache_put_get
```

### **Windows PowerShell:**
```powershell
# Run all tests
.\scripts\run-tests.ps1

# Run specific module
.\scripts\run-tests.ps1 -Module logging

# Run specific test
.\scripts\run-tests.ps1 -Module cache -Test test_in_memory_cache_put_get
```

---

## **TEST EXECUTION WORKFLOW**

### **1. Before Making Changes:**
```bash
# Execute tests for the module you're about to modify
cargo test --lib <module_name> -- --nocapture
```

### **2. After Making Changes:**
```bash
# Execute tests again to verify nothing broke
cargo test --lib <module_name> -- --nocapture
```

### **3. Before Committing:**
```bash
# Run all tests to ensure everything works
cargo test -- --nocapture
```

---

## **TEST VALIDATION CHECKLIST**

- [ ] **Tests are executed** using `cargo test --lib <module>`
- [ ] **All tests pass** without errors
- [ ] **No warnings** in test output
- [ ] **Coverage is adequate** (minimum 85%)
- [ ] **Test results are documented**
- [ ] **Failing tests are fixed** before proceeding

---

## **COMMON TEST FAILURES AND FIXES**

### **Compilation Errors:**
```bash
# Check for syntax errors
cargo check --lib <module>

# Fix compilation issues first
cargo build --lib <module>
```

### **Test Failures:**
```bash
# Run specific failing test with verbose output
cargo test --lib <module> <test_name> -- --nocapture

# Check test logic and fix implementation
```

### **Coverage Issues:**
```bash
# Run tests with coverage
cargo test --lib <module> -- --nocapture --test-threads=1

# Add more test cases if coverage is low
```

---

## **⚠️ CRITICAL RULES**

1. **NO COMMITS WITHOUT PASSING TESTS**
2. **TESTS MUST BE EXECUTED, NOT JUST CREATED**
3. **USE `run_terminal_cmd` TO VERIFY TEST EXECUTION**
4. **FIX ALL FAILING TESTS BEFORE PROCEEDING**
5. **DOCUMENT TEST RESULTS AND COVERAGE**

---

## **EMERGENCY TEST COMMANDS**

### **Quick Test All Modules:**
```bash
cargo test --lib logging && cargo test --lib cache && cargo test --lib rate_limiting && cargo test --lib websocket
```

### **Test with Full Output:**
```bash
cargo test -- --nocapture --test-threads=1
```

### **Test Specific Module with Debug:**
```bash
RUST_LOG=debug cargo test --lib <module> -- --nocapture
```

---

**🚨 REMEMBER: Testing is not optional - it's mandatory and must be executed!**


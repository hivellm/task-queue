# CRITICAL TESTING REQUIREMENTS - TASK QUEUE DEVELOPMENT

## 🚨 MANDATORY TESTING PROTOCOL

### **BEFORE IMPLEMENTING ANY TASK:**

1. **Read and understand these testing requirements**
2. **Update task description with testing requirements**
3. **Follow the testing protocol throughout development**

### **TESTING PHASE REQUIREMENTS:**

#### **TestCreation Phase:**
- Create comprehensive test suite (unit, integration, performance)
- Include edge cases and error scenarios
- Document test coverage goals (minimum 85%)

#### **Testing Phase - CRITICAL:**
- **MUST ACTUALLY EXECUTE ALL TESTS** using appropriate commands:
  - Rust: `cargo test --lib <module_name>`
  - Node.js: `npm test` or `yarn test`
  - E2E: `npm run test:e2e` or `npx playwright test`
- **ALL TESTS MUST PASS** before advancing to AIReview
- **Use `run_terminal_cmd` tool** to execute tests and verify results
- **Fix any failing tests** before proceeding
- **Document test results** and coverage metrics

#### **Test Validation:**
- Verify all tests pass with actual execution
- Confirm test coverage meets requirements
- Document any test failures and fixes
- Provide test execution logs

### **COMMANDS TO EXECUTE TESTS:**

#### **Rust Modules:**
```bash
# Test specific module
cargo test --lib <module_name>

# Test with coverage
cargo test --lib <module_name> -- --nocapture

# Test integration tests
cargo test --test integration
```

#### **Node.js/TypeScript:**
```bash
# Run all tests
npm test

# Run with coverage
npm run test:coverage

# Run E2E tests
npm run test:e2e
```

#### **E2E Tests:**
```bash
# Playwright tests
npx playwright test --headed

# Cypress tests
npx cypress run
```

### **TESTING CHECKLIST:**

- [ ] Tests are created in TestCreation phase
- [ ] Tests are executed using `run_terminal_cmd` in Testing phase
- [ ] All tests pass successfully
- [ ] Test coverage meets minimum requirements
- [ ] Test failures are fixed before proceeding
- [ ] Test results are documented
- [ ] Only advance to AIReview after tests pass

### **⚠️ CRITICAL RULES:**

1. **NO ADVANCEMENT WITHOUT PASSING TESTS**
2. **TESTS MUST BE EXECUTED, NOT JUST CREATED**
3. **USE `run_terminal_cmd` TO VERIFY TEST EXECUTION**
4. **FIX ALL FAILING TESTS BEFORE PROCEEDING**
5. **DOCUMENT TEST RESULTS AND COVERAGE**

### **FAILURE TO FOLLOW TESTING PROTOCOL WILL RESULT IN:**
- Task rejection
- Workflow restart
- Implementation review failure

## **IMPLEMENTATION WORKFLOW:**

1. **Planning Phase**: Create technical documentation
2. **Implementation Phase**: Implement code according to documentation
3. **TestCreation Phase**: Create comprehensive test suite
4. **Testing Phase**: **EXECUTE TESTS AND ENSURE THEY PASS**
5. **AIReview Phase**: Get AI model reviews
6. **Completed**: Task successfully completed

---

**REMEMBER: Testing is not optional - it's mandatory and must be executed!**


#!/bin/bash

# 🚨 CRITICAL TESTING SCRIPT - TASK QUEUE DEVELOPMENT
# This script ensures all tests are executed and validated

set -e  # Exit on any error

echo "🚀 Starting Task Queue Test Suite Execution..."
echo "=============================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to run tests for a module
run_module_tests() {
    local module_name=$1
    local test_name=$2
    
    echo -e "\n${YELLOW}Testing module: $module_name${NC}"
    echo "----------------------------------------"
    
    if cargo test --lib "$module_name" -- --nocapture; then
        echo -e "${GREEN}✅ $module_name tests PASSED${NC}"
        return 0
    else
        echo -e "${RED}❌ $module_name tests FAILED${NC}"
        return 1
    fi
}

# Function to run specific test
run_specific_test() {
    local module_name=$1
    local test_name=$2
    
    echo -e "\n${YELLOW}Running specific test: $test_name in $module_name${NC}"
    echo "----------------------------------------"
    
    if cargo test --lib "$module_name" "$test_name" -- --nocapture; then
        echo -e "${GREEN}✅ $test_name PASSED${NC}"
        return 0
    else
        echo -e "${RED}❌ $test_name FAILED${NC}"
        return 1
    fi
}

# Main test execution
main() {
    local failed_modules=()
    
    echo "🔍 Running all module tests..."
    
    # Test all implemented modules
    modules=("logging" "cache" "rate_limiting" "websocket")
    
    for module in "${modules[@]}"; do
        if ! run_module_tests "$module"; then
            failed_modules+=("$module")
        fi
    done
    
    # Summary
    echo -e "\n=============================================="
    echo -e "📊 TEST EXECUTION SUMMARY"
    echo -e "=============================================="
    
    if [ ${#failed_modules[@]} -eq 0 ]; then
        echo -e "${GREEN}🎉 ALL TESTS PASSED!${NC}"
        echo -e "${GREEN}✅ All modules are working correctly${NC}"
        exit 0
    else
        echo -e "${RED}❌ FAILED MODULES:${NC}"
        for module in "${failed_modules[@]}"; do
            echo -e "${RED}  - $module${NC}"
        done
        echo -e "\n${RED}🚨 SOME TESTS FAILED - FIX BEFORE PROCEEDING${NC}"
        exit 1
    fi
}

# Help function
show_help() {
    echo "Task Queue Test Runner"
    echo ""
    echo "Usage:"
    echo "  $0                    # Run all tests"
    echo "  $0 <module>          # Run tests for specific module"
    echo "  $0 <module> <test>   # Run specific test in module"
    echo "  $0 --help            # Show this help"
    echo ""
    echo "Available modules:"
    echo "  logging       - Structured logging tests"
    echo "  cache         - Cache system tests"
    echo "  rate_limiting - Rate limiting tests"
    echo "  websocket     - WebSocket support tests"
    echo ""
    echo "Examples:"
    echo "  $0 logging"
    echo "  $0 cache test_in_memory_cache_put_get"
    echo "  $0 websocket test_websocket_manager_creation"
}

# Parse arguments
case "$1" in
    --help|-h)
        show_help
        exit 0
        ;;
    "")
        main
        ;;
    *)
        if [ -n "$2" ]; then
            # Run specific test
            run_specific_test "$1" "$2"
        else
            # Run module tests
            run_module_tests "$1"
        fi
        ;;
esac


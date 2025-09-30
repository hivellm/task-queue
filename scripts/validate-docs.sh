#!/bin/bash

# Documentation Validation Script
# This script validates the completeness and accuracy of the documentation

echo "🔍 Validating Task Queue Documentation..."

# Check if documentation file exists
if [ ! -f "docs/COMPLETE_DOCUMENTATION.md" ]; then
    echo "❌ Documentation file not found!"
    exit 1
fi

echo "✅ Documentation file exists"

# Check documentation sections
sections=(
    "Overview"
    "Architecture" 
    "API Reference"
    "Configuration"
    "Deployment"
    "Development Guide"
    "User Guide"
    "Troubleshooting"
    "Contributing"
)

echo "📋 Checking required sections..."

for section in "${sections[@]}"; do
    if grep -q "## $section" docs/COMPLETE_DOCUMENTATION.md; then
        echo "✅ Section '$section' found"
    else
        echo "❌ Section '$section' missing!"
        exit 1
    fi
done

# Check for code examples
echo "💻 Checking for code examples..."

if grep -q "```rust" docs/COMPLETE_DOCUMENTATION.md; then
    echo "✅ Rust code examples found"
else
    echo "❌ No Rust code examples found!"
    exit 1
fi

if grep -q "```bash" docs/COMPLETE_DOCUMENTATION.md; then
    echo "✅ Bash code examples found"
else
    echo "❌ No Bash code examples found!"
    exit 1
fi

if grep -q "```python" docs/COMPLETE_DOCUMENTATION.md; then
    echo "✅ Python code examples found"
else
    echo "❌ No Python code examples found!"
    exit 1
fi

# Check for API endpoints documentation
echo "🌐 Checking API documentation..."

api_endpoints=(
    "GET /tasks"
    "POST /tasks"
    "GET /projects"
    "POST /projects"
    "GET /workflows"
    "GET /stats"
    "GET /health"
)

for endpoint in "${api_endpoints[@]}"; do
    if grep -q "$endpoint" docs/COMPLETE_DOCUMENTATION.md; then
        echo "✅ API endpoint '$endpoint' documented"
    else
        echo "❌ API endpoint '$endpoint' not documented!"
        exit 1
    fi
done

# Check for MCP tools documentation
echo "🤖 Checking MCP tools documentation..."

mcp_tools=(
    "submit_task"
    "get_task"
    "list_tasks"
    "create_project"
    "advance_workflow_phase"
)

for tool in "${mcp_tools[@]}"; do
    if grep -q "$tool" docs/COMPLETE_DOCUMENTATION.md; then
        echo "✅ MCP tool '$tool' documented"
    else
        echo "❌ MCP tool '$tool' not documented!"
        exit 1
    fi
done

# Check documentation length (should be substantial)
word_count=$(wc -w < docs/COMPLETE_DOCUMENTATION.md)
if [ "$word_count" -gt 5000 ]; then
    echo "✅ Documentation is comprehensive ($word_count words)"
else
    echo "❌ Documentation seems too short ($word_count words)"
    exit 1
fi

# Check for links and references
echo "🔗 Checking for internal links..."

if grep -q "\[.*\](#.*)" docs/COMPLETE_DOCUMENTATION.md; then
    echo "✅ Internal links found"
else
    echo "❌ No internal links found!"
    exit 1
fi

# Check for table of contents
echo "📑 Checking table of contents..."

if grep -q "Table of Contents" docs/COMPLETE_DOCUMENTATION.md; then
    echo "✅ Table of contents found"
else
    echo "❌ Table of contents missing!"
    exit 1
fi

echo ""
echo "🎉 Documentation validation completed successfully!"
echo "📊 Coverage: 100% of required sections documented"
echo "📝 Word count: $word_count words"
echo "✅ All validation checks passed!"

exit 0

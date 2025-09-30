# 🚨 CRITICAL TESTING SCRIPT - TASK QUEUE DEVELOPMENT (PowerShell)
# This script ensures all tests are executed and validated

param(
    [string]$Module = "",
    [string]$Test = "",
    [switch]$Help
)

# Colors for output
$Red = "Red"
$Green = "Green"
$Yellow = "Yellow"

# Function to run tests for a module
function Run-ModuleTests {
    param([string]$ModuleName)
    
    Write-Host "`nTesting module: $ModuleName" -ForegroundColor $Yellow
    Write-Host "----------------------------------------"
    
    $result = cargo test --lib $ModuleName -- --nocapture
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ $ModuleName tests PASSED" -ForegroundColor $Green
        return $true
    } else {
        Write-Host "❌ $ModuleName tests FAILED" -ForegroundColor $Red
        return $false
    }
}

# Function to run specific test
function Run-SpecificTest {
    param([string]$ModuleName, [string]$TestName)
    
    Write-Host "`nRunning specific test: $TestName in $ModuleName" -ForegroundColor $Yellow
    Write-Host "----------------------------------------"
    
    $result = cargo test --lib $ModuleName $TestName -- --nocapture
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ $TestName PASSED" -ForegroundColor $Green
        return $true
    } else {
        Write-Host "❌ $TestName FAILED" -ForegroundColor $Red
        return $false
    }
}

# Help function
function Show-Help {
    Write-Host "Task Queue Test Runner (PowerShell)"
    Write-Host ""
    Write-Host "Usage:"
    Write-Host "  .\run-tests.ps1                    # Run all tests"
    Write-Host "  .\run-tests.ps1 -Module <module>  # Run tests for specific module"
    Write-Host "  .\run-tests.ps1 -Module <module> -Test <test>  # Run specific test"
    Write-Host "  .\run-tests.ps1 -Help             # Show this help"
    Write-Host ""
    Write-Host "Available modules:"
    Write-Host "  logging       - Structured logging tests"
    Write-Host "  cache         - Cache system tests"
    Write-Host "  rate_limiting - Rate limiting tests"
    Write-Host "  websocket     - WebSocket support tests"
    Write-Host ""
    Write-Host "Examples:"
    Write-Host "  .\run-tests.ps1 -Module logging"
    Write-Host "  .\run-tests.ps1 -Module cache -Test test_in_memory_cache_put_get"
    Write-Host "  .\run-tests.ps1 -Module websocket -Test test_websocket_manager_creation"
}

# Main test execution
function Main {
    Write-Host "🚀 Starting Task Queue Test Suite Execution..." -ForegroundColor $Green
    Write-Host "=============================================="
    
    $failedModules = @()
    $modules = @("logging", "cache", "rate_limiting", "websocket")
    
    Write-Host "🔍 Running all module tests..."
    
    foreach ($module in $modules) {
        if (-not (Run-ModuleTests $module)) {
            $failedModules += $module
        }
    }
    
    # Summary
    Write-Host "`n=============================================="
    Write-Host "📊 TEST EXECUTION SUMMARY"
    Write-Host "=============================================="
    
    if ($failedModules.Count -eq 0) {
        Write-Host "🎉 ALL TESTS PASSED!" -ForegroundColor $Green
        Write-Host "✅ All modules are working correctly" -ForegroundColor $Green
        exit 0
    } else {
        Write-Host "❌ FAILED MODULES:" -ForegroundColor $Red
        foreach ($module in $failedModules) {
            Write-Host "  - $module" -ForegroundColor $Red
        }
        Write-Host "`n🚨 SOME TESTS FAILED - FIX BEFORE PROCEEDING" -ForegroundColor $Red
        exit 1
    }
}

# Parse arguments and execute
if ($Help) {
    Show-Help
    exit 0
} elseif ($Module -eq "") {
    Main
} elseif ($Test -ne "") {
    Run-SpecificTest $Module $Test
} else {
    Run-ModuleTests $Module
}


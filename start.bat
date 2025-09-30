@echo off
REM Task Queue Start Script for Windows
REM This script starts both HTTP and MCP servers

echo 🚀 Starting Task Queue with MCP...

REM Colors for output (using Windows color codes)
REM We can't use ANSI colors in batch, but we can use color command

REM Function to check if port is in use
:check_port
setlocal enabledelayedexpansion
set PORT=%1
netstat -ano | findstr :!PORT! | findstr LISTENING >nul
if %errorlevel% equ 0 (
    echo ❌ Port !PORT! is already in use
    endlocal
    exit /b 0
) else (
    echo ✅ Port !PORT! is free
    endlocal
    exit /b 1
)

REM Function to kill process using port
:kill_process_on_port
set PORT=%1
for /f "tokens=5" %%a in ('netstat -ano ^| findstr :%PORT% ^| findstr LISTENING') do (
    echo ⚠️  Killing process %%a using port %PORT%
    taskkill /f /pid %%a >nul 2>&1
    timeout /t 2 >nul
)
goto :eof

REM Function to wait for port to be free
:wait_for_port
set PORT=%1
set MAX_ATTEMPTS=10
set ATTEMPT=1

:wait_loop
call :check_port %PORT%
if %errorlevel% equ 1 (
    echo ✅ Port %PORT% is now free
    goto :eof
)

echo ⏳ Waiting for port %PORT% to be free (attempt %ATTEMPT%/%MAX_ATTEMPTS%)...
call :kill_process_on_port %PORT%
timeout /t 1 >nul
set /a ATTEMPT+=1
if %ATTEMPT% leq %MAX_ATTEMPTS% goto wait_loop

echo ❌ Failed to free port %PORT% after %MAX_ATTEMPTS% attempts
exit /b 1

REM Kill any existing task-queue processes
echo 🔍 Checking for existing task-queue processes...
tasklist /fi "imagename eq task-queue.exe" 2>nul | findstr task-queue.exe >nul
if %errorlevel% equ 0 (
    echo ⚠️  Killing existing task-queue processes...
    taskkill /f /im task-queue.exe >nul 2>&1
    timeout /t 2 >nul
)

REM Check and free ports
echo 🔍 Checking ports 16080 and 16081...
call :wait_for_port 16080
if %errorlevel% neq 0 exit /b 1

call :wait_for_port 16081
if %errorlevel% neq 0 exit /b 1

REM Build the project
echo 🔨 Building project...
cargo build --release
if %errorlevel% neq 0 (
    echo ❌ Build failed
    exit /b 1
)

REM Start the server
echo 🚀 Starting Task Queue server...
echo 📋 HTTP API will be available at: http://localhost:16080
echo 📋 MCP WebSocket will be available at: ws://localhost:16081/mcp
echo.

target\release\task-queue.exe

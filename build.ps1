Write-Host "🚀 Building Task Queue Dashboard and Server..." -ForegroundColor Green

# Build the dashboard with Vite
Write-Host "📦 Building dashboard with Vite..." -ForegroundColor Yellow
Set-Location dashboard
npm install
npm run build
Set-Location ..

# Build the Rust server
Write-Host "🔨 Building Rust server..." -ForegroundColor Yellow
cargo build --release

Write-Host "✅ Build complete!" -ForegroundColor Green
Write-Host "📁 Dashboard build: dashboard/dist/" -ForegroundColor Cyan
Write-Host "📁 Server binary: target/release/task-queue.exe" -ForegroundColor Cyan

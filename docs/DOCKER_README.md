# Task Queue - Docker & Containerization

This directory contains all necessary files to containerize and deploy the Task Queue using Docker, Docker Compose, and Kubernetes.

## 📁 File Structure

```
├── devops/
│   ├── Dockerfile              # Optimized production image
│   ├── Dockerfile.dev          # Development image
│   ├── docker-compose.yml      # Development compose
│   ├── docker-compose.prod.yml # Production compose
│   ├── .dockerignore           # Files ignored in build
│   ├── nginx.conf              # Nginx configuration
│   ├── prometheus.yml          # Prometheus configuration
│   ├── k8s/                    # Kubernetes manifests
│   │   ├── deployment.yaml     # Main deployment
│   │   └── monitoring.yaml     # Monitoring and backup
│   └── scripts/
│       └── docker-build.sh     # Build and deploy script
```

## 🚀 Quick Start

### Local Development

```bash
# Navigate to devops directory
cd devops/

# Build and run with Docker Compose
docker-compose up --build

# Development only with hot reload
docker-compose --profile dev up --build
```

### Production

```bash
# Navigate to devops directory
cd devops/

# Build image
docker build -t task-queue:latest .

# Run with Docker Compose
docker-compose -f docker-compose.prod.yml up -d

# With monitoring
docker-compose -f docker-compose.prod.yml --profile monitoring up -d
```

### Kubernetes

```bash
# Navigate to devops directory
cd devops/

# Full deployment
kubectl apply -f k8s/

# Check status
kubectl get pods -n task-queue

# Logs
kubectl logs -f deployment/task-queue -n task-queue
```

## 🐳 Docker Images

### Production (`Dockerfile`)
- **Base**: `rust:1.75-slim`
- **Runtime**: `debian:bookworm-slim`
- **Size**: ~50MB (optimized)
- **User**: Non-root (`taskqueue`)
- **Ports**: 16080 (HTTP API + MCP Server)

### Development (`Dockerfile.dev`)
- **Base**: `rust:1.75-slim`
- **Tools**: `cargo-watch` for hot reload
- **Volumes**: Source code mounted
- **Command**: `cargo watch -x run`

## 🔧 Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_LOG` | `info` | Log level |
| `TASK_QUEUE_DB_PATH` | `/app/data/task-queue.db` | Database path |
| `TASK_QUEUE_PORT` | `16080` | HTTP port (includes MCP) |

### Volumes

- `/app/data` - Persistent data
- `/app/logs` - Application logs
- `/app/config.yml` - Configuration

## 📊 Monitoring

### Prometheus Metrics
- **Endpoint**: `/metrics`
- **Port**: 9090
- **Interval**: 30s

### Health Checks
- **Endpoint**: `/health`
- **Interval**: 30s
- **Timeout**: 10s

### Grafana Dashboards
- **URL**: http://localhost:3001
- **User**: `admin`
- **Password**: `admin`

## 🔒 Security

### Security Headers
- `X-Frame-Options: DENY`
- `X-Content-Type-Options: nosniff`
- `X-XSS-Protection: 1; mode=block`
- `Strict-Transport-Security`

### Rate Limiting
- **API**: 10 req/s per IP
- **Login**: 1 req/s per IP
- **Burst**: 20 requests

### Restricted Access
- **Metrics**: Private networks only
- **Admin**: Authentication required

## 🚀 Automated Deployment

### Build Script

```bash
# Navigate to devops directory
cd devops/

# Simple build
./scripts/docker-build.sh

# Build with deploy
./scripts/docker-build.sh latest localhost:5000 deploy

# Deploy to Kubernetes
./scripts/docker-build.sh latest your-registry.com k8s
```

### CI/CD Pipeline

```yaml
# GitHub Actions example
- name: Build and Push
  run: |
    cd devops/
    docker build -t ${{ secrets.REGISTRY }}/task-queue:${{ github.sha }} .
    docker push ${{ secrets.REGISTRY }}/task-queue:${{ github.sha }}

- name: Deploy to K8s
  run: |
    cd devops/
    kubectl apply -f k8s/
    kubectl set image deployment/task-queue task-queue=${{ secrets.REGISTRY }}/task-queue:${{ github.sha }} -n task-queue
```

## 🔧 Troubleshooting

### Common Issues

1. **Container won't start**
   ```bash
   docker logs task-queue-server
   ```

2. **Health check fails**
   ```bash
   curl http://localhost:16080/health
   ```

3. **Permission issues**
   ```bash
   docker exec -it task-queue-server ls -la /app/data
   ```

### Logs

```bash
# Navigate to devops directory
cd devops/

# Docker Compose
docker-compose logs -f task-queue

# Kubernetes
kubectl logs -f deployment/task-queue -n task-queue

# Nginx
docker-compose logs -f nginx
```

## 📈 Performance

### Recommended Resources

| Environment | CPU | Memory | Storage |
|-------------|-----|--------|---------|
| Development | 0.5 cores | 512MB | 1GB |
| Production | 1 core | 1GB | 10GB |
| Kubernetes | 0.5 cores | 512MB | 10GB |

### Optimizations

- **Multi-stage build** to reduce size
- **Layer caching** for faster builds
- **Health checks** for automatic restart
- **Resource limits** for stability

## 🔄 Backup and Restore

### Automatic Backup (Kubernetes)
- **CronJob**: Daily at 2 AM
- **Retention**: 7 days
- **Location**: `/app/backups/`

### Manual Backup
```bash
# Docker
docker exec task-queue-server cp /app/data/task-queue.db /app/backups/backup-$(date +%Y%m%d).db

# Kubernetes
kubectl exec deployment/task-queue -n task-queue -- cp /app/data/task-queue.db /app/backups/backup-$(date +%Y%m%d).db
```

### Restore
```bash
# Navigate to devops directory
cd devops/

# Stop service
docker-compose down

# Restore backup
docker run --rm -v task-queue-data:/data -v $(pwd)/backups:/backups alpine cp /backups/backup-20240101.db /data/task-queue.db

# Restart service
docker-compose up -d
```
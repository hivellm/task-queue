#!/bin/bash

# Task Queue Docker Build and Deployment Script
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
IMAGE_NAME="task-queue"
TAG="${1:-latest}"
REGISTRY="${2:-localhost:5000}"

echo -e "${GREEN}🚀 Task Queue Docker Build and Deployment${NC}"
echo "================================================"

# Function to print colored output
print_status() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    print_error "Docker is not running. Please start Docker and try again."
    exit 1
fi

print_status "Docker is running"

# Build the Docker image
echo -e "\n${YELLOW}🔨 Building Docker image...${NC}"
docker build -t "${IMAGE_NAME}:${TAG}" .

if [ $? -eq 0 ]; then
    print_status "Docker image built successfully"
else
    print_error "Failed to build Docker image"
    exit 1
fi

# Tag for registry
if [ "$REGISTRY" != "localhost:5000" ]; then
    echo -e "\n${YELLOW}🏷️  Tagging image for registry...${NC}"
    docker tag "${IMAGE_NAME}:${TAG}" "${REGISTRY}/${IMAGE_NAME}:${TAG}"
    print_status "Image tagged for registry: ${REGISTRY}/${IMAGE_NAME}:${TAG}"
fi

# Test the image
echo -e "\n${YELLOW}🧪 Testing Docker image...${NC}"
docker run --rm -d --name test-container -p 16080:16080 "${IMAGE_NAME}:${TAG}"

# Wait for container to start
sleep 5

# Check if container is running
if docker ps | grep -q test-container; then
    print_status "Container started successfully"
    
    # Test health endpoint
    if curl -f http://localhost:16080/health > /dev/null 2>&1; then
        print_status "Health check passed"
    else
        print_warning "Health check failed - service may still be starting"
    fi
    
    # Stop test container
    docker stop test-container
    print_status "Test container stopped"
else
    print_error "Failed to start test container"
    exit 1
fi

# Deploy with Docker Compose (if requested)
if [ "$3" = "deploy" ]; then
    echo -e "\n${YELLOW}🚀 Deploying with Docker Compose...${NC}"
    
    # Stop existing containers
    docker-compose down 2>/dev/null || true
    
    # Start new containers
    docker-compose up -d
    
    if [ $? -eq 0 ]; then
        print_status "Deployed successfully with Docker Compose"
        echo -e "\n${GREEN}📊 Service Status:${NC}"
        docker-compose ps
    else
        print_error "Failed to deploy with Docker Compose"
        exit 1
    fi
fi

# Kubernetes deployment (if requested)
if [ "$3" = "k8s" ]; then
    echo -e "\n${YELLOW}☸️  Deploying to Kubernetes...${NC}"
    
    # Apply Kubernetes manifests
    kubectl apply -f k8s/
    
    if [ $? -eq 0 ]; then
        print_status "Kubernetes deployment applied successfully"
        echo -e "\n${GREEN}📊 Pod Status:${NC}"
        kubectl get pods -n task-queue
    else
        print_error "Failed to deploy to Kubernetes"
        exit 1
    fi
fi

echo -e "\n${GREEN}🎉 Build and deployment completed successfully!${NC}"
echo -e "\n${YELLOW}📋 Next Steps:${NC}"
echo "1. Access the service at: http://localhost:16080"
echo "2. Check logs: docker-compose logs -f task-queue"
echo "3. Monitor health: curl http://localhost:16080/health"
echo "4. View metrics: curl http://localhost:16080/metrics"

echo -e "\n${YELLOW}🔧 Useful Commands:${NC}"
echo "- Build only: ./scripts/docker-build.sh"
echo "- Build and deploy: ./scripts/docker-build.sh latest localhost:5000 deploy"
echo "- Deploy to K8s: ./scripts/docker-build.sh latest your-registry.com k8s"

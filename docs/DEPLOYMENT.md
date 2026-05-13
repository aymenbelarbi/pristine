# Pristine Deployment Guide

## Deployment Options

Pristine can be deployed in several ways depending on your needs:

| Method | Use Case | Complexity |
|--------|----------|------------|
| Binary | Single server, simple deployment | Low |
| Docker | Containerized environments | Medium |
| Docker Compose | Multi-container setups | Medium |
| Kubernetes | Production, high availability | High |

## Binary Deployment

### Download Pre-built Binary

```bash
# Download latest release
curl -sSf https://pristine.dev/install.sh | sh

# Or download directly
curl -LO https://github.com/aymenbelarbi/pristine/releases/latest/download/pristine-linux-x64
chmod +x pristine-linux-x64
sudo mv pristine-linux-x64 /usr/local/bin/pristine
```

### Build from Source

```bash
# Clone repository
git clone https://github.com/aymenbelarbi/pristine.git
cd pristine

# Build release binary
cargo build --release --bin pristine-api

# Binary location
./target/release/pristine-api
```

### Run as Systemd Service

Create `/etc/systemd/system/pristine.service`:

```ini
[Unit]
Description=Pristine API Server
After=network.target

[Service]
Type=simple
User=pristine
ExecStart=/usr/local/bin/pristine-api --config /etc/pristine/config.yaml
Restart=always
RestartSec=5
Environment=PRISTINE_LOG_LEVEL=info

[Install]
WantedBy=multi-user.target
```

Enable and start the service:

```bash
sudo systemctl daemon-reload
sudo systemctl enable pristine
sudo systemctl start pristine
sudo systemctl status pristine
```

## Docker Deployment

### Pull Pre-built Image

```bash
docker pull ghcr.io/aymenbelarbi/pristine:latest
```

### Run Container

```bash
docker run -d \
  --name pristine \
  -p 8080:8080 \
  -e PRISTINE_GITHUB_TOKEN=your_token \
  -e PRISTINE_LOG_LEVEL=info \
  -v pristine-cache:/app/.cache \
  ghcr.io/aymenbelarbi/pristine:latest
```

### Build Custom Image

```bash
# Build from Dockerfile
docker build -t pristine:latest .

# Run custom image
docker run -d \
  --name pristine \
  -p 8080:8080 \
  pristine:latest
```

### Dockerfile Reference

```dockerfile
# Build stage
FROM rust:1.75-slim-bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

RUN cargo build --release --bin pristine-api

# Runtime stage
FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/pristine-api /usr/local/bin/

RUN useradd -m -u 1000 pristine
USER pristine

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

ENTRYPOINT ["pristine-api"]
```

## Docker Compose Deployment

### Basic Setup

Create `docker-compose.yml`:

```yaml
version: '3.8'

services:
  pristine:
    image: ghcr.io/aymenbelarbi/pristine:latest
    ports:
      - "8080:8080"
    environment:
      PRISTINE_GITHUB_TOKEN: ${GITHUB_TOKEN}
      PRISTINE_LOG_LEVEL: info
    volumes:
      - pristine-cache:/app/.cache
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3

volumes:
  pristine-cache:
```

### With Frontend

```yaml
version: '3.8'

services:
  pristine-api:
    image: ghcr.io/aymenbelarbi/pristine:latest
    ports:
      - "8080:8080"
    environment:
      PRISTINE_GITHUB_TOKEN: ${GITHUB_TOKEN}
      PRISTINE_LOG_LEVEL: info
    volumes:
      - pristine-cache:/app/.cache
    restart: unless-stopped

  pristine-web:
    build:
      context: ./web
      dockerfile: Dockerfile
    ports:
      - "3000:80"
    environment:
      VITE_API_URL: http://localhost:8080
    depends_on:
      - pristine-api
    restart: unless-stopped

volumes:
  pristine-cache:
```

### With Reverse Proxy (nginx)

```yaml
version: '3.8'

services:
  pristine-api:
    image: ghcr.io/aymenbelarbi/pristine:latest
    expose:
      - "8080"
    environment:
      PRISTINE_GITHUB_TOKEN: ${GITHUB_TOKEN}
    volumes:
      - pristine-cache:/app/.cache
    restart: unless-stopped

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
      - ./ssl:/etc/nginx/ssl:ro
    depends_on:
      - pristine-api
    restart: unless-stopped

volumes:
  pristine-cache:
```

### Deploy with Docker Compose

```bash
# Start services
docker compose up -d

# View logs
docker compose logs -f

# Scale API instances
docker compose up -d --scale pristine-api=3

# Stop services
docker compose down

# Stop and remove volumes
docker compose down -v
```

## Kubernetes Deployment

### Prerequisites

- Kubernetes cluster (1.19+)
- kubectl configured
- Helm 3 (optional)

### Basic Deployment

Create `k8s/deployment.yaml`:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: pristine
  labels:
    app: pristine
spec:
  replicas: 2
  selector:
    matchLabels:
      app: pristine
  template:
    metadata:
      labels:
        app: pristine
    spec:
      containers:
      - name: pristine
        image: ghcr.io/aymenbelarbi/pristine:latest
        ports:
        - containerPort: 8080
        env:
        - name: PRISTINE_GITHUB_TOKEN
          valueFrom:
            secretKeyRef:
              name: pristine-secrets
              key: github-token
        - name: PRISTINE_LOG_LEVEL
          value: "info"
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 10
          periodSeconds: 30
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 10
        volumeMounts:
        - name: cache
          mountPath: /app/.cache
      volumes:
      - name: cache
        emptyDir: {}
---
apiVersion: v1
kind: Service
metadata:
  name: pristine
spec:
  selector:
    app: pristine
  ports:
  - port: 80
    targetPort: 8080
  type: ClusterIP
---
apiVersion: v1
kind: Secret
metadata:
  name: pristine-secrets
type: Opaque
stringData:
  github-token: "your-github-token"
```

### Ingress Configuration

Create `k8s/ingress.yaml`:

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: pristine
  annotations:
    cert-manager.io/cluster-issuer: letsencrypt-prod
    nginx.ingress.kubernetes.io/rate-limit: "60"
spec:
  tls:
  - hosts:
    - pristine.example.com
    secretName: pristine-tls
  rules:
  - host: pristine.example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: pristine
            port:
              number: 80
```

### Deploy to Kubernetes

```bash
# Apply configurations
kubectl apply -f k8s/

# Check deployment status
kubectl get deployments
kubectl get pods
kubectl get services

# View logs
kubectl logs -l app=pristine -f

# Scale deployment
kubectl scale deployment pristine --replicas=3

# Update image
kubectl set image deployment/pristine pristine=ghcr.io/aymenbelarbi/pristine:v0.2.0
```

### Helm Chart (Optional)

```bash
# Add Helm repository
helm repo add pristine https://charts.pristine.dev
helm repo update

# Install
helm install pristine pristine/pristine \
  --set env.PRISTINE_GITHUB_TOKEN=your_token \
  --set server.port=8080 \
  --set replicaCount=2

# Upgrade
helm upgrade pristine pristine/pristine \
  --set image.tag=v0.2.0

# Uninstall
helm uninstall pristine
```

## Environment Configuration

### Production Environment Variables

```bash
# GitHub authentication
PRISTINE_GITHUB_TOKEN=ghp_xxxxxxxxxxxx

# Logging
PRISTINE_LOG_LEVEL=info

# Server
PRISTINE_SERVER_PORT=8080

# Cache
PRISTINE_CACHE_DIR=/var/cache/pristine

# Policy
PRISTINE_POLICY_MODE=redact
```

### Configuration File for Production

Create `/etc/pristine/config.yaml`:

```yaml
default_profile: overview

output:
  format: json
  include_tree: true
  include_stats: true

budget:
  max_tokens: 100000
  compression_preference: structural

policy:
  mode: redact
  scan_secrets: true
  block_patterns:
    - "vendor/**"
    - "node_modules/**"
    - ".env*"

cache:
  enabled: true
  directory: /var/cache/pristine
  max_size: 2147483648  # 2 GB
  ttl: 172800  # 48 hours

server:
  host: 0.0.0.0
  port: 8080
  cors_origins: ["https://app.example.com"]
  rate_limit:
    requests_per_minute: 120
    burst_size: 20

logging:
  level: info
  format: json
```

## Monitoring and Logging

### Health Checks

```bash
# Check health
curl http://localhost:8080/health

# Expected response
{"status": "healthy", "version": "0.1.0"}
```

### Prometheus Metrics

```bash
# Get metrics
curl http://localhost:8080/metrics

# Configure Prometheus scraping
# prometheus.yml
scrape_configs:
  - job_name: 'pristine'
    static_configs:
      - targets: ['pristine:8080']
    metrics_path: /metrics
```

### Structured Logging

```bash
# JSON logging format
PRISTINE_LOG_FORMAT=json

# Example log output
{
  "timestamp": "2024-01-15T10:30:00Z",
  "level": "INFO",
  "message": "Processing artifact request",
  "source": "https://github.com/user/repo",
  "profile": "overview"
}
```

### Log Aggregation

For production, use a log aggregation service:

```yaml
# Docker Compose with Fluentd
services:
  pristine:
    image: ghcr.io/aymenbelarbi/pristine:latest
    logging:
      driver: fluentd
      options:
        fluentd-address: localhost:24224
        tag: pristine
```

## Security Considerations

### Network Security

```bash
# Use HTTPS
# Configure TLS certificates
# Use reverse proxy (nginx, traefik)

# Firewall rules
# Allow only necessary ports
iptables -A INPUT -p tcp --dport 443 -j ACCEPT
iptables -A INPUT -p tcp --dport 8080 -j DROP  # Block direct access
```

### Secret Management

```bash
# Use Kubernetes secrets
kubectl create secret generic pristine-secrets \
  --from-literal=github-token=your_token

# Use Docker secrets
echo "your_token" | docker secret create github_token -

# Use environment files
# .env (not committed to repository)
PRISTINE_GITHUB_TOKEN=your_token
```

### Rate Limiting

Configure rate limiting to prevent abuse:

```yaml
server:
  rate_limit:
    requests_per_minute: 60
    burst_size: 10
```

## Backup and Recovery

### Cache Backup

```bash
# Backup cache directory
tar -czf pristine-cache-backup.tar.gz /var/cache/pristine

# Restore cache
tar -xzf pristine-cache-backup.tar.gz -C /
```

### Configuration Backup

```bash
# Backup configuration
cp /etc/pristine/config.yaml /backup/pristine-config.yaml

# Restore configuration
cp /backup/pristine-config.yaml /etc/pristine/config.yaml
```

## Troubleshooting

### Common Issues

#### Container Won't Start

```bash
# Check logs
docker logs pristine

# Check resource usage
docker stats pristine

# Verify environment variables
docker exec pristine env
```

#### High Memory Usage

```bash
# Check memory usage
kubectl top pods

# Adjust resource limits
kubectl set resources deployment pristine \
  --limits=memory=1Gi,cpu=1000m
```

#### Connection Refused

```bash
# Check service status
kubectl get services

# Check pod status
kubectl get pods

# Check endpoints
kubectl get endpoints
```

## Scaling

### Horizontal Scaling

```bash
# Docker Compose
docker compose up -d --scale pristine-api=3

# Kubernetes
kubectl scale deployment pristine --replicas=5
```

### Load Balancing

```yaml
# Kubernetes service with load balancing
apiVersion: v1
kind: Service
metadata:
  name: pristine
spec:
  type: LoadBalancer
  selector:
    app: pristine
  ports:
  - port: 80
    targetPort: 8080
```

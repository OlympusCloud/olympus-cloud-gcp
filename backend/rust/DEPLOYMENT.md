# Production Deployment Guide - Olympus Cloud Rust Services

## Overview

This guide covers the complete deployment process for Olympus Cloud Rust services in production environments, including GCP Cloud Run, Kubernetes, and Docker Swarm deployments.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Environment Setup](#environment-setup)
3. [Database Setup](#database-setup)
4. [Docker Deployment](#docker-deployment)
5. [GCP Cloud Run Deployment](#gcp-cloud-run-deployment)
6. [Kubernetes Deployment](#kubernetes-deployment)
7. [Monitoring Setup](#monitoring-setup)
8. [Security Checklist](#security-checklist)
9. [Rollback Procedures](#rollback-procedures)
10. [Troubleshooting](#troubleshooting)

## Prerequisites

### Required Tools
- Docker 24.0+
- Docker Compose 2.20+
- gcloud CLI 450.0+
- kubectl 1.28+
- Terraform 1.6+
- PostgreSQL client 16+
- Redis client 7+

### Access Requirements
- GCP Project with appropriate permissions
- Docker Hub account for image registry
- Domain with DNS management access
- SSL certificates or cert-manager setup

## Environment Setup

### 1. Clone Repository

```bash
git clone https://github.com/OlympusCloud/olympus-cloud-gcp.git
cd olympus-cloud-gcp/backend/rust
```

### 2. Create Environment Files

```bash
# Production environment
cp .env.example .env.production

# Edit with production values
vim .env.production
```

Required environment variables:

```env
# Database
DATABASE_URL=postgresql://olympus:PASSWORD@postgres:5432/olympus
DB_PASSWORD=<strong-password>

# Redis
REDIS_URL=redis://:PASSWORD@redis:6379
REDIS_PASSWORD=<strong-password>

# Security
JWT_SECRET=<32-byte-secret>
ENCRYPTION_KEY=<32-byte-key>

# Monitoring
GRAFANA_USER=admin
GRAFANA_PASSWORD=<strong-password>

# Application
RUST_LOG=info
PORT=8080
ENVIRONMENT=production
```

### 3. Generate Secrets

```bash
# Generate JWT secret
openssl rand -base64 32

# Generate encryption key
openssl rand -hex 32

# Generate strong passwords
openssl rand -base64 24
```

## Database Setup

### 1. Create Production Database

```bash
# Connect to PostgreSQL
psql -h <host> -U postgres

# Create database and user
CREATE DATABASE olympus;
CREATE USER olympus WITH ENCRYPTED PASSWORD '<password>';
GRANT ALL PRIVILEGES ON DATABASE olympus TO olympus;

# Enable extensions
\c olympus
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
```

### 2. Run Migrations

```bash
# Install sqlx-cli
cargo install sqlx-cli

# Run migrations
DATABASE_URL=postgresql://olympus:password@localhost:5432/olympus \
  sqlx migrate run
```

### 3. Verify Schema

```bash
# Check migration status
sqlx migrate info

# Verify tables
psql -h localhost -U olympus -d olympus -c "\dt *.*"
```

## Docker Deployment

### 1. Build Production Image

```bash
# Build with production Dockerfile
docker build -f Dockerfile.production -t olympuscloud/rust-services:latest .

# Tag with version
docker tag olympuscloud/rust-services:latest \
  olympuscloud/rust-services:v1.0.0
```

### 2. Push to Registry

```bash
# Login to Docker Hub
docker login

# Push images
docker push olympuscloud/rust-services:latest
docker push olympuscloud/rust-services:v1.0.0
```

### 3. Deploy with Docker Compose

```bash
# Start services
docker-compose -f docker-compose.production.yml up -d

# Verify deployment
docker-compose -f docker-compose.production.yml ps

# Check logs
docker-compose -f docker-compose.production.yml logs -f rust-services
```

## GCP Cloud Run Deployment

### 1. Setup GCP Project

```bash
# Set project
gcloud config set project olympus-cloud-prod

# Enable required APIs
gcloud services enable \
  run.googleapis.com \
  cloudsql.googleapis.com \
  redis.googleapis.com \
  secretmanager.googleapis.com
```

### 2. Create Secrets

```bash
# Create secrets in Secret Manager
echo -n "postgresql://..." | gcloud secrets create database-url --data-file=-
echo -n "redis://..." | gcloud secrets create redis-url --data-file=-
echo -n "<jwt-secret>" | gcloud secrets create jwt-secret --data-file=-
```

### 3. Deploy to Cloud Run

```bash
# Deploy service
gcloud run deploy olympus-rust-services \
  --image olympuscloud/rust-services:latest \
  --region us-central1 \
  --platform managed \
  --memory 2Gi \
  --cpu 2 \
  --min-instances 1 \
  --max-instances 100 \
  --port 8080 \
  --set-secrets="DATABASE_URL=database-url:latest,REDIS_URL=redis-url:latest,JWT_SECRET=jwt-secret:latest" \
  --set-env-vars="RUST_LOG=info,ENVIRONMENT=production" \
  --allow-unauthenticated

# Get service URL
gcloud run services describe olympus-rust-services \
  --region us-central1 \
  --format 'value(status.url)'
```

### 4. Configure Custom Domain

```bash
# Map custom domain
gcloud run domain-mappings create \
  --service olympus-rust-services \
  --domain api.olympuscloud.io \
  --region us-central1

# Verify DNS records
gcloud run domain-mappings describe \
  --domain api.olympuscloud.io \
  --region us-central1
```

## Kubernetes Deployment

### 1. Create Namespace

```yaml
# k8s/namespace.yaml
apiVersion: v1
kind: Namespace
metadata:
  name: olympus-prod
```

### 2. Deploy Application

```yaml
# k8s/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: rust-services
  namespace: olympus-prod
spec:
  replicas: 3
  selector:
    matchLabels:
      app: rust-services
  template:
    metadata:
      labels:
        app: rust-services
    spec:
      containers:
      - name: rust-services
        image: olympuscloud/rust-services:latest
        ports:
        - containerPort: 8080
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: olympus-secrets
              key: database-url
        - name: REDIS_URL
          valueFrom:
            secretKeyRef:
              name: olympus-secrets
              key: redis-url
        resources:
          requests:
            memory: "512Mi"
            cpu: "500m"
          limits:
            memory: "2Gi"
            cpu: "2000m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
```

### 3. Apply Configuration

```bash
# Apply all configurations
kubectl apply -f k8s/

# Verify deployment
kubectl get pods -n olympus-prod

# Check service status
kubectl get svc -n olympus-prod
```

## Monitoring Setup

### 1. Configure Prometheus

```bash
# Apply Prometheus configuration
docker-compose -f docker-compose.production.yml up -d prometheus

# Access Prometheus UI
open http://localhost:9090
```

### 2. Setup Grafana Dashboards

```bash
# Import dashboards
curl -X POST http://localhost:3000/api/dashboards/import \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $GRAFANA_API_KEY" \
  -d @monitoring/dashboards/rust-services.json
```

### 3. Configure Alerts

```bash
# Test alert configuration
promtool check rules monitoring/alerts/*.yml

# Reload Prometheus
curl -X POST http://localhost:9090/-/reload
```

## Security Checklist

- [ ] All secrets stored in Secret Manager/Vault
- [ ] SSL/TLS certificates configured
- [ ] Security headers enabled
- [ ] Rate limiting configured
- [ ] Input validation enabled
- [ ] Audit logging active
- [ ] Database connections encrypted
- [ ] Redis password protected
- [ ] Firewall rules configured
- [ ] DDoS protection enabled
- [ ] Regular security updates scheduled
- [ ] Backup strategy implemented
- [ ] Disaster recovery plan tested

## Rollback Procedures

### Quick Rollback

```bash
# Cloud Run rollback
gcloud run services update-traffic olympus-rust-services \
  --to-revisions=olympus-rust-services-00001-abc=100 \
  --region us-central1

# Kubernetes rollback
kubectl rollout undo deployment/rust-services -n olympus-prod

# Docker rollback
docker-compose -f docker-compose.production.yml down
docker-compose -f docker-compose.production.yml up -d \
  --scale rust-services=0
docker-compose -f docker-compose.production.yml up -d \
  --scale rust-services=3
```

### Database Rollback

```bash
# Rollback migrations
sqlx migrate revert

# Restore from backup
pg_restore -h localhost -U olympus -d olympus backup.dump
```

## Troubleshooting

### Common Issues

#### Service Won't Start

```bash
# Check logs
docker logs olympus-rust-services

# Verify environment variables
docker exec olympus-rust-services env

# Test database connection
docker exec olympus-rust-services \
  psql $DATABASE_URL -c "SELECT 1"
```

#### High Memory Usage

```bash
# Check memory usage
docker stats olympus-rust-services

# Analyze heap dump
docker exec olympus-rust-services \
  kill -USR1 1
```

#### Performance Issues

```bash
# Run benchmarks
cargo bench

# Profile application
docker exec olympus-rust-services \
  perf record -g -p 1
```

### Health Check Endpoints

- `/health` - Overall system health
- `/ready` - Service readiness
- `/metrics` - Prometheus metrics
- `/live` - Liveness probe

### Log Levels

```bash
# Change log level at runtime
export RUST_LOG=debug
docker-compose restart rust-services
```

## Performance Tuning

### Database Optimization

```sql
-- Analyze query performance
EXPLAIN ANALYZE SELECT ...;

-- Create indexes
CREATE INDEX idx_orders_tenant_created
ON commerce.orders(tenant_id, created_at DESC);

-- Update statistics
ANALYZE;
```

### Connection Pooling

```env
# Optimize pool settings
DATABASE_MAX_CONNECTIONS=100
DATABASE_MIN_CONNECTIONS=10
DATABASE_CONNECT_TIMEOUT=30
```

### Redis Optimization

```bash
# Monitor Redis performance
redis-cli --stat

# Optimize memory
redis-cli CONFIG SET maxmemory-policy allkeys-lru
```

## Maintenance

### Regular Tasks

- **Daily**: Check logs and metrics
- **Weekly**: Review security alerts
- **Monthly**: Update dependencies
- **Quarterly**: Performance review
- **Annually**: Disaster recovery drill

### Backup Strategy

```bash
# Automated daily backups
0 2 * * * pg_dump -h localhost -U olympus olympus | gzip > /backups/olympus_$(date +\%Y\%m\%d).sql.gz

# Verify backups
gunzip -c /backups/olympus_20240101.sql.gz | head
```

## Support

For issues or questions:
- GitHub Issues: https://github.com/OlympusCloud/olympus-cloud-gcp/issues
- Documentation: https://docs.olympuscloud.io
- Emergency: Contact on-call engineer

---

**Last Updated**: 2024-01-19
**Version**: 1.0.0
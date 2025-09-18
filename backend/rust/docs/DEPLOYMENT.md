# Rust Services Deployment Guide

## Overview
This guide covers deploying the Rust services (auth, platform, commerce) to Google Cloud Run.

## Prerequisites
- Google Cloud Project with billing enabled
- Cloud SQL PostgreSQL instance
- Redis instance (Memorystore)
- Artifact Registry repository
- Service account with necessary permissions

## Build & Package

### Local Build
```bash
# Build optimized binary
cargo build --release

# Test the binary
./target/release/olympus-rust
```

### Docker Build
```bash
# Build multi-stage Docker image
docker build -t olympus-rust:latest .

# Test locally
docker run -p 8000:8000 \
  -e DATABASE_URL="postgres://user:pass@host/db" \
  -e REDIS_URL="redis://host:6379" \
  -e JWT_SECRET="your-secret" \
  olympus-rust:latest
```

### Optimized Dockerfile
```dockerfile
# Build stage
FROM rust:1.75-slim AS builder
WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY auth/Cargo.toml auth/
COPY platform/Cargo.toml platform/
COPY commerce/Cargo.toml commerce/
COPY shared/Cargo.toml shared/

# Build dependencies (cached layer)
RUN mkdir -p auth/src platform/src commerce/src shared/src \
    && echo "fn main() {}" > auth/src/main.rs \
    && echo "fn main() {}" > platform/src/main.rs \
    && echo "fn main() {}" > commerce/src/main.rs \
    && echo "" > shared/src/lib.rs \
    && cargo build --release

# Copy source code
COPY . .

# Rebuild with actual source
RUN touch auth/src/main.rs platform/src/main.rs commerce/src/main.rs \
    && cargo build --release

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/olympus-rust /app/olympus-rust
COPY --from=builder /app/migrations /app/migrations

# Non-root user
RUN useradd -m -u 1001 appuser && chown -R appuser:appuser /app
USER appuser

EXPOSE 8000

CMD ["./olympus-rust"]
```

## Environment Variables

### Required
```bash
DATABASE_URL=postgres://user:password@/dbname?host=/cloudsql/project:region:instance
REDIS_URL=redis://10.x.x.x:6379
JWT_SECRET=your-256-bit-secret
PORT=8000
RUST_LOG=info
```

### Optional
```bash
ENVIRONMENT=production
MAX_CONNECTIONS=100
CONNECTION_TIMEOUT=30
JWT_EXPIRY=3600
REFRESH_TOKEN_EXPIRY=604800
CORS_ORIGINS=https://yourdomain.com
```

## Google Cloud Deployment

### 1. Push to Artifact Registry
```bash
# Configure Docker for Artifact Registry
gcloud auth configure-docker us-central1-docker.pkg.dev

# Tag image
docker tag olympus-rust:latest \
  us-central1-docker.pkg.dev/PROJECT_ID/olympus/rust-services:latest

# Push image
docker push us-central1-docker.pkg.dev/PROJECT_ID/olympus/rust-services:latest
```

### 2. Deploy to Cloud Run
```bash
gcloud run deploy olympus-rust \
  --image us-central1-docker.pkg.dev/PROJECT_ID/olympus/rust-services:latest \
  --platform managed \
  --region us-central1 \
  --allow-unauthenticated \
  --service-account olympus-cloud-run@PROJECT_ID.iam.gserviceaccount.com \
  --add-cloudsql-instances PROJECT_ID:us-central1:olympus-postgres \
  --vpc-connector olympus-connector \
  --set-env-vars "ENVIRONMENT=production,RUST_LOG=info" \
  --set-secrets "DATABASE_URL=database-url:latest,JWT_SECRET=jwt-secret:latest,REDIS_URL=redis-url:latest" \
  --cpu 1 \
  --memory 512Mi \
  --min-instances 0 \
  --max-instances 10 \
  --timeout 300 \
  --port 8000
```

### 3. Configure Cloud SQL Proxy
The Cloud Run service automatically handles Cloud SQL connections via Unix socket:
```
/cloudsql/PROJECT_ID:REGION:INSTANCE_NAME
```

### 4. Set up Secrets
```bash
# Create secrets in Secret Manager
echo -n "postgres://olympus:password@/olympus?host=/cloudsql/project:region:instance" | \
  gcloud secrets create database-url --data-file=-

echo -n "redis://10.x.x.x:6379" | \
  gcloud secrets create redis-url --data-file=-

echo -n "your-jwt-secret" | \
  gcloud secrets create jwt-secret --data-file=-
```

## Database Migrations

### Run migrations before deployment
```bash
# Connect to Cloud SQL
gcloud sql connect olympus-postgres --user=olympus --database=olympus

# Run migration
\i migrations/001_initial_schema.sql
```

### Automated migrations (CI/CD)
```yaml
- name: Run migrations
  run: |
    wget https://dl.google.com/cloudsql/cloud_sql_proxy.linux.amd64 -O cloud_sql_proxy
    chmod +x cloud_sql_proxy
    ./cloud_sql_proxy -instances=${{ secrets.CLOUD_SQL_INSTANCE }}=tcp:5432 &
    sleep 5
    export DATABASE_URL="postgres://user:pass@localhost:5432/dbname"
    cargo install sqlx-cli
    sqlx migrate run
```

## Health Checks

Cloud Run will automatically check these endpoints:
- `/health` - Basic health (returns 200)
- `/ready` - Database and Redis connectivity
- `/live` - Process liveness

Configure in Cloud Run:
```bash
gcloud run services update olympus-rust \
  --health-check-path /health \
  --health-check-interval 30s \
  --health-check-timeout 10s \
  --health-check-initial-delay 10s
```

## Monitoring

### Enable Cloud Logging
```rust
// Structured logging for Cloud Logging
use tracing_subscriber::fmt::format::JsonFields;

tracing_subscriber::fmt()
    .json()
    .with_current_span(false)
    .with_span_list(false)
    .init();
```

### Metrics Export
```rust
// Export to Cloud Monitoring
use opentelemetry::sdk::metrics::MeterProvider;
use opentelemetry_gcp::CloudMonitoring;

let exporter = CloudMonitoring::new(project_id);
let meter_provider = MeterProvider::builder()
    .with_exporter(exporter)
    .build();
```

### Custom Metrics
```bash
# View metrics
gcloud monitoring metrics-descriptors list --filter="metric.type:custom.googleapis.com/olympus/*"
```

## Performance Tuning

### Connection Pooling
```rust
// Optimize database connections
let pool = PgPoolOptions::new()
    .max_connections(100)
    .min_connections(10)
    .acquire_timeout(Duration::from_secs(30))
    .idle_timeout(Duration::from_secs(600))
    .connect(&database_url)
    .await?;
```

### Redis Connection
```rust
// Use connection manager for Redis
let redis_manager = RedisConnectionManager::new(redis_url)?;
let redis_pool = Pool::builder()
    .max_size(50)
    .build(redis_manager)
    .await?;
```

### Cloud Run Configuration
```yaml
# Optimize for cold starts
spec:
  template:
    metadata:
      annotations:
        run.googleapis.com/cpu-throttling: "false"
        run.googleapis.com/startup-cpu-boost: "true"
    spec:
      containerConcurrency: 100
      timeoutSeconds: 300
```

## Security Considerations

### Service Account Permissions
Required IAM roles:
- `roles/cloudsql.client` - Connect to Cloud SQL
- `roles/secretmanager.secretAccessor` - Access secrets
- `roles/cloudtrace.agent` - Send traces
- `roles/monitoring.metricWriter` - Write metrics
- `roles/logging.logWriter` - Write logs

### Network Security
- Use VPC connector for private IP access
- Enable Cloud Armor for DDoS protection
- Implement rate limiting at application level

### Secrets Rotation
```bash
# Rotate JWT secret
echo -n "new-secret" | gcloud secrets versions add jwt-secret --data-file=-

# Update Cloud Run to use new version
gcloud run services update olympus-rust \
  --update-secrets JWT_SECRET=jwt-secret:latest
```

## Rollback Strategy

### Blue-Green Deployment
```bash
# Deploy to new revision with no traffic
gcloud run deploy olympus-rust \
  --image new-image \
  --no-traffic \
  --tag green

# Test green deployment
curl https://green---olympus-rust-xxxxx.run.app/health

# Switch traffic
gcloud run services update-traffic olympus-rust \
  --to-tags green=100
```

### Quick Rollback
```bash
# List revisions
gcloud run revisions list --service olympus-rust

# Rollback to previous revision
gcloud run services update-traffic olympus-rust \
  --to-revisions REVISION_NAME=100
```

## Troubleshooting

### View Logs
```bash
# Stream logs
gcloud logging tail "resource.type=cloud_run_revision AND resource.labels.service_name=olympus-rust"

# Query specific errors
gcloud logging read "resource.type=cloud_run_revision AND severity>=ERROR" --limit 50
```

### Common Issues

1. **Database Connection Failed**
   - Check Cloud SQL proxy is enabled
   - Verify DATABASE_URL format
   - Check VPC connector configuration

2. **Redis Connection Failed**
   - Verify Redis instance is in same VPC
   - Check firewall rules
   - Verify REDIS_URL has private IP

3. **Out of Memory**
   - Increase memory allocation
   - Check for memory leaks
   - Optimize connection pools

4. **Cold Start Latency**
   - Set minimum instances to 1
   - Enable CPU boost
   - Optimize Docker image size

## CI/CD Integration

### GitHub Actions Workflow
```yaml
name: Deploy Rust Services

on:
  push:
    branches: [main]
    paths:
      - 'backend/rust/**'

env:
  PROJECT_ID: ${{ secrets.GCP_PROJECT_ID }}
  REGION: us-central1
  SERVICE: olympus-rust

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - uses: google-github-actions/setup-gcloud@v1
        with:
          service_account_key: ${{ secrets.GCP_SA_KEY }}

      - name: Configure Docker
        run: gcloud auth configure-docker ${REGION}-docker.pkg.dev

      - name: Build and Push
        run: |
          docker build -t ${REGION}-docker.pkg.dev/${PROJECT_ID}/olympus/rust-services:${{ github.sha }} backend/rust
          docker push ${REGION}-docker.pkg.dev/${PROJECT_ID}/olympus/rust-services:${{ github.sha }}

      - name: Deploy to Cloud Run
        run: |
          gcloud run deploy ${SERVICE} \
            --image ${REGION}-docker.pkg.dev/${PROJECT_ID}/olympus/rust-services:${{ github.sha }} \
            --region ${REGION} \
            --platform managed
```

## Contact
For questions about Rust services, see `/backend/rust/STATUS.md`
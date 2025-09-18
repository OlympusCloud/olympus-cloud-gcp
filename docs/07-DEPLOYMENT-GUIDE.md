# 🚀 Olympus Cloud GCP - Complete Deployment Guide

> **Production-ready deployment configurations for GCP with Cloudflare Edge**

## 📋 Deployment Overview

```yaml
Architecture:
  Primary: Google Cloud Platform
  Edge: Cloudflare Workers
  Containers: Cloud Run (serverless)
  Orchestration: GKE Autopilot (managed K8s)
  Database: Cloud SQL PostgreSQL
  Cache: Memorystore Redis
  Analytics: BigQuery
  CI/CD: GitHub Actions
  Monitoring: Cloud Operations Suite
```

## 🏗️ Infrastructure as Code

### Terraform Main Configuration

```hcl
# infrastructure/terraform/main.tf

terraform {
  required_version = ">= 1.6.0"
  
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.10"
    }
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.20"
    }
    random = {
      source  = "hashicorp/random"
      version = "~> 3.6"
    }
  }
  
  backend "gcs" {
    bucket = "olympus-terraform-state"
    prefix = "terraform/state"
  }
}

# Variables
variable "project_id" {
  description = "GCP Project ID"
  type        = string
}

variable "region" {
  description = "Primary GCP region"
  type        = string
  default     = "us-central1"
}

variable "environment" {
  description = "Environment name"
  type        = string
  validation {
    condition     = contains(["dev", "staging", "prod"], var.environment)
    error_message = "Environment must be dev, staging, or prod."
  }
}

variable "cloudflare_zone_id" {
  description = "Cloudflare Zone ID"
  type        = string
  sensitive   = true
}

# Providers
provider "google" {
  project = var.project_id
  region  = var.region
}

provider "cloudflare" {
  api_token = var.cloudflare_api_token
}

# Enable required APIs
resource "google_project_service" "required_apis" {
  for_each = toset([
    "run.googleapis.com",
    "sqladmin.googleapis.com",
    "redis.googleapis.com",
    "bigquery.googleapis.com",
    "container.googleapis.com",
    "secretmanager.googleapis.com",
    "cloudkms.googleapis.com",
    "monitoring.googleapis.com",
    "logging.googleapis.com",
    "artifactregistry.googleapis.com",
  ])
  
  service            = each.value
  disable_on_destroy = false
}

# ============================================
# Networking
# ============================================

resource "google_compute_network" "vpc" {
  name                    = "olympus-vpc-${var.environment}"
  auto_create_subnetworks = false
  routing_mode            = "REGIONAL"
}

resource "google_compute_subnetwork" "subnet" {
  name          = "olympus-subnet-${var.environment}"
  ip_cidr_range = "10.0.0.0/24"
  network       = google_compute_network.vpc.id
  region        = var.region
  
  secondary_ip_range {
    range_name    = "pods"
    ip_cidr_range = "10.1.0.0/16"
  }
  
  secondary_ip_range {
    range_name    = "services"
    ip_cidr_range = "10.2.0.0/16"
  }
}

# Cloud NAT for outbound connectivity
resource "google_compute_router" "router" {
  name    = "olympus-router-${var.environment}"
  network = google_compute_network.vpc.id
  region  = var.region
}

resource "google_compute_router_nat" "nat" {
  name                               = "olympus-nat-${var.environment}"
  router                             = google_compute_router.router.name
  region                             = var.region
  nat_ip_allocate_option             = "AUTO_ONLY"
  source_subnetwork_ip_ranges_to_nat = "ALL_SUBNETWORKS_ALL_IP_RANGES"
}

# ============================================
# Cloud SQL PostgreSQL
# ============================================

resource "random_password" "db_password" {
  length  = 32
  special = true
}

resource "google_sql_database_instance" "postgres" {
  name             = "olympus-db-${var.environment}"
  database_version = "POSTGRES_15"
  region           = var.region
  
  settings {
    tier              = var.environment == "prod" ? "db-custom-4-16384" : "db-custom-2-8192"
    availability_type = var.environment == "prod" ? "REGIONAL" : "ZONAL"
    disk_size         = var.environment == "prod" ? 100 : 50
    disk_type         = "PD_SSD"
    disk_autoresize   = true
    
    backup_configuration {
      enabled                        = true
      start_time                     = "02:00"
      point_in_time_recovery_enabled = var.environment == "prod"
      transaction_log_retention_days = var.environment == "prod" ? 7 : 1
      
      backup_retention_settings {
        retained_backups = var.environment == "prod" ? 30 : 7
        retention_unit   = "COUNT"
      }
    }
    
    database_flags {
      name  = "max_connections"
      value = var.environment == "prod" ? "1000" : "200"
    }
    
    database_flags {
      name  = "shared_preload_libraries"
      value = "pg_stat_statements"
    }
    
    insights_config {
      query_insights_enabled  = true
      query_plans_per_minute  = 5
      query_string_length     = 1024
      record_application_tags = true
      record_client_address   = true
    }
    
    ip_configuration {
      ipv4_enabled    = false
      private_network = google_compute_network.vpc.id
      
      dynamic "authorized_networks" {
        for_each = var.environment == "dev" ? [1] : []
        content {
          name  = "dev-access"
          value = "0.0.0.0/0"
        }
      }
    }
    
    maintenance_window {
      day          = 7  # Sunday
      hour         = 4
      update_track = "stable"
    }
  }
  
  deletion_protection = var.environment == "prod"
}

resource "google_sql_database" "olympus" {
  name     = "olympus"
  instance = google_sql_database_instance.postgres.name
}

resource "google_sql_user" "app_user" {
  name     = "olympus_app"
  instance = google_sql_database_instance.postgres.name
  password = random_password.db_password.result
}

# ============================================
# Redis Cache
# ============================================

resource "google_redis_instance" "cache" {
  name               = "olympus-cache-${var.environment}"
  tier               = var.environment == "prod" ? "STANDARD_HA" : "BASIC"
  memory_size_gb     = var.environment == "prod" ? 5 : 1
  region             = var.region
  authorized_network = google_compute_network.vpc.id
  
  redis_version = "REDIS_7_0"
  display_name  = "Olympus Cache ${var.environment}"
  
  redis_configs = {
    maxmemory-policy = "allkeys-lru"
    notify-keyspace-events = "Ex"
  }
  
  maintenance_policy {
    weekly_maintenance_window {
      day = "SUNDAY"
      start_time {
        hours   = 2
        minutes = 0
      }
    }
  }
}

# ============================================
# Cloud Run Services
# ============================================

# Artifact Registry for container images
resource "google_artifact_registry_repository" "containers" {
  location      = var.region
  repository_id = "olympus-containers"
  format        = "DOCKER"
  
  cleanup_policies {
    id     = "keep-recent"
    action = "KEEP"
    
    condition {
      tag_state    = "TAGGED"
      tag_prefixes = ["v", "prod", "staging"]
    }
  }
  
  cleanup_policies {
    id     = "delete-old-untagged"
    action = "DELETE"
    
    condition {
      tag_state = "UNTAGGED"
      older_than = "7d"
    }
  }
}

# Service Account for Cloud Run
resource "google_service_account" "cloud_run" {
  account_id   = "olympus-cloud-run-${var.environment}"
  display_name = "Olympus Cloud Run Service Account"
}

resource "google_project_iam_member" "cloud_run_permissions" {
  for_each = toset([
    "roles/cloudsql.client",
    "roles/redis.editor",
    "roles/bigquery.dataEditor",
    "roles/secretmanager.secretAccessor",
    "roles/cloudtrace.agent",
  ])
  
  project = var.project_id
  role    = each.value
  member  = "serviceAccount:${google_service_account.cloud_run.email}"
}

# Cloud Run API Service
resource "google_cloud_run_service" "api" {
  name     = "olympus-api-${var.environment}"
  location = var.region
  
  template {
    spec {
      service_account_name = google_service_account.cloud_run.email
      
      containers {
        image = "${var.region}-docker.pkg.dev/${var.project_id}/olympus-containers/api:latest"
        
        resources {
          limits = {
            cpu    = var.environment == "prod" ? "4" : "2"
            memory = var.environment == "prod" ? "8Gi" : "4Gi"
          }
        }
        
        env {
          name  = "ENVIRONMENT"
          value = var.environment
        }
        
        env {
          name  = "PORT"
          value = "8080"
        }
        
        env {
          name = "DATABASE_URL"
          value_from {
            secret_key_ref {
              name = google_secret_manager_secret.db_url.secret_id
              key  = "latest"
            }
          }
        }
        
        env {
          name = "REDIS_URL"
          value_from {
            secret_key_ref {
              name = google_secret_manager_secret.redis_url.secret_id
              key  = "latest"
            }
          }
        }
        
        ports {
          container_port = 8080
        }
        
        startup_probe {
          http_get {
            path = "/health"
          }
          initial_delay_seconds = 10
          period_seconds        = 5
          failure_threshold     = 3
        }
        
        liveness_probe {
          http_get {
            path = "/health"
          }
          period_seconds    = 10
          failure_threshold = 3
        }
      }
    }
    
    metadata {
      annotations = {
        "autoscaling.knative.dev/minScale"         = var.environment == "prod" ? "2" : "0"
        "autoscaling.knative.dev/maxScale"         = var.environment == "prod" ? "100" : "10"
        "run.googleapis.com/cpu-throttling"        = "false"
        "run.googleapis.com/startup-cpu-boost"     = "true"
        "run.googleapis.com/vpc-access-connector"  = google_vpc_access_connector.connector.name
        "run.googleapis.com/vpc-access-egress"     = "all-traffic"
      }
    }
  }
  
  traffic {
    percent         = 100
    latest_revision = true
  }
  
  autogenerate_revision_name = true
}

# VPC Connector for Cloud Run
resource "google_vpc_access_connector" "connector" {
  name          = "olympus-connector-${var.environment}"
  region        = var.region
  ip_cidr_range = "10.8.0.0/28"
  network       = google_compute_network.vpc.name
  
  min_throughput = 200
  max_throughput = var.environment == "prod" ? 1000 : 300
}

# ============================================
# BigQuery Data Warehouse
# ============================================

resource "google_bigquery_dataset" "analytics" {
  dataset_id    = "olympus_analytics_${var.environment}"
  friendly_name = "Olympus Analytics ${var.environment}"
  location      = "US"
  
  default_table_expiration_ms = var.environment == "prod" ? null : 7776000000  # 90 days for non-prod
  
  labels = {
    environment = var.environment
    team        = "data"
  }
  
  access {
    role          = "OWNER"
    user_by_email = google_service_account.cloud_run.email
  }
}

# BigQuery tables
resource "google_bigquery_table" "events" {
  dataset_id = google_bigquery_dataset.analytics.dataset_id
  table_id   = "events"
  
  time_partitioning {
    type  = "DAY"
    field = "created_at"
  }
  
  clustering = ["tenant_id", "event_type"]
  
  schema = jsonencode([
    {
      name = "id"
      type = "STRING"
      mode = "REQUIRED"
    },
    {
      name = "tenant_id"
      type = "STRING"
      mode = "REQUIRED"
    },
    {
      name = "event_type"
      type = "STRING"
      mode = "REQUIRED"
    },
    {
      name = "event_data"
      type = "JSON"
      mode = "NULLABLE"
    },
    {
      name = "created_at"
      type = "TIMESTAMP"
      mode = "REQUIRED"
    }
  ])
}

# ============================================
# Secrets Management
# ============================================

resource "google_secret_manager_secret" "db_url" {
  secret_id = "db-url-${var.environment}"
  
  replication {
    automatic = true
  }
}

resource "google_secret_manager_secret_version" "db_url" {
  secret = google_secret_manager_secret.db_url.id
  
  secret_data = format(
    "postgresql://%s:%s@%s/%s?sslmode=require",
    google_sql_user.app_user.name,
    google_sql_user.app_user.password,
    google_sql_database_instance.postgres.private_ip_address,
    google_sql_database.olympus.name
  )
}

resource "google_secret_manager_secret" "redis_url" {
  secret_id = "redis-url-${var.environment}"
  
  replication {
    automatic = true
  }
}

resource "google_secret_manager_secret_version" "redis_url" {
  secret = google_secret_manager_secret.redis_url.id
  
  secret_data = format(
    "redis://%s:6379",
    google_redis_instance.cache.host
  )
}

# ============================================
# Cloudflare Edge Configuration
# ============================================

resource "cloudflare_record" "api" {
  zone_id = var.cloudflare_zone_id
  name    = var.environment == "prod" ? "api" : "api-${var.environment}"
  value   = google_cloud_run_service.api.status[0].url
  type    = "CNAME"
  proxied = true
}

resource "cloudflare_worker_script" "edge" {
  name    = "olympus-edge-${var.environment}"
  content = file("${path.module}/edge-worker.js")
  
  plain_text_binding {
    name = "JWT_SECRET"
    text = var.jwt_secret
  }
  
  kv_namespace_binding {
    name         = "CACHE"
    namespace_id = cloudflare_workers_kv_namespace.cache.id
  }
  
  service_binding {
    name        = "API"
    service     = google_cloud_run_service.api.status[0].url
    environment = var.environment
  }
}

resource "cloudflare_workers_kv_namespace" "cache" {
  title = "olympus-cache-${var.environment}"
}

resource "cloudflare_worker_route" "api" {
  zone_id     = var.cloudflare_zone_id
  pattern     = var.environment == "prod" ? "api.olympuscloud.io/*" : "api-${var.environment}.olympuscloud.io/*"
  script_name = cloudflare_worker_script.edge.name
}

# ============================================
# Monitoring and Alerting
# ============================================

resource "google_monitoring_uptime_check_config" "api" {
  display_name = "API Health Check - ${var.environment}"
  timeout      = "10s"
  period       = "60s"
  
  http_check {
    path         = "/health"
    port         = "443"
    use_ssl      = true
    validate_ssl = true
  }
  
  monitored_resource {
    type = "uptime_url"
    labels = {
      project_id = var.project_id
      host       = var.environment == "prod" ? "api.olympuscloud.io" : "api-${var.environment}.olympuscloud.io"
    }
  }
}

resource "google_monitoring_alert_policy" "high_latency" {
  display_name = "High API Latency - ${var.environment}"
  combiner     = "OR"
  
  conditions {
    display_name = "95th percentile latency > 500ms"
    
    condition_threshold {
      filter          = "resource.type = \"cloud_run_revision\" AND metric.type = \"run.googleapis.com/request_latencies\""
      duration        = "300s"
      comparison      = "COMPARISON_GT"
      threshold_value = 500
      
      aggregations {
        alignment_period     = "60s"
        per_series_aligner   = "ALIGN_PERCENTILE_95"
        cross_series_reducer = "REDUCE_MEAN"
        
        group_by_fields = ["resource.service_name"]
      }
    }
  }
  
  notification_channels = [google_monitoring_notification_channel.email.name]
  
  alert_strategy {
    auto_close = "1800s"
  }
}

resource "google_monitoring_notification_channel" "email" {
  display_name = "Email Notification"
  type         = "email"
  
  labels = {
    email_address = "alerts@olympuscloud.io"
  }
}

# ============================================
# Outputs
# ============================================

output "api_url" {
  value = google_cloud_run_service.api.status[0].url
}

output "database_connection" {
  value     = google_sql_database_instance.postgres.connection_name
  sensitive = true
}

output "redis_host" {
  value = google_redis_instance.cache.host
}

output "cloudflare_worker" {
  value = cloudflare_worker_script.edge.name
}
```

## 🐳 Docker Configuration

### API Dockerfile

```dockerfile
# backend/Dockerfile.api

# Build stage for Rust
FROM rust:1.75 AS rust-builder

WORKDIR /app
COPY backend/rust/Cargo.toml backend/rust/Cargo.lock ./
COPY backend/rust/src ./src

# Build release binary
RUN cargo build --release --bin api

# Build stage for Go
FROM golang:1.21-alpine AS go-builder

WORKDIR /app
COPY backend/go/go.mod backend/go/go.sum ./
RUN go mod download

COPY backend/go ./
RUN CGO_ENABLED=0 GOOS=linux go build -o api-gateway ./cmd/api

# Build stage for Python
FROM python:3.11-slim AS python-builder

WORKDIR /app
COPY backend/python/requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt --target=/app/deps

COPY backend/python ./

# Final stage
FROM gcr.io/distroless/cc-debian12

# Copy Rust binary
COPY --from=rust-builder /app/target/release/api /usr/local/bin/api-rust

# Copy Go binary
COPY --from=go-builder /app/api-gateway /usr/local/bin/api-gateway

# Copy Python deps and code
COPY --from=python-builder /app/deps /app/deps
COPY --from=python-builder /app /app/python

# Set environment
ENV PYTHONPATH=/app/deps
ENV PORT=8080

# Run the Go API gateway as entry point
ENTRYPOINT ["/usr/local/bin/api-gateway"]
```

### Docker Compose for Development

```yaml
# docker-compose.yml
version: '3.9'

services:
  postgres:
    image: postgres:15-alpine
    environment:
      POSTGRES_DB: olympus
      POSTGRES_USER: olympus
      POSTGRES_PASSWORD: devpassword
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./docs/05-COMPLETE-DATABASE-SCHEMA.sql:/docker-entrypoint-initdb.d/schema.sql
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U olympus"]
      interval: 10s
      timeout: 5s
      retries: 5

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    command: redis-server --appendonly yes
    volumes:
      - redis_data:/data
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 10s
      timeout: 5s
      retries: 5

  api:
    build:
      context: .
      dockerfile: backend/Dockerfile.api
    ports:
      - "8080:8080"
    environment:
      DATABASE_URL: postgresql://olympus:devpassword@postgres:5432/olympus?sslmode=disable
      REDIS_URL: redis://redis:6379
      JWT_SECRET: dev-secret-change-in-production
      ENVIRONMENT: development
    depends_on:
      postgres:
        condition: service_healthy
      redis:
        condition: service_healthy
    volumes:
      - ./backend:/app
    command: ["air", "-c", "/app/.air.toml"]

  frontend:
    build:
      context: ./frontend
      dockerfile: Dockerfile.dev
    ports:
      - "3000:3000"
    environment:
      API_URL: http://localhost:8080
    volumes:
      - ./frontend:/app
      - /app/node_modules
    command: ["flutter", "run", "-d", "web-server", "--web-port", "3000", "--web-hostname", "0.0.0.0"]

volumes:
  postgres_data:
  redis_data:
```

## 🔄 CI/CD Pipeline

### GitHub Actions Workflow

```yaml
# .github/workflows/deploy.yml
name: Deploy to GCP

on:
  push:
    branches:
      - main
      - staging
      - develop
  pull_request:
    branches:
      - main

env:
  PROJECT_ID: olympus-cloud
  REGION: us-central1
  REGISTRY: us-central1-docker.pkg.dev

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        service: [rust, go, python, flutter]
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup ${{ matrix.service }}
        uses: ./.github/actions/setup-${{ matrix.service }}
      
      - name: Run tests
        run: make test-${{ matrix.service }}
      
      - name: Upload coverage
        uses: codecov/codecov-action@v3
        with:
          files: ./coverage-${{ matrix.service }}.xml

  build:
    needs: test
    runs-on: ubuntu-latest
    if: github.event_name == 'push'
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Set up Cloud SDK
        uses: google-github-actions/setup-gcloud@v2
        with:
          service_account_key: ${{ secrets.GCP_SA_KEY }}
          project_id: ${{ env.PROJECT_ID }}
      
      - name: Configure Docker
        run: gcloud auth configure-docker ${{ env.REGISTRY }}
      
      - name: Build and push API image
        run: |
          docker build -t ${{ env.REGISTRY }}/${{ env.PROJECT_ID }}/olympus-containers/api:${{ github.sha }} \
            -f backend/Dockerfile.api .
          docker push ${{ env.REGISTRY }}/${{ env.PROJECT_ID }}/olympus-containers/api:${{ github.sha }}
      
      - name: Build and push Flutter web
        run: |
          cd frontend
          flutter build web --release
          docker build -t ${{ env.REGISTRY }}/${{ env.PROJECT_ID }}/olympus-containers/web:${{ github.sha }} .
          docker push ${{ env.REGISTRY }}/${{ env.PROJECT_ID }}/olympus-containers/web:${{ github.sha }}

  deploy:
    needs: build
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main' || github.ref == 'refs/heads/staging'
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Set environment
        run: |
          if [ "${{ github.ref }}" == "refs/heads/main" ]; then
            echo "ENVIRONMENT=prod" >> $GITHUB_ENV
          else
            echo "ENVIRONMENT=staging" >> $GITHUB_ENV
          fi
      
      - name: Deploy to Cloud Run
        uses: google-github-actions/deploy-cloudrun@v2
        with:
          service: olympus-api-${{ env.ENVIRONMENT }}
          image: ${{ env.REGISTRY }}/${{ env.PROJECT_ID }}/olympus-containers/api:${{ github.sha }}
          region: ${{ env.REGION }}
          env_vars: |
            ENVIRONMENT=${{ env.ENVIRONMENT }}
          
      - name: Run migrations
        run: |
          gcloud run jobs execute migrate-${{ env.ENVIRONMENT }} --region=${{ env.REGION }}
      
      - name: Verify deployment
        run: |
          RESPONSE=$(curl -s -o /dev/null -w "%{http_code}" https://api-${{ env.ENVIRONMENT }}.olympuscloud.io/health)
          if [ $RESPONSE != "200" ]; then
            echo "Health check failed with status $RESPONSE"
            exit 1
          fi

  rollback:
    needs: deploy
    runs-on: ubuntu-latest
    if: failure()
    
    steps:
      - name: Rollback to previous version
        run: |
          gcloud run services update-traffic olympus-api-${{ env.ENVIRONMENT }} \
            --to-revisions=LATEST=0 \
            --region=${{ env.REGION }}
      
      - name: Notify team
        uses: 8398a7/action-slack@v3
        with:
          status: custom
          custom_payload: |
            {
              text: "Deployment failed and rolled back",
              attachments: [{
                color: 'danger',
                title: 'Deployment Rollback',
                text: 'Environment: ${{ env.ENVIRONMENT }}\nCommit: ${{ github.sha }}'
              }]
            }
        env:
          SLACK_WEBHOOK_URL: ${{ secrets.SLACK_WEBHOOK }}
```

## 🔐 Security Configuration

### Cloud Armor WAF Rules

```hcl
# infrastructure/terraform/security.tf

resource "google_compute_security_policy" "waf" {
  name = "olympus-waf-${var.environment}"
  
  # Rate limiting rule
  rule {
    action   = "rate_based_ban"
    priority = "1000"
    
    match {
      versioned_expr = "SRC_IPS_V1"
      config {
        src_ip_ranges = ["*"]
      }
    }
    
    rate_limit_options {
      conform_action = "allow"
      exceed_action   = "deny(429)"
      
      rate_limit_threshold {
        count        = 100
        interval_sec = 60
      }
      
      ban_duration_sec = 600
    }
  }
  
  # SQL Injection protection
  rule {
    action   = "deny(403)"
    priority = "2000"
    
    match {
      expr {
        expression = "evaluatePreconfiguredExpr('sqli-stable')"
      }
    }
  }
  
  # XSS protection
  rule {
    action   = "deny(403)"
    priority = "2001"
    
    match {
      expr {
        expression = "evaluatePreconfiguredExpr('xss-stable')"
      }
    }
  }
  
  # Default allow rule
  rule {
    action   = "allow"
    priority = "2147483647"
    match {
      versioned_expr = "SRC_IPS_V1"
      config {
        src_ip_ranges = ["*"]
      }
    }
  }
}
```

## 📊 Monitoring Configuration

### Prometheus Configuration

```yaml
# monitoring/prometheus.yml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'api'
    static_configs:
      - targets: ['api:8080']
    metrics_path: '/metrics'

  - job_name: 'postgres'
    static_configs:
      - targets: ['postgres-exporter:9187']

  - job_name: 'redis'
    static_configs:
      - targets: ['redis-exporter:9121']

  - job_name: 'cloudrun'
    gce_sd_configs:
      - project: olympus-cloud
        zone: us-central1-a
        filter: 'labels.app="olympus"'
```

### Grafana Dashboard

```json
{
  "dashboard": {
    "title": "Olympus Cloud Metrics",
    "panels": [
      {
        "title": "API Response Time",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))"
          }
        ]
      },
      {
        "title": "Request Rate",
        "targets": [
          {
            "expr": "rate(http_requests_total[5m])"
          }
        ]
      },
      {
        "title": "Error Rate",
        "targets": [
          {
            "expr": "rate(http_requests_total{status=~\"5..\"}[5m])"
          }
        ]
      },
      {
        "title": "Database Connections",
        "targets": [
          {
            "expr": "pg_stat_database_numbackends"
          }
        ]
      },
      {
        "title": "Cache Hit Rate",
        "targets": [
          {
            "expr": "redis_keyspace_hits_total / (redis_keyspace_hits_total + redis_keyspace_misses_total)"
          }
        ]
      }
    ]
  }
}
```

## 🚀 Deployment Commands

### Quick Deployment Script

```bash
#!/bin/bash
# deploy.sh

set -e

ENVIRONMENT=${1:-staging}
REGION="us-central1"
PROJECT="olympus-cloud"

echo "🚀 Deploying to $ENVIRONMENT..."

# Build and push images
echo "📦 Building containers..."
make build-all

echo "⬆️ Pushing to registry..."
make push-$ENVIRONMENT

# Deploy infrastructure
echo "🏗️ Applying Terraform..."
cd infrastructure/terraform
terraform workspace select $ENVIRONMENT
terraform apply -auto-approve -var="environment=$ENVIRONMENT"

# Run migrations
echo "🗄️ Running database migrations..."
gcloud run jobs execute migrate-$ENVIRONMENT --region=$REGION --wait

# Deploy services
echo "🚀 Deploying services..."
gcloud run deploy olympus-api-$ENVIRONMENT \
  --image=$REGION-docker.pkg.dev/$PROJECT/olympus-containers/api:latest \
  --region=$REGION \
  --platform=managed \
  --no-traffic

# Run smoke tests
echo "🧪 Running smoke tests..."
./scripts/smoke-test.sh $ENVIRONMENT

# Switch traffic
echo "🔄 Switching traffic..."
gcloud run services update-traffic olympus-api-$ENVIRONMENT \
  --to-latest \
  --region=$REGION

echo "✅ Deployment complete!"
echo "🌐 API URL: https://api-$ENVIRONMENT.olympuscloud.io"
```

---

**This deployment guide provides everything needed to deploy Olympus Cloud to production on Google Cloud Platform with enterprise-grade reliability and security.**
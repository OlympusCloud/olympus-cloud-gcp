# Olympus Cloud GCP Infrastructure

This directory contains Terraform configurations for deploying the Olympus Cloud platform on Google Cloud Platform.

## Architecture Overview

The infrastructure is designed as a modular, scalable, and secure cloud-native platform supporting multiple business verticals (Restaurant, Retail, Salon, Events, Hospitality).

### Core Components

- **Compute**: Cloud Run services for containerized applications
- **Database**: Cloud SQL PostgreSQL with high availability
- **Cache**: Redis Memory Store for session and application caching
- **Storage**: Cloud Storage for application assets
- **Analytics**: BigQuery for data warehousing and analytics
- **Networking**: VPC with private service access
- **Security**: Secret Manager for sensitive data
- **Monitoring**: Cloud Monitoring with alerting
- **CDN**: Cloudflare for global content delivery

## Quick Start

### Prerequisites

- Google Cloud SDK installed and configured
- Terraform >= 1.6.0
- Access to GCP project with appropriate permissions
- Cloudflare account and API token

### Environment Setup

1. **Clone and navigate to infrastructure directory**:
   ```bash
   cd infrastructure/terraform
   ```

2. **Copy and configure variables**:
   ```bash
   cp terraform.tfvars.example terraform.tfvars
   # Edit terraform.tfvars with your specific values
   ```

3. **Initialize Terraform**:
   ```bash
   terraform init
   ```

4. **Plan deployment**:
   ```bash
   terraform plan
   ```

5. **Deploy infrastructure**:
   ```bash
   terraform apply
   ```

## Configuration

### Required Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `project_id` | GCP Project ID | `olympus-cloud-prod` |
| `region` | Primary GCP region | `us-central1` |
| `environment` | Environment name | `prod`, `staging`, `dev` |
| `domain` | Primary domain | `olympuscloud.io` |
| `alert_email` | Email for alerts | `alerts@company.com` |
| `cloudflare_api_token` | Cloudflare API token | `your-token-here` |

### Optional Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `vpc_connector_cidr` | `10.8.0.0/28` | VPC connector CIDR range |
| `db_password` | Generated | Database password (auto-generated if not provided) |

## Module Structure

### Core Modules

- **`networking`**: VPC, subnets, firewall rules, private service access
- **`database`**: Cloud SQL PostgreSQL and Redis Memory Store
- **`compute`**: Cloud Run services and Artifact Registry
- **`storage`**: Cloud Storage buckets with lifecycle policies
- **`security`**: Secret Manager and IAM policies
- **`monitoring`**: Cloud Monitoring, alerting, and uptime checks
- **`analytics`**: BigQuery datasets and tables

### External Integrations

- **`cloudflare`**: DNS management and CDN configuration

## Environments

### Development
- Single-zone database
- Basic tier Redis
- Minimal compute resources
- Cost-optimized configuration

### Staging
- Regional database
- Standard Redis
- Production-like configuration
- Automated testing integration

### Production
- High availability database
- Standard HA Redis
- Auto-scaling compute
- Comprehensive monitoring
- Backup and disaster recovery

## Security Features

### Data Protection
- All data encrypted at rest and in transit
- Private IP addresses for databases
- VPC-native networking
- Secret Manager for sensitive data

### Access Control
- IAM policies with least privilege
- Service accounts for applications
- Network-level security controls
- Audit logging enabled

### Compliance
- SOC 2 Type II ready
- GDPR compliance features
- PCI DSS considerations
- HIPAA-eligible services where applicable

## Monitoring & Alerting

### Metrics Monitored
- Application performance (latency, errors, throughput)
- Infrastructure health (CPU, memory, disk)
- Database performance (connections, queries, replication)
- Security events (failed logins, unusual access patterns)

### Alert Policies
- High error rates (>5% for 5 minutes)
- High latency (>2s for 5 minutes)
- Database connection issues
- Storage quota warnings (>80%)
- Budget alerts for cost control

## Cost Management

### Cost Optimization
- Committed use discounts for predictable workloads
- Preemptible instances for batch processing
- Storage lifecycle policies
- Resource scheduling for non-production

### Budget Controls
- Environment-specific budgets
- Automated alerts at 50%, 80%, 100% of budget
- Cost allocation by service and environment
- Regular cost reviews and optimization

## Disaster Recovery

### Backup Strategy
- Automated database backups (daily, 30-day retention)
- Point-in-time recovery enabled
- Cross-region backup replication
- Application data backup to Cloud Storage

### Recovery Procedures
- RTO: 4 hours for production
- RPO: 1 hour for production
- Automated failover for database
- Infrastructure as Code for rapid rebuild

## CI/CD Integration

### GitHub Actions Workflows
- Terraform validation and security scanning
- Automated deployment to staging
- Manual approval for production
- Rollback capabilities

### Security Scanning
- `tfsec` for Terraform security analysis
- `terraform validate` for syntax checking
- `tflint` for best practices
- Dependency vulnerability scanning

## Troubleshooting

### Common Issues

**Terraform State Lock**
```bash
# If state is locked, force unlock (use carefully)
terraform force-unlock LOCK_ID
```

**Permission Denied**
```bash
# Ensure proper GCP authentication
gcloud auth application-default login
gcloud config set project YOUR_PROJECT_ID
```

**Resource Quota Exceeded**
```bash
# Check and request quota increases
gcloud compute project-info describe --project=YOUR_PROJECT_ID
```

### Debugging

**Enable Terraform Debug Logging**
```bash
export TF_LOG=DEBUG
terraform apply
```

**Check Resource Status**
```bash
# Cloud SQL
gcloud sql instances list

# Cloud Run
gcloud run services list

# VPC
gcloud compute networks list
```

## Maintenance

### Regular Tasks
- Review and update Terraform modules monthly
- Security patch management
- Cost optimization reviews
- Performance monitoring and tuning
- Backup verification

### Upgrade Procedures
1. Test changes in development environment
2. Apply to staging environment
3. Validate functionality
4. Schedule maintenance window for production
5. Apply changes with rollback plan ready

## Support

### Documentation
- [Terraform GCP Provider](https://registry.terraform.io/providers/hashicorp/google/latest/docs)
- [Google Cloud Documentation](https://cloud.google.com/docs)
- [Cloudflare API Documentation](https://api.cloudflare.com/)

### Getting Help
- Internal documentation in `/docs` directory
- Architecture decision records in `/docs/adr`
- Runbooks in `/docs/runbooks`
- Contact: infrastructure@olympuscloud.io

---

**Last Updated**: 2025-09-18
**Version**: 1.0.0
**Maintained By**: Google Gemini Infrastructure Team
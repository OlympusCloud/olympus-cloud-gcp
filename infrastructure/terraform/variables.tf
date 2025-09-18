# GCP Project Variables
variable "project_id" {
  description = "The GCP project ID"
  type        = string
}

variable "region" {
  description = "The GCP region for resources"
  type        = string
  default     = "us-central1"
}

variable "zone" {
  description = "The GCP zone for resources"
  type        = string
  default     = "us-central1-a"
}

variable "environment" {
  description = "Environment name (dev, staging, prod)"
  type        = string
  default     = "dev"
}

# Network Variables
variable "vpc_network_name" {
  description = "Name of the VPC network"
  type        = string
  default     = "olympus-vpc"
}

variable "subnet_cidr" {
  description = "CIDR range for the main subnet"
  type        = string
  default     = "10.0.0.0/24"
}

# Database Variables
variable "db_tier" {
  description = "The tier for the Cloud SQL instance"
  type        = string
  default     = "db-f1-micro"
}

variable "db_disk_size" {
  description = "Initial disk size for Cloud SQL in GB"
  type        = number
  default     = 10
}

variable "db_max_disk_size" {
  description = "Maximum disk size for Cloud SQL in GB"
  type        = number
  default     = 100
}

variable "db_name" {
  description = "The name of the database"
  type        = string
  default     = "olympus"
}

variable "db_user" {
  description = "The database username"
  type        = string
  default     = "olympus"
}

variable "db_password" {
  description = "The database password"
  type        = string
  sensitive   = true
}

# Redis Variables
variable "redis_tier" {
  description = "The service tier of the Redis instance"
  type        = string
  default     = "BASIC"
}

variable "redis_memory_gb" {
  description = "Redis memory size in GB"
  type        = number
  default     = 1
}

# Cloud Run Variables
variable "api_cpu" {
  description = "CPU allocation for API service"
  type        = string
  default     = "1"
}

variable "api_memory" {
  description = "Memory allocation for API service"
  type        = string
  default     = "512Mi"
}

variable "min_instances" {
  description = "Minimum number of Cloud Run instances"
  type        = number
  default     = 0
}

variable "max_instances" {
  description = "Maximum number of Cloud Run instances"
  type        = number
  default     = 10
}

variable "container_concurrency" {
  description = "Max concurrent requests per container"
  type        = number
  default     = 80
}

variable "request_timeout" {
  description = "Request timeout in seconds"
  type        = number
  default     = 300
}

# Docker Images
variable "api_image" {
  description = "Docker image for the API service"
  type        = string
  default     = "gcr.io/PROJECT_ID/olympus-api:latest"
}

# Secrets
variable "jwt_secret_name" {
  description = "Name of the JWT secret in Secret Manager"
  type        = string
  default     = "jwt-secret"
}

variable "database_url_secret_name" {
  description = "Name of the database URL secret"
  type        = string
  default     = "database-url"
}
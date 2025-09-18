 variable "project_id" {
   description = "GCP Project ID"
 variable "environment" {
   description = "Environment name (e.g., 'dev', 'staging', 'prod')"
   type        = string
 }
 
 variable "region" {
   description = "Primary GCP region"
   type        = string
 }
 
 variable "environment" {
   description = "Environment name (e.g., 'dev', 'staging', 'prod')"
 variable "db_tier" {
   description = "The machine type for the Cloud SQL instance."
 variable "subnet_cidr" {
   description = "The primary CIDR block for the subnet."
   type        = string
 }
 
 variable "vpc_network_name" {
   description = "The self-link of the VPC network."
 variable "pods_cidr" {
   description = "The secondary CIDR block for GKE pods."
   type        = string
 }
 
 variable "connector_cidr" {
   description = "CIDR range for the VPC Access Connector."
 variable "services_cidr" {
   description = "The secondary CIDR block for GKE services."
   type        = string
 }
 
 variable "connector_min_throughput" {
   description = "Minimum throughput of the VPC connector in Mbps."
 variable "db_disk_size" {
   description = "The initial disk size for the Cloud SQL instance in GB."
   type        = number
 }
 
 variable "connector_max_throughput" {
   description = "Maximum throughput of the VPC connector in Mbps."
 variable "db_max_disk_size" {
   description = "The maximum disk size for the Cloud SQL instance in GB."
   type        = number
 }
 
 variable "min_instances" {
   description = "Minimum number of Cloud Run instances."
   type        = number
 }
 
 variable "max_instances" {
   description = "Maximum number of Cloud Run instances."
   type        = number
 }
 
 variable "db_connection_name" {
   description = "Cloud SQL instance connection name."
 variable "vpc_network_id" {
   description = "The self-link of the VPC network."
   type        = string
 }
 
 variable "container_concurrency" {
   description = "Number of concurrent requests a Cloud Run container can receive."
   type        = number
 variable "private_vpc_connection_dependency" {
   description = "A dependency to ensure the private VPC connection is created before this module."
 variable "api_dependency" {
   description = "A dependency to ensure APIs are enabled before creating the network."
   type        = any
   default     = null
 }
 
 variable "request_timeout" {
   description = "Request timeout in seconds for Cloud Run."
   type        = number
 }
 
 variable "api_image" {
   description = "Docker image for the API service."
 variable "redis_tier" {
   description = "The service tier of the Redis instance."
   type        = string
 }
 
 variable "database_url_secret_name" {
   description = "The name of the Secret Manager secret containing the Database URL."
   type        = string
 }
 
 variable "redis_host" {
   description = "The host of the Redis instance."
   type        = string
 }
 
 variable "redis_port" {
   description = "The port of the Redis instance."
 variable "redis_memory_gb" {
   description = "The memory size of the Redis instance in GB."
   type        = number
 }
 
 variable "jwt_secret_name" {
   description = "The name of the Secret Manager secret containing the JWT secret."
   type        = string
 }
 
 variable "api_cpu" {
   description = "CPU limit for the API container."
   type        = string
 }
 
 variable "api_memory" {
   description = "Memory limit for the API container."
   type        = string
 }

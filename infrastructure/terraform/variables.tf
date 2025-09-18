variable "project_id" {
  description = "The GCP project ID."
  type        = string
}

variable "region" {
  description = "The GCP region."
  type        = string
}

variable "environment" {
  description = "The deployment environment."
  type        = string
}

variable "vpc_id" {
  description = "The ID of the VPC network."
  type        = string
}

variable "vpc_connector_cidr" {
  description = "The CIDR range for the VPC Access Connector."
  type        = string
}

variable "api_dependency_artifactregistry" {
  description = "Dependency for the Artifact Registry API."
  type        = any
}

variable "api_dependency_vpcaccess" {
  description = "Dependency for the VPC Access API."
  type        = any
}

variable "sql_instance_dependency" {
  description = "Dependency for the Cloud SQL instance."
  type        = any
}

variable "redis_instance_dependency" {
  description = "Dependency for the Redis instance."
  type        = any
}
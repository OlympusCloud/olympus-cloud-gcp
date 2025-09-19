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
  }

  backend "gcs" {
    bucket = "olympus-terraform-state"
    prefix = "terraform/state"
  }
}

provider "google" {
  project = var.project_id
  region  = var.region
}

provider "cloudflare" {
  api_token = var.cloudflare_api_token
}

module "networking" {
  source = "./modules/networking"

  project_id        = var.project_id
  region            = var.region
  environment       = var.environment
  subnet_cidr_range = "10.0.0.0/24"
  api_dependency    = google_project_service.servicenetworking
}

module "database" {
  source = "./modules/database"

  project_id             = var.project_id
  region                 = var.region
  environment            = var.environment
  db_password            = var.db_password
  vpc_id                 = module.networking.vpc_id
  private_vpc_connection = module.networking.private_vpc_connection
}

module "compute" {
  source = "./modules/compute"

  project_id                      = var.project_id
  region                          = var.region
  environment                     = var.environment
  vpc_id                          = module.networking.vpc_id
  vpc_connector_cidr              = var.vpc_connector_cidr
  api_dependency_artifactregistry = google_project_service.artifactregistry
  api_dependency_vpcaccess        = google_project_service.vpcaccess
  sql_instance_dependency         = module.database.sql_instance
  redis_instance_dependency       = module.database.redis_instance
}

module "monitoring" {
  source = "./modules/monitoring"

  project_id             = var.project_id
  environment            = var.environment
  alert_email            = var.alert_email
  cloud_run_service_name = module.compute.api_service_name # Assumes output from compute module
  api_domain             = local.api_domain

  depends_on = [
    module.compute,
    # module.cloudflare, # Cloudflare module not present in this file's context
  ]
}

locals {
  # Construct the API domain based on environment for the uptime check
  # This assumes a `domain` variable exists, which is standard practice.
  api_domain = var.environment == "prod" ? "api.${var.domain}" : "api-${var.environment}.${var.domain}"
}

variable "project_id" {
  description = "GCP Project ID"
  type        = string
}

variable "region" {
  description = "GCP Region"
  type        = string
  default     = "us-central1"
}

variable "environment" {
  description = "The deployment environment (e.g., 'dev', 'staging', 'prod')."
  type        = string
}

variable "domain" {
  description = "The primary domain name for the application."
  type        = string
  default     = "olympuscloud.io"
}

variable "alert_email" {
  description = "The email address for sending monitoring alerts."
  type        = string
}

# The following variables are assumed to exist for other modules
# variable "db_password" { ... }
# variable "vpc_connector_cidr" { ... }
# resource "google_project_service" "servicenetworking" { ... }
# resource "google_project_service" "artifactregistry" { ... }
# resource "google_project_service" "vpcaccess" { ... }
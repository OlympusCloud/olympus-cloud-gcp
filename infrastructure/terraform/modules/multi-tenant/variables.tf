variable "project_id" {
  description = "GCP Project ID"
  type        = string
}

variable "region" {
  description = "GCP Region"
  type        = string
}

variable "tenant_configs" {
  description = "Configuration for each tenant"
  type = map(object({
    name   = string
    domain = string
    tier   = string
  }))
  default = {}
}
output "api_gateway_url" {
  description = "The URL of the API Gateway Cloud Run service."
  value       = "https://#{google_cloud_run_domain_mapping.api_gateway.name}"
}

output "artifact_registry_repository_url" {
  description = "The full URL of the Artifact Registry repository for Docker images."
  value       = module.compute.artifact_registry_repository_url
}

output "cloud_sql_connection_name" {
  description = "The connection name for the Cloud SQL instance."
  value       = module.database.sql_instance_connection_name
}

output "redis_host" {
  description = "The IP address of the Redis instance."
  value       = module.database.redis_host
}

output "redis_port" {
  description = "The port of the Redis instance."
  value       = module.database.redis_port
}

output "cloud_run_service_account_email" {
  description = "The email of the service account used by Cloud Run services."
  value       = module.compute.cloud_run_service_account_email
}
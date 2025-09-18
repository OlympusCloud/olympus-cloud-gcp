 output "api_service_uri" {
   description = "The URI of the API Cloud Run service."
   value       = google_cloud_run_v2_service.api.uri
 output "instance_connection_name" {
   description = "The connection name of the Cloud SQL instance."
   value       = google_sql_database_instance.postgres.connection_name
 output "vpc_id" {
   description = "The ID of the VPC network."
   value       = google_compute_network.vpc.id
 }
 
 output "api_service_name" {
   description = "The name of the API Cloud Run service."
   value       = google_cloud_run_v2_service.api.name
 output "instance_private_ip" {
   description = "The private IP address of the Cloud SQL instance."
   value       = google_sql_database_instance.postgres.private_ip_address
 output "vpc_name" {
   description = "The name of the VPC network."
   value       = google_compute_network.vpc.name
 }
 
 output "artifact_registry_id" {
   description = "The ID of the Artifact Registry repository."
   value       = google_artifact_registry_repository.containers.id
 output "db_name" {
   description = "The name of the main application database."
   value       = google_sql_database.olympus.name
 output "subnet_id" {
   description = "The ID of the subnet."
   value       = google_compute_subnetwork.subnet.id
 }
 
 output "db_user_name" {
   description = "The name of the application database user."
   value       = google_sql_user.app_user.name
 output "private_vpc_connection" {
   description = "The private VPC connection resource for dependency ordering."
   value       = google_service_networking_connection.private_vpc_connection
 }
 
 output "db_password" {
   description = "The generated password for the database user."
   value       = random_password.db_password.result
   sensitive   = true
 }
 
 output "redis_host" {
   description = "The host of the Redis instance."
   value       = google_redis_instance.cache.host
 }
 
 output "redis_port" {
   description = "The port of the Redis instance."
   value       = google_redis_instance.cache.port
 }

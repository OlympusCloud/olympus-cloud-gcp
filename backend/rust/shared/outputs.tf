:
 output "instance_connection_name" {
   description = "The connection name of the Cloud SQL instance."
   value       = google_sql_database_instance.postgres.connection_name
 }
 
 output "instance_private_ip" {
   description = "The private IP address of the Cloud SQL instance."
   value       = google_sql_database_instance.postgres.private_ip_address
 }
 
 output "db_name" {
   description = "The name of the main application database."
   value       = google_sql_database.olympus.name
 }
 
 output "db_user_name" {
   description = "The name of the application database user."
   value       = google_sql_user.app_user.name
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

output "replica_instance_connection_name" {
  description = "The connection name of the Cloud SQL read replica instance. Only available in prod."
  value       = join("", google_sql_database_instance.replica.*.connection_name)
}
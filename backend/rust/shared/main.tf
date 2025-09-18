 # Generate secure password for the database user
 resource "random_password" "db_password" {
   length           = 32
   special          = true
   override_special = "!#$%&*()-_=+[]{}<>:?"
 }
 
 # Cloud SQL PostgreSQL instance
 resource "google_sql_database_instance" "postgres" {
   name             = "olympus-db-${var.environment}"
   database_version = "POSTGRES_15"
   region           = var.region
 
   settings {
     tier                  = var.db_tier
     availability_type     = var.environment == "prod" ? "REGIONAL" : "ZONAL"
     disk_type             = "PD_SSD"
     disk_size             = var.db_disk_size
     disk_autoresize       = true
     disk_autoresize_limit = var.db_max_disk_size
 
     backup_configuration {
       enabled                        = true
       start_time                     = "03:00"
       point_in_time_recovery_enabled = var.environment == "prod"
       backup_retention_settings {
         retained_backups = var.environment == "prod" ? 30 : 7
       }
     }
 
     ip_configuration {
       ipv4_enabled    = false
       private_network = var.vpc_network_id
     }
 
     database_flags {
       name  = "cloudsql.iam_authentication"
       value = "on"
     }
 
     database_flags {
       name  = "shared_preload_libraries"
       value = "pg_stat_statements"
     }
 
     database_flags {
       name  = "log_statement"
       value = "ddl" # 'all' can be very verbose and costly. 'ddl' is a safer default.
     }
 
     insights_config {
       query_insights_enabled  = true
       record_application_tags = true
       record_client_address   = true
     }
 
     maintenance_window {
       day          = 7 # Sunday
       hour         = 3 # 3 AM
       update_track = "stable"
     }
   }
 
   deletion_protection = var.environment == "prod"
 
   depends_on = [var.private_vpc_connection_dependency]
 }
 
 # Main application database
 resource "google_sql_database" "olympus" {
   name      = "olympus"
   instance  = google_sql_database_instance.postgres.name
   charset   = "UTF8"
   collation = "en_US.UTF8"
 }
 
 # Application user
 resource "google_sql_user" "app_user" {
   name     = "olympus_app"
   instance = google_sql_database_instance.postgres.name
   password = random_password.db_password.result
 }
 
 # Redis for caching and sessions
 resource "google_redis_instance" "cache" {
   name           = "olympus-cache-${var.environment}"
   tier           = var.redis_tier
   memory_size_gb = var.redis_memory_gb
   region         = var.region
 
   location_id             = "${var.region}-a"
   alternative_location_id = var.environment == "prod" ? "${var.region}-b" : null
 
   authorized_network = var.vpc_network_id
   connect_mode       = "PRIVATE_SERVICE_ACCESS"
 
   redis_version = "REDIS_7_0"
   display_name  = "Olympus Cache ${title(var.environment)}"
 
   redis_configs = {
     maxmemory-policy = "allkeys-lru"
     timeout          = "300"
   }
 
   maintenance_policy {
     weekly_maintenance_window {
       day = "SUNDAY"
       start_time {
         hours   = 3
         minutes = 0
         seconds = 0
         nanos   = 0
       }
     }
   }
 
   depends_on = [var.private_vpc_connection_dependency]
 }

# Read replica for the production environment
resource "google_sql_database_instance" "replica" {
  count = var.environment == "prod" ? 1 : 0

  name                 = "${google_sql_database_instance.postgres.name}-replica"
  master_instance_name = google_sql_database_instance.postgres.name
  database_version     = "POSTGRES_15"
  region               = var.region

  settings {
    tier              = var.db_tier # Replicas should generally match the primary's tier
    availability_type = "ZONAL"     # Replicas are Zonal
    disk_type         = "PD_SSD"
    disk_autoresize   = true

    ip_configuration {
      ipv4_enabled    = false
      private_network = var.vpc_network_id
    }

    database_flags {
      name  = "hot_standby"
      value = "on"
    }
  }
}
 # Artifact Registry for container images
 resource "google_artifact_registry_repository" "containers" {
   location      = var.region
   repository_id = "olympus-containers"
   description   = "Olympus application containers"
   format        = "DOCKER"
 resource "google_compute_network" "vpc" {
   name                    = "olympus-vpc-${var.environment}"
   auto_create_subnetworks = false
   routing_mode            = "REGIONAL"
 
   cleanup_policies {
     id     = "keep-recent"
     action = "KEEP"
     most_recent_versions {
       keep_count = 10
     }
   }
 
   cleanup_policies {
     id         = "delete-old"
     action     = "DELETE"
     condition {
       older_than = "2592000s" # 30 days
     }
   }
 # Generate secure password for the database user
 resource "random_password" "db_password" {
   length           = 32
   special          = true
   override_special = "!#$%&*()-_=+[]{}<>:?"
   depends_on = [var.api_dependency]
 }
 
 # Service Account for Cloud Run
 resource "google_service_account" "cloud_run" {
   account_id   = "olympus-cloud-run-${var.environment}"
   display_name = "Olympus Cloud Run Service Account"
 }
 # Cloud SQL PostgreSQL instance
 resource "google_sql_database_instance" "postgres" {
   name             = "olympus-db-${var.environment}"
   database_version = "POSTGRES_15"
   region           = var.region
 
 # IAM bindings for service account
 resource "google_project_iam_member" "cloud_run_permissions" {
   for_each = toset([
     "roles/cloudsql.client",
     "roles/secretmanager.secretAccessor",
     "roles/bigquery.dataEditor",
     "roles/bigquery.jobUser",
     "roles/monitoring.metricWriter",
     "roles/logging.logWriter",
     "roles/redis.editor",
   ])
   settings {
     tier                  = var.db_tier
     availability_type     = var.environment == "prod" ? "REGIONAL" : "ZONAL"
     disk_type             = "PD_SSD"
     disk_size             = var.db_disk_size
     disk_autoresize       = true
     disk_autoresize_limit = var.db_max_disk_size
 
   role    = each.value
   member  = "serviceAccount:${google_service_account.cloud_run.email}"
   project = var.project_id
 }
     backup_configuration {
       enabled                        = true
       start_time                     = "03:00"
       point_in_time_recovery_enabled = var.environment == "prod"
       backup_retention_settings {
         retained_backups = var.environment == "prod" ? 30 : 7
       }
     }
 
 # VPC Connector for Cloud Run
 resource "google_vpc_access_connector" "connector" {
   name          = "olympus-connector-${var.environment}"
 resource "google_compute_subnetwork" "subnet" {
   name          = "olympus-subnet-${var.environment}"
   ip_cidr_range = var.subnet_cidr
   region        = var.region
   network       = var.vpc_network_name
   ip_cidr_range = var.connector_cidr
     ip_configuration {
       ipv4_enabled    = false
       private_network = var.vpc_network_id
     }
   network       = google_compute_network.vpc.id
 
   min_throughput = var.connector_min_throughput
   max_throughput = var.connector_max_throughput
 }
     database_flags {
       name  = "cloudsql.iam_authentication"
       value = "on"
     }
   # Enable private IP Google access
   private_ip_google_access = true
 
 # Cloud Run API Service
 resource "google_cloud_run_v2_service" "api" {
   name     = "olympus-api-${var.environment}"
   location = var.region
     database_flags {
       name  = "shared_preload_libraries"
       value = "pg_stat_statements"
     }
   # Secondary ranges for pods and services (if using GKE later)
   secondary_ip_range {
     range_name    = "pods"
     ip_cidr_range = var.pods_cidr
   }
 
   template {
     scaling {
       min_instance_count = var.min_instances
       max_instance_count = var.max_instances
     database_flags {
       name  = "log_statement"
       value = "ddl" # 'all' can be very verbose and costly. 'ddl' is a safer default.
     }
 
     volumes {
       name = "cloudsql"
       cloud_sql_instance {
         instances = [var.db_connection_name]
       }
     insights_config {
       query_insights_enabled  = true
       record_application_tags = true
       record_client_address   = true
     }
 
     vpc_access {
       connector = google_vpc_access_connector.connector.id
       egress    = "PRIVATE_RANGES_ONLY"
     maintenance_window {
       day          = 7 # Sunday
       hour         = 3 # 3 AM
       update_track = "stable"
     }
   secondary_ip_range {
     range_name    = "services"
     ip_cidr_range = var.services_cidr
   }
 
     service_account     = google_service_account.cloud_run.email
     container_concurrency = var.container_concurrency
     timeout             = "${var.request_timeout}s"
   deletion_protection = var.environment == "prod"
 
     containers {
       image = var.api_image
   depends_on = [var.private_vpc_connection_dependency]
 }
 
       ports {
         container_port = 8080
       }
 # Main application database
 resource "google_sql_database" "olympus" {
   name      = "olympus"
   instance  = google_sql_database_instance.postgres.name
   charset   = "UTF8"
   collation = "en_US.UTF8"
 # Cloud NAT for outbound connectivity
 resource "google_compute_router" "router" {
   name    = "olympus-router-${var.environment}"
   network = google_compute_network.vpc.id
   region  = var.region
 }
 
       env {
         name  = "ENVIRONMENT"
         value = var.environment
       }
 # Application user
 resource "google_sql_user" "app_user" {
   name     = "olympus_app"
   instance = google_sql_database_instance.postgres.name
   password = random_password.db_password.result
 }
 resource "google_compute_router_nat" "nat" {
   name   = "olympus-nat-${var.environment}"
   router = google_compute_router.router.name
   region = var.region
 
       env {
         name = "DATABASE_URL"
         value_source {
           secret_key_ref {
             secret  = var.database_url_secret_name
             version = "latest"
           }
         }
       }
 # Redis for caching and sessions
 resource "google_redis_instance" "cache" {
   name           = "olympus-cache-${var.environment}"
   tier           = var.redis_tier
   memory_size_gb = var.redis_memory_gb
   region         = var.region
   nat_ip_allocate_option             = "AUTO_ONLY"
   source_subnetwork_ip_ranges_to_nat = "ALL_SUBNETWORKS_ALL_IP_RANGES"
 
       env {
         name  = "REDIS_URL"
         value = "redis://${var.redis_host}:${var.redis_port}"
       }
   location_id             = "${var.region}-a"
   alternative_location_id = var.environment == "prod" ? "${var.region}-b" : null
 
       env {
         name = "JWT_SECRET"
         value_source {
           secret_key_ref {
             secret  = var.jwt_secret_name
             version = "latest"
           }
         }
       }
   authorized_network = var.vpc_network_id
   connect_mode       = "PRIVATE_SERVICE_ACCESS"
 
       resources {
         limits = {
           cpu    = var.api_cpu
           memory = var.api_memory
         }
         cpu_idle = false # Corresponds to "CPU is always allocated"
       }
   redis_version = "REDIS_7_0"
   display_name  = "Olympus Cache ${title(var.environment)}"
 
       liveness_probe {
         http_get {
           path = "/health"
         }
         initial_delay_seconds = 30
         period_seconds        = 10
         timeout_seconds       = 5
         failure_threshold     = 3
       }
   redis_configs = {
     maxmemory-policy = "allkeys-lru"
     timeout          = "300"
   log_config {
     enable = true
     filter = "ERRORS_ONLY"
   }
 }
 
       startup_probe {
         http_get {
           path = "/health"
         }
         initial_delay_seconds = 10
         period_seconds        = 3
         timeout_seconds       = 1
         failure_threshold     = 10
   maintenance_policy {
     weekly_maintenance_window {
       day = "SUNDAY"
       start_time {
         hours   = 3
         minutes = 0
         seconds = 0
         nanos   = 0
       }
 
       volume_mounts {
         name       = "cloudsql"
         mount_path = "/cloudsql"
       }
     }
   }
 
   traffic {
     type    = "TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST"
     percent = 100
   }
 
   depends_on = [
     google_project_iam_member.cloud_run_permissions,
     google_vpc_access_connector.connector,
   ]
   depends_on = [var.private_vpc_connection_dependency]
 # Private connection for Cloud SQL and other Google services
 resource "google_compute_global_address" "private_ip_address" {
   name          = "olympus-private-ip-${var.environment}"
   purpose       = "VPC_PEERING"
   address_type  = "INTERNAL"
   prefix_length = 16
   network       = google_compute_network.vpc.id
 }
 
 # IAM for public access (development) or authenticated access (production)
 resource "google_cloud_run_service_iam_member" "public_access" {
   count = var.environment == "dev" ? 1 : 0
 
   service  = google_cloud_run_v2_service.api.name
   location = google_cloud_run_v2_service.api.location
   role     = "roles/run.invoker"
   member   = "allUsers"
 resource "google_service_networking_connection" "private_vpc_connection" {
   network                 = google_compute_network.vpc.id
   service                 = "servicenetworking.googleapis.com"
   reserved_peering_ranges = [google_compute_global_address.private_ip_address.name]
 }

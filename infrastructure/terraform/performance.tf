resource "google_redis_instance" "cache_optimization" {
  name           = "olympus-cache-${var.environment}"
  tier           = var.environment == "prod" ? "STANDARD_HA" : "BASIC"
  memory_size_gb = var.environment == "prod" ? 4 : 1
  region         = var.region
  
  redis_configs = {
    maxmemory-policy = "allkeys-lru"
    timeout = "300"
  }
}

resource "google_compute_global_address" "cdn_ip" {
  name = "olympus-cdn-ip"
}

resource "google_compute_backend_bucket" "static_assets" {
  name        = "olympus-static-backend"
  bucket_name = google_storage_bucket.industry_assets.name
  enable_cdn  = true
}

resource "google_compute_url_map" "cdn_map" {
  name = "olympus-cdn-map"
  
  default_service = google_compute_backend_bucket.static_assets.id
}

resource "google_monitoring_alert_policy" "high_latency" {
  display_name = "High API Latency"
  combiner     = "OR"
  
  conditions {
    display_name = "API latency > 2s"
    
    condition_threshold {
      filter         = "resource.type=\"cloud_run_revision\""
      comparison     = "COMPARISON_GREATER_THAN"
      threshold_value = 2000
      duration       = "300s"
    }
  }
  
  notification_channels = [google_monitoring_notification_channel.email.name]
}

resource "google_monitoring_notification_channel" "email" {
  display_name = "Email Alerts"
  type         = "email"
  
  labels = {
    email_address = var.alert_email
  }
}
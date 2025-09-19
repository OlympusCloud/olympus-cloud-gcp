resource "google_bigquery_dataset" "restaurant_analytics" {
  dataset_id = "restaurant_analytics"
  location   = var.region
  
  access {
    role   = "READER"
    domain = var.domain
  }
}

resource "google_bigquery_table" "table_analytics" {
  dataset_id = google_bigquery_dataset.restaurant_analytics.dataset_id
  table_id   = "table_analytics"
  
  schema = jsonencode([
    {
      name = "table_id"
      type = "STRING"
      mode = "REQUIRED"
    },
    {
      name = "occupancy_duration"
      type = "INTEGER"
      mode = "NULLABLE"
    },
    {
      name = "revenue"
      type = "FLOAT"
      mode = "NULLABLE"
    },
    {
      name = "timestamp"
      type = "TIMESTAMP"
      mode = "REQUIRED"
    }
  ])
}

resource "google_bigquery_dataset" "retail_analytics" {
  dataset_id = "retail_analytics"
  location   = var.region
}

resource "google_bigquery_table" "product_performance" {
  dataset_id = google_bigquery_dataset.retail_analytics.dataset_id
  table_id   = "product_performance"
  
  schema = jsonencode([
    {
      name = "product_id"
      type = "STRING"
      mode = "REQUIRED"
    },
    {
      name = "sales_count"
      type = "INTEGER"
      mode = "NULLABLE"
    },
    {
      name = "revenue"
      type = "FLOAT"
      mode = "NULLABLE"
    }
  ])
}

resource "google_storage_bucket" "industry_assets" {
  name     = "${var.project_id}-industry-assets"
  location = var.region
  
  uniform_bucket_level_access = true
}
resource "google_project_iam_custom_role" "tenant_isolation" {
  role_id     = "tenantIsolation"
  title       = "Tenant Isolation Role"
  permissions = [
    "cloudsql.instances.connect",
    "storage.objects.get",
    "bigquery.datasets.get",
    "bigquery.jobs.create"
  ]
}

resource "google_service_account" "tenant_service" {
  for_each = var.tenant_configs
  
  account_id   = "tenant-${each.key}"
  display_name = "Service Account for ${each.value.name}"
}

resource "google_storage_bucket" "tenant_assets" {
  for_each = var.tenant_configs
  
  name     = "${var.project_id}-tenant-${each.key}-assets"
  location = var.region
  
  uniform_bucket_level_access = true
}

resource "google_bigquery_dataset" "tenant_analytics" {
  for_each = var.tenant_configs
  
  dataset_id = "tenant_${each.key}_analytics"
  location   = var.region
  
  access {
    role          = "OWNER"
    user_by_email = google_service_account.tenant_service[each.key].email
  }
}
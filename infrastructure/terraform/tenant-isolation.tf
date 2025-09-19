module "multi_tenant" {
  source = "./modules/multi-tenant"
  
  project_id = var.project_id
  region     = var.region
  
  tenant_configs = {
    "demo-restaurant" = {
      name   = "Demo Restaurant"
      domain = "demo-restaurant.olympuscloud.io"
      tier   = "demo"
    }
    "acme-retail" = {
      name   = "ACME Retail"
      domain = "acme-retail.olympuscloud.io"
      tier   = "enterprise"
    }
  }
}

resource "google_sql_database" "tenant_databases" {
  for_each = module.multi_tenant.tenant_configs
  
  name     = "tenant_${each.key}"
  instance = google_sql_database_instance.olympus_postgres.name
}

resource "google_cloud_run_service_iam_member" "tenant_access" {
  for_each = module.multi_tenant.tenant_service_accounts
  
  service  = google_cloud_run_service.api.name
  location = var.region
  role     = "roles/run.invoker"
  member   = "serviceAccount:${each.value}"
}
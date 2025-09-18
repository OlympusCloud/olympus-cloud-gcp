# Map the custom domain to the API Gateway Cloud Run service
resource "google_cloud_run_domain_mapping" "api_gateway" {
  project  = var.project_id
  location = var.region
  name     = var.domain_name

  spec {
    route_name     = module.compute.api_gateway_service.name
    force_override = true
  }
}

# Create Cloudflare DNS records based on the domain mapping status.
# This automatically creates the CNAME/A/AAAA records required by Google Cloud Run for domain verification and traffic routing.
resource "cloudflare_record" "run_dns" {
  for_each = { for r in google_cloud_run_domain_mapping.api_gateway.status[0].resource_records : r.rrdata => r }

  zone_id = var.cloudflare_zone_id
  name    = each.value.name
  value   = trimsuffix(each.value.rrdata, ".")
  type    = each.value.type
  proxied = false # Must be false for Google's verification and for the service to work correctly with Google's load balancer.
  ttl     = 1     # Use a short TTL
}
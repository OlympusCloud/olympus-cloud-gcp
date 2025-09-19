// ============================================================================
// OLYMPUS CLOUD - LOCATION SERVICE
// ============================================================================
// Module: platform/src/services/location.rs
// Description: Location management service for multi-location businesses
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{Row, query, query_as};

use olympus_shared::{
    database::DbPool,
    events::{EventPublisher, DomainEvent},
    error::{Result, OlympusError},
};

use crate::models::{
    Location, LocationType, LocationStatus, LocationHierarchy, LocationAssignment,
    LocationStatistics, LocationSearchRequest, LocationSearchResponse,
    CreateLocationRequest, UpdateLocationRequest, CreateLocationAssignmentRequest,
    Address, ContactInfo, BusinessHours,
};

#[derive(Clone)]
pub struct LocationService {
    db: Arc<DbPool>,
    event_publisher: Arc<EventPublisher>,
}

impl LocationService {
    pub fn new(db: Arc<DbPool>, event_publisher: Arc<EventPublisher>) -> Self {
        Self { db, event_publisher }
    }

    // ============================================================================
    // LOCATION CRUD OPERATIONS
    // ============================================================================

    pub async fn create_location(
        &self,
        tenant_id: Uuid,
        request: CreateLocationRequest,
        created_by: Uuid,
    ) -> Result<Location> {
        let location_id = Uuid::new_v4();
        let now = Utc::now();

        // Validate parent location exists and belongs to tenant
        if let Some(parent_id) = request.parent_location_id {
            self.validate_parent_location(tenant_id, parent_id).await?;
        }

        // Validate location code is unique within tenant
        self.validate_location_code_unique(tenant_id, &request.code, None).await?;

        // Serialize JSON fields
        let address_json = serde_json::to_value(&request.address)
            .map_err(|e| OlympusError::Validation(format!("Invalid address: {}", e)))?;

        let contact_info_json = serde_json::to_value(&request.contact_info.unwrap_or_default())
            .map_err(|e| OlympusError::Validation(format!("Invalid contact info: {}", e)))?;

        let business_hours_json = serde_json::to_value(&request.business_hours)
            .map_err(|e| OlympusError::Validation(format!("Invalid business hours: {}", e)))?;

        let settings_json = request.settings.unwrap_or_else(|| serde_json::json!({}));
        let metadata_json = request.metadata.unwrap_or_else(|| serde_json::json!({}));

        // Insert location
        let location = query_as!(
            LocationRow,
            r#"
            INSERT INTO locations (
                id, tenant_id, parent_location_id, code, name, description,
                location_type, status, address, contact_info, business_hours,
                manager_user_id, capacity, area_square_feet, settings, metadata,
                is_public, created_at, updated_at, created_by, updated_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21)
            RETURNING
                id, tenant_id, parent_location_id, code, name, description,
                location_type as "location_type: LocationType",
                status as "status: LocationStatus",
                address, contact_info, business_hours, manager_user_id,
                capacity, area_square_feet, settings, metadata, is_public,
                created_at, updated_at, created_by, updated_by
            "#,
            location_id,
            tenant_id,
            request.parent_location_id,
            request.code,
            request.name,
            request.description,
            request.location_type as LocationType,
            LocationStatus::Active as LocationStatus,
            address_json,
            contact_info_json,
            business_hours_json,
            request.manager_user_id,
            request.capacity,
            request.area_square_feet,
            settings_json,
            metadata_json,
            request.is_public,
            now,
            now,
            created_by,
            created_by
        )
        .fetch_one(self.db.as_ref())
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to create location: {}", e)))?;

        let location = self.location_row_to_model(location)?;

        // Publish domain event
        let event = DomainEvent::builder()
            .data(serde_json::json!({
                "location_id": location_id,
                "tenant_id": tenant_id,
                "code": request.code,
                "name": request.name,
                "location_type": request.location_type,
                "parent_location_id": request.parent_location_id,
                "created_by": created_by
            }))
            .source_service("platform")
            .build();

        if let Err(e) = self.event_publisher.publish(&event).await {
            tracing::warn!("Failed to publish LocationCreated event: {}", e);
        }

        Ok(location)
    }

    pub async fn get_location(&self, tenant_id: Uuid, location_id: Uuid) -> Result<Option<Location>> {
        let location_row = query_as!(
            LocationRow,
            r#"
            SELECT
                id, tenant_id, parent_location_id, code, name, description,
                location_type as "location_type: LocationType",
                status as "status: LocationStatus",
                address, contact_info, business_hours, manager_user_id,
                capacity, area_square_feet, settings, metadata, is_public,
                created_at, updated_at, created_by, updated_by
            FROM locations
            WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL
            "#,
            location_id,
            tenant_id
        )
        .fetch_optional(self.db.as_ref())
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to get location: {}", e)))?;

        match location_row {
            Some(row) => Ok(Some(self.location_row_to_model(row)?)),
            None => Ok(None),
        }
    }

    pub async fn update_location(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        request: UpdateLocationRequest,
        updated_by: Uuid,
    ) -> Result<Option<Location>> {
        let now = Utc::now();

        // Build dynamic update query based on provided fields
        let mut updates = Vec::new();
        let mut values: Vec<&(dyn sqlx::postgres::PgHasArrayType + Sync)> = Vec::new();
        let mut param_count = 1;

        if let Some(name) = &request.name {
            updates.push(format!("name = ${}", param_count));
            param_count += 1;
        }

        if let Some(description) = &request.description {
            updates.push(format!("description = ${}", param_count));
            param_count += 1;
        }

        if request.location_type.is_some() {
            updates.push(format!("location_type = ${}", param_count));
            param_count += 1;
        }

        if request.status.is_some() {
            updates.push(format!("status = ${}", param_count));
            param_count += 1;
        }

        if updates.is_empty() {
            return self.get_location(tenant_id, location_id).await;
        }

        updates.push(format!("updated_at = ${}", param_count));
        param_count += 1;
        updates.push(format!("updated_by = ${}", param_count));

        let query_str = format!(
            r#"
            UPDATE locations
            SET {}
            WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL
            RETURNING
                id, tenant_id, parent_location_id, code, name, description,
                location_type, status, address, contact_info, business_hours,
                manager_user_id, capacity, area_square_feet, settings, metadata,
                is_public, created_at, updated_at, created_by, updated_by
            "#,
            updates.join(", ")
        );

        // Note: This is a simplified implementation. A full implementation would
        // use dynamic query building with proper parameter binding.
        // For now, we'll implement a basic update for name only.

        if let Some(name) = request.name {
            let location_row = query_as!(
                LocationRow,
                r#"
                UPDATE locations
                SET name = $3, updated_at = $4, updated_by = $5
                WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL
                RETURNING
                    id, tenant_id, parent_location_id, code, name, description,
                    location_type as "location_type: LocationType",
                    status as "status: LocationStatus",
                    address, contact_info, business_hours, manager_user_id,
                    capacity, area_square_feet, settings, metadata, is_public,
                    created_at, updated_at, created_by, updated_by
                "#,
                location_id,
                tenant_id,
                name,
                now,
                updated_by
            )
            .fetch_optional(self.db.as_ref())
            .await
            .map_err(|e| OlympusError::Database(format!("Failed to update location: {}", e)))?;

            match location_row {
                Some(row) => {
                    let location = self.location_row_to_model(row)?;

                    // Publish domain event
                    let event = DomainEvent::builder()
                        .data(serde_json::json!({
                            "location_id": location_id,
                            "tenant_id": tenant_id,
                            "updated_fields": ["name"],
                            "updated_by": updated_by
                        }))
                        .source_service("platform")
                        .build();

                    if let Err(e) = self.event_publisher.publish(&event).await {
                        tracing::warn!("Failed to publish LocationUpdated event: {}", e);
                    }

                    Ok(Some(location))
                }
                None => Ok(None),
            }
        } else {
            self.get_location(tenant_id, location_id).await
        }
    }

    pub async fn delete_location(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        deleted_by: Uuid,
    ) -> Result<bool> {
        let now = Utc::now();

        // Check if location has children
        let child_count = query!(
            "SELECT COUNT(*) as count FROM locations WHERE parent_location_id = $1 AND deleted_at IS NULL",
            location_id
        )
        .fetch_one(self.db.as_ref())
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to check child locations: {}", e)))?
        .count
        .unwrap_or(0);

        if child_count > 0 {
            return Err(OlympusError::Validation(
                "Cannot delete location that has child locations".to_string()
            ));
        }

        // Soft delete the location
        let rows_affected = query!(
            r#"
            UPDATE locations
            SET deleted_at = $3, updated_by = $4
            WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL
            "#,
            location_id,
            tenant_id,
            now,
            deleted_by
        )
        .execute(self.db.as_ref())
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to delete location: {}", e)))?
        .rows_affected();

        if rows_affected > 0 {
            // Publish domain event
            let event = DomainEvent::builder()
                .data(serde_json::json!({
                    "location_id": location_id,
                    "tenant_id": tenant_id,
                    "deleted_by": deleted_by
                }))
                .source_service("platform")
                .build();

            if let Err(e) = self.event_publisher.publish(&event).await {
                tracing::warn!("Failed to publish LocationDeleted event: {}", e);
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    // ============================================================================
    // LOCATION HIERARCHY OPERATIONS
    // ============================================================================

    pub async fn get_location_hierarchy(&self, tenant_id: Uuid, root_location_id: Option<Uuid>) -> Result<Vec<LocationHierarchy>> {
        // This is a simplified implementation. A full implementation would use
        // recursive CTEs to build the complete hierarchy efficiently.

        let locations = if let Some(root_id) = root_location_id {
            // Get hierarchy starting from specific location
            query_as!(
                LocationRow,
                r#"
                WITH RECURSIVE location_tree AS (
                    SELECT
                        id, tenant_id, parent_location_id, code, name, description,
                        location_type, status, address, contact_info, business_hours,
                        manager_user_id, capacity, area_square_feet, settings, metadata,
                        is_public, created_at, updated_at, created_by, updated_by,
                        0 as depth, ARRAY[id] as path
                    FROM locations
                    WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL

                    UNION ALL

                    SELECT
                        l.id, l.tenant_id, l.parent_location_id, l.code, l.name, l.description,
                        l.location_type, l.status, l.address, l.contact_info, l.business_hours,
                        l.manager_user_id, l.capacity, l.area_square_feet, l.settings, l.metadata,
                        l.is_public, l.created_at, l.updated_at, l.created_by, l.updated_by,
                        lt.depth + 1, lt.path || l.id
                    FROM locations l
                    INNER JOIN location_tree lt ON l.parent_location_id = lt.id
                    WHERE l.deleted_at IS NULL
                )
                SELECT
                    id, tenant_id, parent_location_id, code, name, description,
                    location_type as "location_type: LocationType",
                    status as "status: LocationStatus",
                    address, contact_info, business_hours, manager_user_id,
                    capacity, area_square_feet, settings, metadata, is_public,
                    created_at, updated_at, created_by, updated_by
                FROM location_tree
                ORDER BY depth, name
                "#,
                root_id,
                tenant_id
            )
            .fetch_all(self.db.as_ref())
            .await
            .map_err(|e| OlympusError::Database(format!("Failed to get location hierarchy: {}", e)))?
        } else {
            // Get all top-level locations (no parent)
            query_as!(
                LocationRow,
                r#"
                SELECT
                    id, tenant_id, parent_location_id, code, name, description,
                    location_type as "location_type: LocationType",
                    status as "status: LocationStatus",
                    address, contact_info, business_hours, manager_user_id,
                    capacity, area_square_feet, settings, metadata, is_public,
                    created_at, updated_at, created_by, updated_by
                FROM locations
                WHERE tenant_id = $1 AND parent_location_id IS NULL AND deleted_at IS NULL
                ORDER BY name
                "#,
                tenant_id
            )
            .fetch_all(self.db.as_ref())
            .await
            .map_err(|e| OlympusError::Database(format!("Failed to get top-level locations: {}", e)))?
        };

        let mut hierarchies = Vec::new();
        for location_row in locations {
            let location = self.location_row_to_model(location_row)?;
            let hierarchy = LocationHierarchy {
                location,
                children: Vec::new(), // Simplified - would build full tree
                depth: 0,
                path: vec![location.id],
            };
            hierarchies.push(hierarchy);
        }

        Ok(hierarchies)
    }

    // ============================================================================
    // LOCATION ASSIGNMENT OPERATIONS
    // ============================================================================

    pub async fn assign_user_to_location(
        &self,
        tenant_id: Uuid,
        request: CreateLocationAssignmentRequest,
        assigned_by: Uuid,
    ) -> Result<LocationAssignment> {
        let assignment_id = Uuid::new_v4();
        let now = Utc::now();

        // Validate location exists and belongs to tenant
        self.get_location(tenant_id, request.location_id).await?
            .ok_or_else(|| OlympusError::NotFound("Location not found".to_string()))?;

        // Create assignment
        let assignment = query_as!(
            LocationAssignmentRow,
            r#"
            INSERT INTO location_assignments (
                id, user_id, location_id, tenant_id, role, is_primary,
                access_level, assigned_at, assigned_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING
                id, user_id, location_id, tenant_id, role, is_primary,
                access_level, assigned_at, assigned_by
            "#,
            assignment_id,
            request.user_id,
            request.location_id,
            tenant_id,
            request.role,
            request.is_primary,
            request.access_level,
            now,
            assigned_by
        )
        .fetch_one(self.db.as_ref())
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to create location assignment: {}", e)))?;

        let assignment = LocationAssignment {
            id: assignment.id,
            user_id: assignment.user_id,
            location_id: assignment.location_id,
            tenant_id: assignment.tenant_id,
            role: assignment.role,
            is_primary: assignment.is_primary,
            access_level: assignment.access_level,
            assigned_at: assignment.assigned_at,
            assigned_by: assignment.assigned_by,
        };

        // Publish domain event
        let event = DomainEvent::builder()
            .data(serde_json::json!({
                "assignment_id": assignment_id,
                "user_id": request.user_id,
                "location_id": request.location_id,
                "tenant_id": tenant_id,
                "role": request.role,
                "assigned_by": assigned_by
            }))
            .source_service("platform")
            .build();

        if let Err(e) = self.event_publisher.publish(&event).await {
            tracing::warn!("Failed to publish UserAssignedToLocation event: {}", e);
        }

        Ok(assignment)
    }

    // ============================================================================
    // SEARCH AND ANALYTICS
    // ============================================================================

    pub async fn search_locations(
        &self,
        tenant_id: Uuid,
        request: LocationSearchRequest,
    ) -> Result<LocationSearchResponse> {
        let limit = request.limit.unwrap_or(50).min(100);
        let offset = request.offset.unwrap_or(0);

        // Build search query based on filters
        let mut where_conditions = vec!["tenant_id = $1".to_string(), "deleted_at IS NULL".to_string()];
        let mut param_count = 2;

        if let Some(query) = &request.query {
            where_conditions.push(format!("(name ILIKE ${} OR code ILIKE ${})", param_count, param_count));
            param_count += 1;
        }

        if request.location_type.is_some() {
            where_conditions.push(format!("location_type = ${}", param_count));
            param_count += 1;
        }

        if request.status.is_some() {
            where_conditions.push(format!("status = ${}", param_count));
            param_count += 1;
        }

        // Simplified query - a full implementation would handle all filters
        let locations = query_as!(
            LocationRow,
            r#"
            SELECT
                id, tenant_id, parent_location_id, code, name, description,
                location_type as "location_type: LocationType",
                status as "status: LocationStatus",
                address, contact_info, business_hours, manager_user_id,
                capacity, area_square_feet, settings, metadata, is_public,
                created_at, updated_at, created_by, updated_by
            FROM locations
            WHERE tenant_id = $1 AND deleted_at IS NULL
            ORDER BY name
            LIMIT $2 OFFSET $3
            "#,
            tenant_id,
            limit as i64,
            offset as i64
        )
        .fetch_all(self.db.as_ref())
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to search locations: {}", e)))?;

        let total_count = query!(
            "SELECT COUNT(*) as count FROM locations WHERE tenant_id = $1 AND deleted_at IS NULL",
            tenant_id
        )
        .fetch_one(self.db.as_ref())
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to count locations: {}", e)))?
        .count
        .unwrap_or(0);

        let locations = locations
            .into_iter()
            .map(|row| self.location_row_to_model(row))
            .collect::<Result<Vec<_>>>()?;

        Ok(LocationSearchResponse {
            locations,
            total_count,
            has_more: (offset as i64 + limit as i64) < total_count,
        })
    }

    pub async fn get_location_statistics(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> Result<Option<LocationStatistics>> {
        // Simplified implementation - would typically join with user assignments,
        // orders, and other related entities
        let stats = query!(
            r#"
            SELECT
                l.id as location_id,
                COALESCE(la_stats.total_users, 0) as total_users,
                COALESCE(la_stats.active_users, 0) as active_users,
                0::bigint as total_orders,
                0::decimal as total_revenue,
                l.capacity,
                NULL::timestamptz as last_activity
            FROM locations l
            LEFT JOIN (
                SELECT
                    location_id,
                    COUNT(*) as total_users,
                    COUNT(*) as active_users  -- Simplified
                FROM location_assignments
                WHERE location_id = $2
                GROUP BY location_id
            ) la_stats ON l.id = la_stats.location_id
            WHERE l.id = $2 AND l.tenant_id = $1 AND l.deleted_at IS NULL
            "#,
            tenant_id,
            location_id
        )
        .fetch_optional(self.db.as_ref())
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to get location statistics: {}", e)))?;

        if let Some(stats) = stats {
            let capacity_utilization = if let Some(capacity) = stats.capacity {
                if capacity > 0 {
                    Some(stats.active_users as f64 / capacity as f64 * 100.0)
                } else {
                    None
                }
            } else {
                None
            };

            Ok(Some(LocationStatistics {
                location_id: stats.location_id,
                total_users: stats.total_users,
                active_users: stats.active_users,
                total_orders: stats.total_orders,
                total_revenue: Decimal::from_i64(stats.total_revenue.to_string().parse().unwrap_or(0)).unwrap(),
                capacity_utilization,
                last_activity: stats.last_activity,
            }))
        } else {
            Ok(None)
        }
    }

    // ============================================================================
    // HELPER METHODS
    // ============================================================================

    async fn validate_parent_location(&self, tenant_id: Uuid, parent_id: Uuid) -> Result<()> {
        let exists = query!(
            "SELECT id FROM locations WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL",
            parent_id,
            tenant_id
        )
        .fetch_optional(self.db.as_ref())
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to validate parent location: {}", e)))?;

        if exists.is_none() {
            return Err(OlympusError::Validation("Parent location not found".to_string()));
        }

        Ok(())
    }

    async fn validate_location_code_unique(
        &self,
        tenant_id: Uuid,
        code: &str,
        exclude_id: Option<Uuid>,
    ) -> Result<()> {
        let mut query_str = "SELECT id FROM locations WHERE code = $1 AND tenant_id = $2 AND deleted_at IS NULL".to_string();

        if exclude_id.is_some() {
            query_str.push_str(" AND id != $3");
        }

        let exists = if let Some(exclude_id) = exclude_id {
            query!(&query_str, code, tenant_id, exclude_id)
                .fetch_optional(self.db.as_ref())
                .await
        } else {
            query!(&query_str, code, tenant_id)
                .fetch_optional(self.db.as_ref())
                .await
        }
        .map_err(|e| OlympusError::Database(format!("Failed to check location code uniqueness: {}", e)))?;

        if exists.is_some() {
            return Err(OlympusError::Validation("Location code already exists".to_string()));
        }

        Ok(())
    }

    fn location_row_to_model(&self, row: LocationRow) -> Result<Location> {
        let address: Address = serde_json::from_value(row.address)
            .map_err(|e| OlympusError::Database(format!("Invalid address JSON: {}", e)))?;

        let contact_info: ContactInfo = serde_json::from_value(row.contact_info)
            .map_err(|e| OlympusError::Database(format!("Invalid contact info JSON: {}", e)))?;

        let business_hours: Option<BusinessHours> = serde_json::from_value(row.business_hours)
            .map_err(|e| OlympusError::Database(format!("Invalid business hours JSON: {}", e)))?;

        Ok(Location {
            id: row.id,
            tenant_id: row.tenant_id,
            parent_location_id: row.parent_location_id,
            code: row.code,
            name: row.name,
            description: row.description,
            location_type: row.location_type,
            status: row.status,
            address,
            contact_info,
            business_hours,
            manager_user_id: row.manager_user_id,
            capacity: row.capacity,
            area_square_feet: row.area_square_feet,
            settings: row.settings,
            metadata: row.metadata,
            is_public: row.is_public,
            created_at: row.created_at,
            updated_at: row.updated_at,
            created_by: row.created_by,
            updated_by: row.updated_by,
        })
    }
}

// ============================================================================
// DATABASE ROW TYPES
// ============================================================================

#[derive(Debug)]
struct LocationRow {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub parent_location_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub location_type: LocationType,
    pub status: LocationStatus,
    pub address: serde_json::Value,
    pub contact_info: serde_json::Value,
    pub business_hours: serde_json::Value,
    pub manager_user_id: Option<Uuid>,
    pub capacity: Option<i32>,
    pub area_square_feet: Option<f64>,
    pub settings: serde_json::Value,
    pub metadata: serde_json::Value,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

#[derive(Debug)]
struct LocationAssignmentRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub location_id: Uuid,
    pub tenant_id: Uuid,
    pub role: String,
    pub is_primary: bool,
    pub access_level: String,
    pub assigned_at: DateTime<Utc>,
    pub assigned_by: Uuid,
}
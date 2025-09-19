// ============================================================================
// OLYMPUS CLOUD - ROLE-BASED ACCESS CONTROL (RBAC)
// ============================================================================
// Module: platform/src/services/rbac.rs
// Description: Comprehensive RBAC system with role hierarchy and dynamic permissions
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use chrono::{DateTime, Utc};
use olympus_shared::database::DbPool;
use olympus_shared::events::{DomainEvent, EventPublisher};
use olympus_shared::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tracing::{error, info, warn};
use uuid::Uuid;
use validator::Validate;

/// Resource types in the system
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    Tenant,
    User,
    Location,
    Product,
    Order,
    Payment,
    Report,
    Settings,
    ApiKey,
    Webhook,
    Inventory,
    Customer,
}

impl ResourceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Tenant => "tenant",
            Self::User => "user",
            Self::Location => "location",
            Self::Product => "product",
            Self::Order => "order",
            Self::Payment => "payment",
            Self::Report => "report",
            Self::Settings => "settings",
            Self::ApiKey => "api_key",
            Self::Webhook => "webhook",
            Self::Inventory => "inventory",
            Self::Customer => "customer",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "tenant" => Some(Self::Tenant),
            "user" => Some(Self::User),
            "location" => Some(Self::Location),
            "product" => Some(Self::Product),
            "order" => Some(Self::Order),
            "payment" => Some(Self::Payment),
            "report" => Some(Self::Report),
            "settings" => Some(Self::Settings),
            "api_key" => Some(Self::ApiKey),
            "webhook" => Some(Self::Webhook),
            "inventory" => Some(Self::Inventory),
            "customer" => Some(Self::Customer),
            _ => None,
        }
    }
}

/// Actions that can be performed on resources
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    Create,
    Read,
    Update,
    Delete,
    List,
    Approve,
    Reject,
    Export,
    Import,
    Assign,
    Unassign,
    Activate,
    Deactivate,
    Configure,
    Monitor,
}

impl Action {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Create => "create",
            Self::Read => "read",
            Self::Update => "update",
            Self::Delete => "delete",
            Self::List => "list",
            Self::Approve => "approve",
            Self::Reject => "reject",
            Self::Export => "export",
            Self::Import => "import",
            Self::Assign => "assign",
            Self::Unassign => "unassign",
            Self::Activate => "activate",
            Self::Deactivate => "deactivate",
            Self::Configure => "configure",
            Self::Monitor => "monitor",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "create" => Some(Self::Create),
            "read" => Some(Self::Read),
            "update" => Some(Self::Update),
            "delete" => Some(Self::Delete),
            "list" => Some(Self::List),
            "approve" => Some(Self::Approve),
            "reject" => Some(Self::Reject),
            "export" => Some(Self::Export),
            "import" => Some(Self::Import),
            "assign" => Some(Self::Assign),
            "unassign" => Some(Self::Unassign),
            "activate" => Some(Self::Activate),
            "deactivate" => Some(Self::Deactivate),
            "configure" => Some(Self::Configure),
            "monitor" => Some(Self::Monitor),
            _ => None,
        }
    }
}

/// Permission represents the ability to perform an action on a resource
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Permission {
    pub resource_type: ResourceType,
    pub action: Action,
    pub scope: PermissionScope,
}

impl Permission {
    pub fn new(resource_type: ResourceType, action: Action, scope: PermissionScope) -> Self {
        Self {
            resource_type,
            action,
            scope,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}:{}",
                self.resource_type.as_str(),
                self.action.as_str(),
                self.scope.as_str())
    }

    pub fn from_string(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 {
            return None;
        }

        let resource_type = ResourceType::from_str(parts[0])?;
        let action = Action::from_str(parts[1])?;
        let scope = PermissionScope::from_str(parts[2])?;

        Some(Self {
            resource_type,
            action,
            scope,
        })
    }
}

/// Scope of permission (what the permission applies to)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PermissionScope {
    /// Can perform action on any resource in any tenant
    Global,
    /// Can perform action on any resource within tenant
    Tenant,
    /// Can perform action on resources within specific location
    Location(Uuid),
    /// Can perform action only on own resources
    Own,
    /// Can perform action on specific resource instance
    Resource(Uuid),
}

impl PermissionScope {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Global => "global",
            Self::Tenant => "tenant",
            Self::Location(_) => "location",
            Self::Own => "own",
            Self::Resource(_) => "resource",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "global" => Some(Self::Global),
            "tenant" => Some(Self::Tenant),
            "own" => Some(Self::Own),
            s if s.starts_with("location:") => {
                let id_str = s.strip_prefix("location:")?;
                let id = Uuid::parse_str(id_str).ok()?;
                Some(Self::Location(id))
            }
            s if s.starts_with("resource:") => {
                let id_str = s.strip_prefix("resource:")?;
                let id = Uuid::parse_str(id_str).ok()?;
                Some(Self::Resource(id))
            }
            _ => None,
        }
    }

    pub fn includes(&self, other: &PermissionScope) -> bool {
        match (self, other) {
            (Self::Global, _) => true,
            (Self::Tenant, Self::Location(_)) => true,
            (Self::Tenant, Self::Own) => true,
            (Self::Tenant, Self::Resource(_)) => true,
            (Self::Location(a), Self::Location(b)) => a == b,
            (Self::Location(_), Self::Own) => true,
            (Self::Location(_), Self::Resource(_)) => true, // Would need location context
            (Self::Resource(a), Self::Resource(b)) => a == b,
            _ => false,
        }
    }
}

/// Role represents a collection of permissions
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id: Uuid,
    pub tenant_id: Option<Uuid>, // None for global roles
    pub name: String,
    pub description: Option<String>,
    pub is_system: bool, // Cannot be deleted or modified
    pub is_active: bool,
    pub parent_role_id: Option<Uuid>, // For role hierarchy
    pub permissions: serde_json::Value, // JSON array of permission strings
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Role {
    pub fn new(name: String, tenant_id: Option<Uuid>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id,
            name,
            description: None,
            is_system: false,
            is_active: true,
            parent_role_id: None,
            permissions: serde_json::json!([]),
            metadata: serde_json::json!({}),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn get_permissions(&self) -> Vec<Permission> {
        if let Ok(permission_strings) = serde_json::from_value::<Vec<String>>(self.permissions.clone()) {
            permission_strings
                .iter()
                .filter_map(|s| Permission::from_string(s))
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn set_permissions(&mut self, permissions: Vec<Permission>) {
        let permission_strings: Vec<String> = permissions
            .into_iter()
            .map(|p| p.to_string())
            .collect();
        self.permissions = serde_json::json!(permission_strings);
        self.updated_at = Utc::now();
    }

    pub fn add_permission(&mut self, permission: Permission) {
        let mut permissions = self.get_permissions();
        if !permissions.contains(&permission) {
            permissions.push(permission);
            self.set_permissions(permissions);
        }
    }

    pub fn remove_permission(&mut self, permission: &Permission) {
        let mut permissions = self.get_permissions();
        permissions.retain(|p| p != permission);
        self.set_permissions(permissions);
    }

    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.get_permissions().contains(permission)
    }
}

/// User role assignment
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserRole {
    pub id: Uuid,
    pub user_id: Uuid,
    pub role_id: Uuid,
    pub tenant_id: Uuid,
    pub location_id: Option<Uuid>, // Role is scoped to specific location
    pub assigned_by: Uuid,
    pub assigned_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
}

impl UserRole {
    pub fn new(user_id: Uuid, role_id: Uuid, tenant_id: Uuid, assigned_by: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            role_id,
            tenant_id,
            location_id: None,
            assigned_by,
            assigned_at: Utc::now(),
            expires_at: None,
            is_active: true,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at
            .map(|expires| expires <= Utc::now())
            .unwrap_or(false)
    }

    pub fn is_valid(&self) -> bool {
        self.is_active && !self.is_expired()
    }
}

/// RBAC service for managing roles and permissions
#[derive(Debug)]
pub struct RbacService {
    db: Arc<DbPool>,
    event_publisher: Arc<EventPublisher>,
    // Cache for role hierarchies and permissions
    role_cache: std::sync::RwLock<HashMap<Uuid, Role>>,
    permission_cache: std::sync::RwLock<HashMap<Uuid, HashSet<Permission>>>,
}

impl RbacService {
    pub fn new(db: Arc<DbPool>, event_publisher: Arc<EventPublisher>) -> Self {
        Self {
            db,
            event_publisher,
            role_cache: std::sync::RwLock::new(HashMap::new()),
            permission_cache: std::sync::RwLock::new(HashMap::new()),
        }
    }

    /// Check if user has permission to perform action on resource
    pub async fn check_permission(
        &self,
        user_id: Uuid,
        tenant_id: Uuid,
        resource_type: ResourceType,
        action: Action,
        resource_id: Option<Uuid>,
        location_id: Option<Uuid>,
    ) -> Result<bool> {
        // Get user's effective permissions
        let permissions = self.get_user_permissions(user_id, tenant_id).await?;

        // Check if any permission allows this action
        for permission in permissions {
            if permission.resource_type == resource_type && permission.action == action {
                match &permission.scope {
                    PermissionScope::Global => return Ok(true),
                    PermissionScope::Tenant => return Ok(true),
                    PermissionScope::Location(loc_id) => {
                        if let Some(req_loc_id) = location_id {
                            if loc_id == &req_loc_id {
                                return Ok(true);
                            }
                        }
                    }
                    PermissionScope::Own => {
                        // Check if resource belongs to user (implementation dependent)
                        if let Some(res_id) = resource_id {
                            if self.resource_belongs_to_user(user_id, resource_type.clone(), res_id).await? {
                                return Ok(true);
                            }
                        }
                    }
                    PermissionScope::Resource(res_id) => {
                        if let Some(req_res_id) = resource_id {
                            if res_id == &req_res_id {
                                return Ok(true);
                            }
                        }
                    }
                }
            }
        }

        Ok(false)
    }

    /// Get all effective permissions for a user (including inherited from role hierarchy)
    pub async fn get_user_permissions(&self, user_id: Uuid, tenant_id: Uuid) -> Result<HashSet<Permission>> {
        // Check cache first
        if let Ok(cache) = self.permission_cache.read() {
            if let Some(permissions) = cache.get(&user_id) {
                return Ok(permissions.clone());
            }
        }

        // Get user roles
        let user_roles = self.get_user_roles(user_id, tenant_id).await?;
        let mut all_permissions = HashSet::new();

        // Collect permissions from all roles (including inherited)
        for user_role in user_roles {
            if user_role.is_valid() {
                let role_permissions = self.get_role_permissions(user_role.role_id).await?;
                all_permissions.extend(role_permissions);
            }
        }

        // Update cache
        if let Ok(mut cache) = self.permission_cache.write() {
            cache.insert(user_id, all_permissions.clone());
        }

        Ok(all_permissions)
    }

    /// Get permissions for a role including inherited permissions
    pub async fn get_role_permissions(&self, role_id: Uuid) -> Result<HashSet<Permission>> {
        let mut all_permissions = HashSet::new();
        let mut visited_roles = HashSet::new();

        self.collect_role_permissions(role_id, &mut all_permissions, &mut visited_roles).await?;

        Ok(all_permissions)
    }

    /// Recursively collect permissions from role hierarchy
    async fn collect_role_permissions(
        &self,
        role_id: Uuid,
        permissions: &mut HashSet<Permission>,
        visited: &mut HashSet<Uuid>,
    ) -> Result<()> {
        if visited.contains(&role_id) {
            return Ok(()); // Prevent infinite loops
        }
        visited.insert(role_id);

        // Get role from cache or database
        let role = self.get_role(role_id).await?;

        // Add role's own permissions
        permissions.extend(role.get_permissions());

        // Add permissions from parent role
        if let Some(parent_id) = role.parent_role_id {
            self.collect_role_permissions(parent_id, permissions, visited).await?;
        }

        Ok(())
    }

    /// Get role by ID
    pub async fn get_role(&self, role_id: Uuid) -> Result<Role> {
        // Check cache first
        if let Ok(cache) = self.role_cache.read() {
            if let Some(role) = cache.get(&role_id) {
                return Ok(role.clone());
            }
        }

        // Query database (placeholder - would use actual SQL)
        // For now, return a default role structure
        let role = Role::new("Default Role".to_string(), None);

        // Update cache
        if let Ok(mut cache) = self.role_cache.write() {
            cache.insert(role_id, role.clone());
        }

        Ok(role)
    }

    /// Get user roles in tenant
    pub async fn get_user_roles(&self, user_id: Uuid, tenant_id: Uuid) -> Result<Vec<UserRole>> {
        // Placeholder implementation - would query database
        Ok(vec![UserRole::new(user_id, Uuid::new_v4(), tenant_id, Uuid::new_v4())])
    }

    /// Assign role to user
    pub async fn assign_role(
        &self,
        user_id: Uuid,
        role_id: Uuid,
        tenant_id: Uuid,
        assigned_by: Uuid,
        location_id: Option<Uuid>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<UserRole> {
        let mut user_role = UserRole::new(user_id, role_id, tenant_id, assigned_by);
        user_role.location_id = location_id;
        user_role.expires_at = expires_at;

        // Insert into database (placeholder)
        info!("Assigning role {} to user {} in tenant {}", role_id, user_id, tenant_id);

        // Clear permission cache for user
        if let Ok(mut cache) = self.permission_cache.write() {
            cache.remove(&user_id);
        }

        // Publish domain event
        let event = DomainEvent::builder(
            "RoleAssigned".to_string(),
            user_id,
            "User".to_string(),
            tenant_id,
        )
        .data(serde_json::json!({
            "user_id": user_id,
            "role_id": role_id,
            "tenant_id": tenant_id,
            "assigned_by": assigned_by,
            "location_id": location_id,
            "expires_at": expires_at
        }))
        .unwrap()
        .source_service("platform".to_string())
        .build();

        if let Err(e) = self.event_publisher.publish(&event).await {
            warn!("Failed to publish RoleAssigned event: {}", e);
        }

        Ok(user_role)
    }

    /// Revoke role from user
    pub async fn revoke_role(
        &self,
        user_id: Uuid,
        role_id: Uuid,
        tenant_id: Uuid,
        revoked_by: Uuid,
    ) -> Result<()> {
        // Update database to mark role as inactive (placeholder)
        info!("Revoking role {} from user {} in tenant {}", role_id, user_id, tenant_id);

        // Clear permission cache for user
        if let Ok(mut cache) = self.permission_cache.write() {
            cache.remove(&user_id);
        }

        // Publish domain event
        let event = DomainEvent::builder(
            "RoleRevoked".to_string(),
            user_id,
            "User".to_string(),
            tenant_id,
        )
        .data(serde_json::json!({
            "user_id": user_id,
            "role_id": role_id,
            "tenant_id": tenant_id,
            "revoked_by": revoked_by
        }))
        .unwrap()
        .source_service("platform".to_string())
        .build();

        if let Err(e) = self.event_publisher.publish(&event).await {
            warn!("Failed to publish RoleRevoked event: {}", e);
        }

        Ok(())
    }

    /// Create a new role
    pub async fn create_role(
        &self,
        name: String,
        description: Option<String>,
        tenant_id: Option<Uuid>,
        permissions: Vec<Permission>,
        parent_role_id: Option<Uuid>,
    ) -> Result<Role> {
        let mut role = Role::new(name, tenant_id);
        role.description = description;
        role.parent_role_id = parent_role_id;
        role.set_permissions(permissions);

        // Insert into database (placeholder)
        info!("Creating role: {}", role.name);

        // Update cache
        if let Ok(mut cache) = self.role_cache.write() {
            cache.insert(role.id, role.clone());
        }

        // Publish domain event
        let event = DomainEvent::builder(
            "RoleCreated".to_string(),
            role.id,
            "Role".to_string(),
            tenant_id.unwrap_or_default(),
        )
        .data(serde_json::json!({
            "role_id": role.id,
            "name": role.name,
            "tenant_id": tenant_id,
            "parent_role_id": parent_role_id
        }))
        .unwrap()
        .source_service("platform".to_string())
        .build();

        if let Err(e) = self.event_publisher.publish(&event).await {
            warn!("Failed to publish RoleCreated event: {}", e);
        }

        Ok(role)
    }

    /// Check if resource belongs to user (for "Own" scope permissions)
    async fn resource_belongs_to_user(
        &self,
        user_id: Uuid,
        resource_type: ResourceType,
        resource_id: Uuid,
    ) -> Result<bool> {
        // Implementation would depend on resource type
        // For now, return false as placeholder
        match resource_type {
            ResourceType::User => Ok(user_id == resource_id),
            _ => Ok(false), // Would need actual ownership checking
        }
    }

    /// Clear all caches
    pub fn clear_cache(&self) {
        if let Ok(mut role_cache) = self.role_cache.write() {
            role_cache.clear();
        }
        if let Ok(mut permission_cache) = self.permission_cache.write() {
            permission_cache.clear();
        }
    }

    /// Initialize default system roles
    pub async fn initialize_system_roles(&self) -> Result<()> {
        info!("Initializing system roles");

        // Super Admin role (global scope)
        let super_admin_permissions = vec![
            Permission::new(ResourceType::Tenant, Action::Create, PermissionScope::Global),
            Permission::new(ResourceType::Tenant, Action::Read, PermissionScope::Global),
            Permission::new(ResourceType::Tenant, Action::Update, PermissionScope::Global),
            Permission::new(ResourceType::Tenant, Action::Delete, PermissionScope::Global),
            Permission::new(ResourceType::User, Action::Create, PermissionScope::Global),
            Permission::new(ResourceType::User, Action::Read, PermissionScope::Global),
            Permission::new(ResourceType::User, Action::Update, PermissionScope::Global),
            Permission::new(ResourceType::User, Action::Delete, PermissionScope::Global),
            Permission::new(ResourceType::Settings, Action::Configure, PermissionScope::Global),
        ];

        self.create_role(
            "Super Admin".to_string(),
            Some("Global system administrator with all permissions".to_string()),
            None, // Global role
            super_admin_permissions,
            None,
        ).await?;

        // Tenant Admin role (tenant scope)
        let tenant_admin_permissions = vec![
            Permission::new(ResourceType::User, Action::Create, PermissionScope::Tenant),
            Permission::new(ResourceType::User, Action::Read, PermissionScope::Tenant),
            Permission::new(ResourceType::User, Action::Update, PermissionScope::Tenant),
            Permission::new(ResourceType::User, Action::Delete, PermissionScope::Tenant),
            Permission::new(ResourceType::Location, Action::Create, PermissionScope::Tenant),
            Permission::new(ResourceType::Location, Action::Read, PermissionScope::Tenant),
            Permission::new(ResourceType::Location, Action::Update, PermissionScope::Tenant),
            Permission::new(ResourceType::Settings, Action::Configure, PermissionScope::Tenant),
        ];

        self.create_role(
            "Tenant Admin".to_string(),
            Some("Tenant administrator with full tenant permissions".to_string()),
            None, // Available to all tenants
            tenant_admin_permissions,
            None,
        ).await?;

        // Manager role
        let manager_permissions = vec![
            Permission::new(ResourceType::User, Action::Read, PermissionScope::Tenant),
            Permission::new(ResourceType::Product, Action::Create, PermissionScope::Tenant),
            Permission::new(ResourceType::Product, Action::Read, PermissionScope::Tenant),
            Permission::new(ResourceType::Product, Action::Update, PermissionScope::Tenant),
            Permission::new(ResourceType::Order, Action::Read, PermissionScope::Tenant),
            Permission::new(ResourceType::Order, Action::Update, PermissionScope::Tenant),
            Permission::new(ResourceType::Report, Action::Read, PermissionScope::Tenant),
        ];

        self.create_role(
            "Manager".to_string(),
            Some("Location manager with operational permissions".to_string()),
            None,
            manager_permissions,
            None,
        ).await?;

        // Employee role
        let employee_permissions = vec![
            Permission::new(ResourceType::Product, Action::Read, PermissionScope::Tenant),
            Permission::new(ResourceType::Order, Action::Create, PermissionScope::Tenant),
            Permission::new(ResourceType::Order, Action::Read, PermissionScope::Tenant),
            Permission::new(ResourceType::Order, Action::Update, PermissionScope::Own),
            Permission::new(ResourceType::Customer, Action::Create, PermissionScope::Tenant),
            Permission::new(ResourceType::Customer, Action::Read, PermissionScope::Tenant),
        ];

        self.create_role(
            "Employee".to_string(),
            Some("Standard employee with basic operational permissions".to_string()),
            None,
            employee_permissions,
            None,
        ).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_string_conversion() {
        let permission = Permission::new(
            ResourceType::User,
            Action::Create,
            PermissionScope::Tenant,
        );

        let permission_string = permission.to_string();
        assert_eq!(permission_string, "user:create:tenant");

        let parsed_permission = Permission::from_string(&permission_string).unwrap();
        assert_eq!(parsed_permission, permission);
    }

    #[test]
    fn test_permission_scope_includes() {
        let global = PermissionScope::Global;
        let tenant = PermissionScope::Tenant;
        let location = PermissionScope::Location(Uuid::new_v4());
        let own = PermissionScope::Own;

        assert!(global.includes(&tenant));
        assert!(global.includes(&location));
        assert!(global.includes(&own));

        assert!(tenant.includes(&location));
        assert!(tenant.includes(&own));
        assert!(!tenant.includes(&global));

        assert!(!own.includes(&tenant));
        assert!(!own.includes(&location));
    }

    #[test]
    fn test_role_permissions() {
        let mut role = Role::new("Test Role".to_string(), Some(Uuid::new_v4()));

        let permission = Permission::new(
            ResourceType::User,
            Action::Read,
            PermissionScope::Tenant,
        );

        role.add_permission(permission.clone());
        assert!(role.has_permission(&permission));

        role.remove_permission(&permission);
        assert!(!role.has_permission(&permission));
    }

    #[test]
    fn test_user_role_expiry() {
        let mut user_role = UserRole::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
        );

        assert!(user_role.is_valid());

        // Set expiry in the past
        user_role.expires_at = Some(Utc::now() - chrono::Duration::days(1));
        assert!(!user_role.is_valid());

        // Set expiry in the future
        user_role.expires_at = Some(Utc::now() + chrono::Duration::days(1));
        assert!(user_role.is_valid());
    }
}
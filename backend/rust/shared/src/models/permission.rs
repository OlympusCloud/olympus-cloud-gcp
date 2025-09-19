// ============================================================================
// OLYMPUS CLOUD - PERMISSION AND RBAC MODELS
// ============================================================================
// Module: shared/src/models/permission.rs
// Description: Role-based access control models
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use super::{TenantScoped, ValidateEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::ValidationError;

/// Role entity for RBAC
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub permissions: Vec<String>,
    pub is_system: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Role {
    /// Create a new role
    pub fn new(tenant_id: Uuid, name: String, permissions: Vec<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id,
            name,
            display_name: None,
            description: None,
            permissions,
            is_system: false,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if role has permission
    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(&permission.to_string())
    }

    /// Add permission
    pub fn add_permission(&mut self, permission: String) {
        if !self.permissions.contains(&permission) {
            self.permissions.push(permission);
            self.updated_at = Utc::now();
        }
    }

    /// Remove permission
    pub fn remove_permission(&mut self, permission: &str) {
        if let Some(index) = self.permissions.iter().position(|p| p == permission) {
            self.permissions.remove(index);
            self.updated_at = Utc::now();
        }
    }
}

impl TenantScoped for Role {
    fn tenant_id(&self) -> Uuid {
        self.tenant_id
    }
}

/// User role assignment
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserRole {
    pub user_id: Uuid,
    pub role_id: Uuid,
    pub assigned_at: DateTime<Utc>,
    pub assigned_by: Option<Uuid>,
}

/// Permission entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub name: String,
    pub description: String,
    pub resource: String,
    pub action: String,
}

/// User permissions cache
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserPermission {
    pub user_id: Uuid,
    pub permission: String,
    pub granted_at: DateTime<Utc>,
    pub granted_by: Option<Uuid>,
}

/// Common permission constants
pub mod permissions {
    // User management
    pub const READ_USERS: &str = "read:users";
    pub const WRITE_USERS: &str = "write:users";
    pub const DELETE_USERS: &str = "delete:users";

    // Product management
    pub const READ_PRODUCTS: &str = "read:products";
    pub const WRITE_PRODUCTS: &str = "write:products";
    pub const DELETE_PRODUCTS: &str = "delete:products";

    // Order management
    pub const READ_ORDERS: &str = "read:orders";
    pub const WRITE_ORDERS: &str = "write:orders";
    pub const DELETE_ORDERS: &str = "delete:orders";

    // Analytics
    pub const READ_ANALYTICS: &str = "read:analytics";
    pub const WRITE_ANALYTICS: &str = "write:analytics";

    // Admin permissions
    pub const ADMIN_USERS: &str = "admin:users";
    pub const ADMIN_SETTINGS: &str = "admin:settings";
    pub const ADMIN_BILLING: &str = "admin:billing";
}

impl ValidateEntity for Role {
    type Error = ValidationError;

    fn validate(&self) -> Result<(), Self::Error> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::new("empty_name"));
        }
        Ok(())
    }
}
// ============================================================================
// OLYMPUS CLOUD - SHARED DATABASE MODELS
// ============================================================================
// Module: shared/src/models/mod.rs
// Description: Shared database models for all services
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

pub mod user;
pub mod tenant;
pub mod session;
pub mod permission;
pub mod product;
pub mod order;
pub mod payment;
pub mod event;
pub mod customer;
pub mod location;
pub mod inventory;

// Re-export commonly used models
pub use user::{User, UserRole, UserStatus, CreateUserRequest, UpdateUserRequest};
pub use tenant::{Tenant, SubscriptionTier, SubscriptionStatus, IndustryType};
pub use session::{UserSession, SessionStatus, TokenType};
pub use permission::{Role, Permission, UserPermission};
pub use product::{Product, ProductVariant, Category};
pub use order::{Order, OrderItem, OrderStatus, PaymentStatus, FulfillmentStatus};
pub use payment::{Payment, PaymentMethod, PaymentType};
pub use event::{DomainEvent, EventStatus, AggregateSnapshot};
pub use customer::{Customer, CustomerAddress};
pub use location::{Location, BusinessHours};
pub use inventory::{Inventory, InventoryAdjustment, AdjustmentType};

// Common traits and types
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

/// Base audit fields that most entities have
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuditFields {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Pagination parameters for list queries
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PaginationParams {
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub cursor: Option<String>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            limit: Some(20),
            offset: Some(0),
            cursor: None,
        }
    }
}

/// Pagination response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: Option<i64>,
    pub has_next: bool,
    pub next_cursor: Option<String>,
}

/// Filter parameters for queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterParams {
    pub tenant_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub is_active: Option<bool>,
    pub search: Option<String>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
}

/// Common database result type
pub type DbResult<T> = Result<T, sqlx::Error>;

/// Soft delete trait for entities
pub trait SoftDelete {
    fn is_deleted(&self) -> bool;
    fn delete(&mut self);
    fn restore(&mut self);
}

/// Tenant-scoped entity trait
pub trait TenantScoped {
    fn tenant_id(&self) -> Uuid;
    fn validate_tenant(&self, tenant_id: Uuid) -> bool {
        self.tenant_id() == tenant_id
    }
}

/// Searchable entity trait
pub trait Searchable {
    fn search_fields(&self) -> Vec<String>;
    fn matches_search(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();
        self.search_fields()
            .iter()
            .any(|field| field.to_lowercase().contains(&query_lower))
    }
}

/// Entity validation trait
pub trait ValidateEntity {
    type Error;
    fn validate(&self) -> Result<(), Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_params_default() {
        let params = PaginationParams::default();
        assert_eq!(params.limit, Some(20));
        assert_eq!(params.offset, Some(0));
        assert!(params.cursor.is_none());
    }

    #[test]
    fn test_pagination_validation() {
        let mut params = PaginationParams::default();
        params.limit = Some(150); // Over the max limit

        let validation_result = params.validate();
        assert!(validation_result.is_err());
    }
}
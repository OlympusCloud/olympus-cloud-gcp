// ============================================================================
// OLYMPUS CLOUD - SIMPLIFIED MODELS
// ============================================================================
// Module: shared/src/models/simple_models.rs
// Description: Simplified models for remaining entities (to be expanded later)
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use super::TenantScoped;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Customer entity (simplified)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Customer {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TenantScoped for Customer {
    fn tenant_id(&self) -> Uuid {
        self.tenant_id
    }
}

/// Location entity (simplified)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Location {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub code: Option<String>,
    pub description: Option<String>,
    pub address: serde_json::Value,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TenantScoped for Location {
    fn tenant_id(&self) -> Uuid {
        self.tenant_id
    }
}

/// Inventory entity (simplified)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Inventory {
    pub id: Uuid,
    pub product_id: Uuid,
    pub location_id: Uuid,
    pub quantity_on_hand: i32,
    pub quantity_reserved: i32,
    pub quantity_available: i32,
    pub updated_at: DateTime<Utc>,
}

/// Domain event entity (simplified)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DomainEvent {
    pub id: Uuid,
    pub tenant_id: Option<Uuid>,
    pub aggregate_id: Uuid,
    pub aggregate_type: String,
    pub event_type: String,
    pub event_data: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

impl TenantScoped for DomainEvent {
    fn tenant_id(&self) -> Uuid {
        self.tenant_id.unwrap_or_default()
    }
}
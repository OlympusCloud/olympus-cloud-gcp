use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

// Common Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRequest {
    pub page: i32,
    pub per_page: i32,
    pub sort_by: Option<String>,
    pub sort_order: Option<SortOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiError>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

// Audit Fields
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AuditFields {
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

// Money representation (in cents)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Money {
    pub amount: i64,
    pub currency: Currency,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Currency {
    USD,
    EUR,
    GBP,
    CAD,
    AUD,
}

impl Money {
    pub fn new(amount: i64, currency: Currency) -> Self {
        Self { amount, currency }
    }

    pub fn zero(currency: Currency) -> Self {
        Self { amount: 0, currency }
    }

    pub fn add(&self, other: &Money) -> Result<Money, String> {
        if self.currency != other.currency {
            return Err("Cannot add different currencies".to_string());
        }
        Ok(Money {
            amount: self.amount + other.amount,
            currency: self.currency,
        })
    }

    pub fn subtract(&self, other: &Money) -> Result<Money, String> {
        if self.currency != other.currency {
            return Err("Cannot subtract different currencies".to_string());
        }
        Ok(Money {
            amount: self.amount - other.amount,
            currency: self.currency,
        })
    }

    pub fn to_decimal(&self) -> f64 {
        self.amount as f64 / 100.0
    }
}

// Address
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Address {
    #[validate(length(min = 1, max = 100))]
    pub street1: String,
    pub street2: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub city: String,
    #[validate(length(min = 2, max = 100))]
    pub state_province: String,
    #[validate(length(min = 2, max = 20))]
    pub postal_code: String,
    #[validate(length(equal = 2))]
    pub country_code: String, // ISO 3166-1 alpha-2
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

// Phone Number
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PhoneNumber {
    #[validate(length(min = 1, max = 5))]
    pub country_code: String,
    #[validate(length(min = 1, max = 20))]
    pub number: String,
    pub extension: Option<String>,
}

// Business Hours
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessHours {
    pub day_of_week: DayOfWeek,
    pub open_time: String, // "09:00"
    pub close_time: String, // "17:00"
    pub is_closed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

// File Attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAttachment {
    pub id: Uuid,
    pub filename: String,
    pub content_type: String,
    pub size_bytes: i64,
    pub url: String,
    pub uploaded_at: DateTime<Utc>,
    pub uploaded_by: Uuid,
}

// Permission & Role types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub resource: String,
    pub action: String,
    pub scope: PermissionScope,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionScope {
    Own,      // Can only access own resources
    Tenant,   // Can access all resources in tenant
    Location, // Can access resources in specific location
    Global,   // Can access all resources (super admin)
}

// Helper functions
impl PageRequest {
    pub fn new(page: i32, per_page: i32) -> Self {
        Self {
            page: page.max(1),
            per_page: per_page.max(1).min(100),
            sort_by: None,
            sort_order: None,
        }
    }

    pub fn offset(&self) -> i64 {
        ((self.page - 1) * self.per_page) as i64
    }

    pub fn limit(&self) -> i64 {
        self.per_page as i64
    }
}

impl<T> PageResponse<T> {
    pub fn new(data: Vec<T>, total: i64, page: i32, per_page: i32) -> Self {
        let total_pages = ((total as f64) / (per_page as f64)).ceil() as i32;
        Self {
            data,
            total,
            page,
            per_page,
            total_pages,
        }
    }
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: Utc::now(),
        }
    }

    pub fn error(code: String, message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ApiError {
                code,
                message,
                details: None,
            }),
            timestamp: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_operations() {
        let usd1 = Money::new(1000, Currency::USD);
        let usd2 = Money::new(500, Currency::USD);

        let sum = usd1.add(&usd2).unwrap();
        assert_eq!(sum.amount, 1500);

        let diff = usd1.subtract(&usd2).unwrap();
        assert_eq!(diff.amount, 500);

        assert_eq!(usd1.to_decimal(), 10.0);
    }

    #[test]
    fn test_page_request() {
        let page_req = PageRequest::new(2, 20);
        assert_eq!(page_req.offset(), 20);
        assert_eq!(page_req.limit(), 20);
    }
}
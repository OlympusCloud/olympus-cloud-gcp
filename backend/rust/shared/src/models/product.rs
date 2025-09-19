// ============================================================================
// OLYMPUS CLOUD - PRODUCT MODELS
// ============================================================================
// Module: shared/src/models/product.rs
// Description: Product catalog and inventory models
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use super::{AuditFields, SoftDelete, TenantScoped, ValidateEntity, Searchable};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;
use validator::{Validate, ValidationError};
use rust_decimal::Decimal;

/// Product entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<Uuid>,
    pub brand: Option<String>,
    pub unit_price: Decimal,
    pub compare_at_price: Option<Decimal>,
    pub cost: Option<Decimal>,
    pub tax_rate: Option<Decimal>,
    pub weight_value: Option<Decimal>,
    pub weight_unit: Option<String>,
    pub dimensions: serde_json::Value,
    pub is_digital: bool,
    pub is_active: bool,
    pub requires_shipping: bool,
    pub track_inventory: bool,
    pub allow_backorder: bool,
    pub images: Vec<String>,
    pub attributes: serde_json::Value,
    pub metadata: serde_json::Value,
    pub tags: Vec<String>,
    #[sqlx(flatten)]
    pub audit_fields: AuditFields,
}

impl Product {
    /// Create a new product
    pub fn new(tenant_id: Uuid, sku: String, name: String, unit_price: Decimal) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id,
            sku,
            name,
            description: None,
            category_id: None,
            brand: None,
            unit_price,
            compare_at_price: None,
            cost: None,
            tax_rate: None,
            weight_value: None,
            weight_unit: None,
            dimensions: serde_json::json!({}),
            is_digital: false,
            is_active: true,
            requires_shipping: true,
            track_inventory: true,
            allow_backorder: false,
            images: vec![],
            attributes: serde_json::json!({}),
            metadata: serde_json::json!({}),
            tags: vec![],
            audit_fields: AuditFields {
                created_at: now,
                updated_at: now,
                deleted_at: None,
            },
        }
    }

    /// Check if product is on sale
    pub fn is_on_sale(&self) -> bool {
        self.compare_at_price
            .map(|compare_price| compare_price > self.unit_price)
            .unwrap_or(false)
    }

    /// Get sale percentage
    pub fn sale_percentage(&self) -> Option<Decimal> {
        if let Some(compare_price) = self.compare_at_price {
            if compare_price > self.unit_price {
                let discount = compare_price - self.unit_price;
                Some((discount / compare_price) * Decimal::from(100))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Add an image URL
    pub fn add_image(&mut self, image_url: String) {
        if !self.images.contains(&image_url) {
            self.images.push(image_url);
            self.audit_fields.updated_at = Utc::now();
        }
    }

    /// Remove an image URL
    pub fn remove_image(&mut self, image_url: &str) {
        if let Some(index) = self.images.iter().position(|img| img == image_url) {
            self.images.remove(index);
            self.audit_fields.updated_at = Utc::now();
        }
    }

    /// Add a tag
    pub fn add_tag(&mut self, tag: String) {
        let tag_lower = tag.to_lowercase();
        if !self.tags.iter().any(|t| t.to_lowercase() == tag_lower) {
            self.tags.push(tag);
            self.audit_fields.updated_at = Utc::now();
        }
    }

    /// Remove a tag
    pub fn remove_tag(&mut self, tag: &str) {
        let tag_lower = tag.to_lowercase();
        if let Some(index) = self.tags.iter().position(|t| t.to_lowercase() == tag_lower) {
            self.tags.remove(index);
            self.audit_fields.updated_at = Utc::now();
        }
    }

    /// Get primary image
    pub fn primary_image(&self) -> Option<&String> {
        self.images.first()
    }

    /// Get display name (name with brand if available)
    pub fn display_name(&self) -> String {
        match &self.brand {
            Some(brand) => format!("{} {}", brand, self.name),
            None => self.name.clone(),
        }
    }
}

impl TenantScoped for Product {
    fn tenant_id(&self) -> Uuid {
        self.tenant_id
    }
}

impl SoftDelete for Product {
    fn is_deleted(&self) -> bool {
        self.audit_fields.deleted_at.is_some()
    }

    fn delete(&mut self) {
        self.audit_fields.deleted_at = Some(Utc::now());
        self.audit_fields.updated_at = Utc::now();
        self.is_active = false;
    }

    fn restore(&mut self) {
        self.audit_fields.deleted_at = None;
        self.audit_fields.updated_at = Utc::now();
        self.is_active = true;
    }
}

impl Searchable for Product {
    fn search_fields(&self) -> Vec<String> {
        let mut fields = vec![
            self.name.clone(),
            self.sku.clone(),
        ];

        if let Some(description) = &self.description {
            fields.push(description.clone());
        }

        if let Some(brand) = &self.brand {
            fields.push(brand.clone());
        }

        fields.extend(self.tags.clone());
        fields
    }
}

impl ValidateEntity for Product {
    type Error = ValidationError;

    fn validate(&self) -> Result<(), Self::Error> {
        // SKU validation
        if self.sku.trim().is_empty() {
            return Err(ValidationError::new("empty_sku"));
        }

        // Name validation
        if self.name.trim().is_empty() {
            return Err(ValidationError::new("empty_name"));
        }

        // Price validation
        if self.unit_price < Decimal::ZERO {
            return Err(ValidationError::new("negative_price"));
        }

        // Compare price validation
        if let Some(compare_price) = self.compare_at_price {
            if compare_price < Decimal::ZERO {
                return Err(ValidationError::new("negative_compare_price"));
            }
        }

        // Cost validation
        if let Some(cost) = self.cost {
            if cost < Decimal::ZERO {
                return Err(ValidationError::new("negative_cost"));
            }
        }

        // Weight validation
        if let Some(weight) = self.weight_value {
            if weight < Decimal::ZERO {
                return Err(ValidationError::new("negative_weight"));
            }
        }

        Ok(())
    }
}

/// Product variant entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductVariant {
    pub id: Uuid,
    pub product_id: Uuid,
    pub sku: String,
    pub name: Option<String>,
    pub options: serde_json::Value,
    pub price: Option<Decimal>,
    pub compare_at_price: Option<Decimal>,
    pub cost: Option<Decimal>,
    pub weight_value: Option<Decimal>,
    pub weight_unit: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ProductVariant {
    /// Create a new product variant
    pub fn new(product_id: Uuid, sku: String, options: serde_json::Value) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            product_id,
            sku,
            name: None,
            options,
            price: None,
            compare_at_price: None,
            cost: None,
            weight_value: None,
            weight_unit: None,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// Get variant display name
    pub fn display_name(&self) -> String {
        if let Some(name) = &self.name {
            name.clone()
        } else {
            // Create name from options
            if let Ok(options_map) = serde_json::from_value::<std::collections::HashMap<String, serde_json::Value>>(self.options.clone()) {
                let option_strings: Vec<String> = options_map
                    .iter()
                    .map(|(key, value)| {
                        format!("{}: {}", key, value.as_str().unwrap_or(""))
                    })
                    .collect();
                option_strings.join(", ")
            } else {
                format!("Variant {}", self.sku)
            }
        }
    }

    /// Check if variant is on sale
    pub fn is_on_sale(&self, product_price: Decimal) -> bool {
        let variant_price = self.price.unwrap_or(product_price);
        self.compare_at_price
            .map(|compare_price| compare_price > variant_price)
            .unwrap_or(false)
    }
}

/// Product category entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub sort_order: i32,
    pub is_active: bool,
    pub metadata: serde_json::Value,
    #[sqlx(flatten)]
    pub audit_fields: AuditFields,
}

impl Category {
    /// Create a new category
    pub fn new(tenant_id: Uuid, name: String, slug: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id,
            parent_id: None,
            name,
            slug,
            description: None,
            image_url: None,
            sort_order: 0,
            is_active: true,
            metadata: serde_json::json!({}),
            audit_fields: AuditFields {
                created_at: now,
                updated_at: now,
                deleted_at: None,
            },
        }
    }

    /// Check if this is a root category
    pub fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }

    /// Generate slug from name
    pub fn generate_slug(name: &str) -> String {
        name.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .join("-")
    }
}

impl TenantScoped for Category {
    fn tenant_id(&self) -> Uuid {
        self.tenant_id
    }
}

impl SoftDelete for Category {
    fn is_deleted(&self) -> bool {
        self.audit_fields.deleted_at.is_some()
    }

    fn delete(&mut self) {
        self.audit_fields.deleted_at = Some(Utc::now());
        self.audit_fields.updated_at = Utc::now();
        self.is_active = false;
    }

    fn restore(&mut self) {
        self.audit_fields.deleted_at = None;
        self.audit_fields.updated_at = Utc::now();
        self.is_active = true;
    }
}

/// Request to create a new product
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateProductRequest {
    #[validate(length(min = 1, max = 100))]
    pub sku: String,

    #[validate(length(min = 1, max = 500))]
    pub name: String,

    #[validate(length(max = 2000))]
    pub description: Option<String>,

    pub category_id: Option<Uuid>,

    #[validate(length(max = 255))]
    pub brand: Option<String>,

    pub unit_price: Decimal,
    pub compare_at_price: Option<Decimal>,
    pub cost: Option<Decimal>,
    pub tax_rate: Option<Decimal>,
    pub weight_value: Option<Decimal>,
    pub weight_unit: Option<String>,
    pub dimensions: Option<serde_json::Value>,
    pub is_digital: Option<bool>,
    pub requires_shipping: Option<bool>,
    pub track_inventory: Option<bool>,
    pub allow_backorder: Option<bool>,
    pub images: Option<Vec<String>>,
    pub attributes: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub tags: Option<Vec<String>>,
}

/// Request to update a product
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateProductRequest {
    #[validate(length(min = 1, max = 100))]
    pub sku: Option<String>,

    #[validate(length(min = 1, max = 500))]
    pub name: Option<String>,

    #[validate(length(max = 2000))]
    pub description: Option<String>,

    pub category_id: Option<Uuid>,

    #[validate(length(max = 255))]
    pub brand: Option<String>,

    pub unit_price: Option<Decimal>,
    pub compare_at_price: Option<Decimal>,
    pub cost: Option<Decimal>,
    pub tax_rate: Option<Decimal>,
    pub weight_value: Option<Decimal>,
    pub weight_unit: Option<String>,
    pub dimensions: Option<serde_json::Value>,
    pub is_digital: Option<bool>,
    pub is_active: Option<bool>,
    pub requires_shipping: Option<bool>,
    pub track_inventory: Option<bool>,
    pub allow_backorder: Option<bool>,
    pub images: Option<Vec<String>>,
    pub attributes: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub tags: Option<Vec<String>>,
}

/// Product with inventory information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductWithInventory {
    #[serde(flatten)]
    pub product: Product,
    pub total_inventory: i32,
    pub available_inventory: i32,
    pub reserved_inventory: i32,
}

/// Product summary for lists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSummary {
    pub id: Uuid,
    pub sku: String,
    pub name: String,
    pub brand: Option<String>,
    pub unit_price: Decimal,
    pub compare_at_price: Option<Decimal>,
    pub is_active: bool,
    pub primary_image: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<Product> for ProductSummary {
    fn from(product: Product) -> Self {
        let primary_image = product.primary_image().cloned();
        Self {
            id: product.id,
            sku: product.sku,
            name: product.name,
            brand: product.brand,
            unit_price: product.unit_price,
            compare_at_price: product.compare_at_price,
            is_active: product.is_active,
            primary_image,
            created_at: product.audit_fields.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_product_creation() {
        let tenant_id = Uuid::new_v4();
        let sku = "TEST-001".to_string();
        let name = "Test Product".to_string();
        let price = dec!(19.99);

        let product = Product::new(tenant_id, sku.clone(), name.clone(), price);

        assert_eq!(product.tenant_id, tenant_id);
        assert_eq!(product.sku, sku);
        assert_eq!(product.name, name);
        assert_eq!(product.unit_price, price);
        assert!(product.is_active);
        assert!(product.track_inventory);
    }

    #[test]
    fn test_product_sale_calculations() {
        let mut product = Product::new(
            Uuid::new_v4(),
            "TEST-001".to_string(),
            "Test Product".to_string(),
            dec!(15.00),
        );

        // Not on sale initially
        assert!(!product.is_on_sale());
        assert!(product.sale_percentage().is_none());

        // Set compare at price higher than unit price (on sale)
        product.compare_at_price = Some(dec!(20.00));
        assert!(product.is_on_sale());

        // Should be 25% off ((20-15)/20 * 100)
        let sale_percent = product.sale_percentage().unwrap();
        assert_eq!(sale_percent, dec!(25.00));
    }

    #[test]
    fn test_product_image_management() {
        let mut product = Product::new(
            Uuid::new_v4(),
            "TEST-001".to_string(),
            "Test Product".to_string(),
            dec!(19.99),
        );

        // Add images
        product.add_image("image1.jpg".to_string());
        product.add_image("image2.jpg".to_string());
        assert_eq!(product.images.len(), 2);
        assert_eq!(product.primary_image(), Some(&"image1.jpg".to_string()));

        // Remove image
        product.remove_image("image1.jpg");
        assert_eq!(product.images.len(), 1);
        assert_eq!(product.primary_image(), Some(&"image2.jpg".to_string()));
    }

    #[test]
    fn test_product_tag_management() {
        let mut product = Product::new(
            Uuid::new_v4(),
            "TEST-001".to_string(),
            "Test Product".to_string(),
            dec!(19.99),
        );

        // Add tags
        product.add_tag("electronics".to_string());
        product.add_tag("GADGETS".to_string()); // Should be case-insensitive
        assert_eq!(product.tags.len(), 2);

        // Don't add duplicate (case-insensitive)
        product.add_tag("Electronics".to_string());
        assert_eq!(product.tags.len(), 2);

        // Remove tag
        product.remove_tag("electronics");
        assert_eq!(product.tags.len(), 1);
    }

    #[test]
    fn test_product_search() {
        let product = Product::new(
            Uuid::new_v4(),
            "LAPTOP-001".to_string(),
            "Gaming Laptop".to_string(),
            dec!(999.99),
        );

        assert!(product.matches_search("laptop"));
        assert!(product.matches_search("GAMING"));
        assert!(product.matches_search("LAPTOP-001"));
        assert!(!product.matches_search("tablet"));
    }

    #[test]
    fn test_product_validation() {
        // Valid product
        let product = Product::new(
            Uuid::new_v4(),
            "TEST-001".to_string(),
            "Test Product".to_string(),
            dec!(19.99),
        );
        assert!(product.validate().is_ok());

        // Invalid product with negative price
        let mut invalid_product = product.clone();
        invalid_product.unit_price = dec!(-5.00);
        assert!(invalid_product.validate().is_err());

        // Invalid product with empty name
        let mut invalid_product = product.clone();
        invalid_product.name = "".to_string();
        assert!(invalid_product.validate().is_err());
    }

    #[test]
    fn test_product_variant() {
        let product_id = Uuid::new_v4();
        let options = serde_json::json!({"size": "Large", "color": "Red"});

        let variant = ProductVariant::new(
            product_id,
            "TEST-001-L-R".to_string(),
            options,
        );

        assert_eq!(variant.product_id, product_id);
        assert!(variant.is_active);
        assert!(variant.display_name().contains("size"));
        assert!(variant.display_name().contains("color"));
    }

    #[test]
    fn test_category() {
        let tenant_id = Uuid::new_v4();
        let name = "Electronics".to_string();
        let slug = Category::generate_slug(&name);

        let category = Category::new(tenant_id, name.clone(), slug.clone());

        assert_eq!(category.tenant_id, tenant_id);
        assert_eq!(category.name, name);
        assert_eq!(category.slug, "electronics");
        assert!(category.is_root());
        assert!(category.is_active);
    }

    #[test]
    fn test_slug_generation() {
        assert_eq!(Category::generate_slug("Electronics & Gadgets"), "electronics-gadgets");
        assert_eq!(Category::generate_slug("Home & Garden"), "home-garden");
        assert_eq!(Category::generate_slug("Books, Movies & Music"), "books-movies-music");
    }
}
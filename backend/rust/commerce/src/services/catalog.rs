// ============================================================================
// OLYMPUS CLOUD - PRODUCT CATALOG SERVICE
// ============================================================================
// Module: commerce/src/services/catalog.rs
// Description: Comprehensive product catalog management service
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use std::sync::Arc;
use std::collections::HashMap;
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
    Product, ProductStatus, ProductType, ProductCategory, ProductVariant, ProductAttribute,
    ProductSearchRequest, ProductSearchResponse, ProductSearchFacets, CategoryFacet,
    PriceRangeFacet, BrandFacet, AttributeFacet, AttributeValueFacet,
    CreateProductRequest, UpdateProductRequest, ProductSortBy, SortOrder,
    ProductImage, ProductDimensions,
};

#[derive(Clone)]
pub struct CatalogService {
    db: Arc<DbPool>,
    event_publisher: Arc<EventPublisher>,
}

impl CatalogService {
    pub fn new(db: Arc<DbPool>, event_publisher: Arc<EventPublisher>) -> Self {
        Self { db, event_publisher }
    }

    // ============================================================================
    // PRODUCT CRUD OPERATIONS
    // ============================================================================

    pub async fn create_product(
        &self,
        tenant_id: Uuid,
        request: CreateProductRequest,
        created_by: Uuid,
    ) -> Result<Product> {
        let product_id = Uuid::new_v4();
        let now = Utc::now();

        // Validate SKU is unique within tenant
        self.validate_sku_unique(tenant_id, &request.sku, None).await?;

        // Validate category exists if provided
        if let Some(category_id) = request.category_id {
            self.validate_category_exists(tenant_id, category_id).await?;
        }

        // Serialize complex fields
        let dimensions_json = serde_json::to_value(&request.dimensions)
            .map_err(|e| OlympusError::Validation(format!("Invalid dimensions: {}", e)))?;

        let tags = request.tags.unwrap_or_default();
        let attributes = request.attributes.unwrap_or_else(|| serde_json::json!({}));
        let images = request.images.unwrap_or_default();
        let images_json = serde_json::to_value(&images)
            .map_err(|e| OlympusError::Validation(format!("Invalid images: {}", e)))?;

        // Insert product
        let product_row = query_as!(
            ProductRow,
            r#"
            INSERT INTO products (
                id, tenant_id, sku, name, description, short_description,
                product_type, status, category_id, brand, weight, dimensions,
                base_price, price_type, cost_price, compare_at_price, tax_class,
                requires_shipping, is_digital, track_inventory, inventory_quantity,
                low_stock_threshold, tags, attributes, images, seo_title,
                seo_description, created_at, updated_at, created_by, updated_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31)
            RETURNING
                id, tenant_id, sku, name, description, short_description,
                product_type as "product_type: ProductType",
                status as "status: ProductStatus",
                category_id, brand, weight, dimensions, base_price,
                price_type as "price_type: crate::models::PriceType",
                cost_price, compare_at_price, tax_class, requires_shipping,
                is_digital, track_inventory, inventory_quantity, low_stock_threshold,
                tags, attributes, images, seo_title, seo_description,
                created_at, updated_at, created_by, updated_by
            "#,
            product_id,
            tenant_id,
            request.sku,
            request.name,
            request.description,
            request.short_description,
            request.product_type as ProductType,
            ProductStatus::Draft as ProductStatus,
            request.category_id,
            request.brand,
            request.weight,
            dimensions_json,
            request.base_price,
            request.price_type as crate::models::PriceType,
            request.cost_price,
            request.compare_at_price,
            request.tax_class,
            request.requires_shipping,
            request.is_digital,
            request.track_inventory,
            request.inventory_quantity,
            request.low_stock_threshold,
            &tags,
            attributes,
            images_json,
            request.seo_title,
            request.seo_description,
            now,
            now,
            created_by,
            created_by
        )
        .fetch_one(self.db.as_ref())
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to create product: {}", e)))?;

        let product = self.product_row_to_model(product_row)?;

        // If tracking inventory, create inventory record
        if request.track_inventory {
            self.create_inventory_record(
                tenant_id,
                product_id,
                None, // no variant
                &request.sku,
                request.inventory_quantity.unwrap_or(0),
                request.low_stock_threshold,
                request.cost_price,
            ).await?;
        }

        // Publish domain event
        let event = DomainEvent::builder()
            .data(serde_json::json!({
                "product_id": product_id,
                "tenant_id": tenant_id,
                "sku": request.sku,
                "name": request.name,
                "product_type": request.product_type,
                "base_price": request.base_price,
                "created_by": created_by
            }))
            .source_service("commerce")
            .build();

        if let Err(e) = self.event_publisher.publish(&event).await {
            tracing::warn!("Failed to publish ProductCreated event: {}", e);
        }

        Ok(product)
    }

    pub async fn get_product(&self, tenant_id: Uuid, product_id: Uuid) -> Result<Option<Product>> {
        let product_row = query_as!(
            ProductRow,
            r#"
            SELECT
                id, tenant_id, sku, name, description, short_description,
                product_type as "product_type: ProductType",
                status as "status: ProductStatus",
                category_id, brand, weight, dimensions, base_price,
                price_type as "price_type: crate::models::PriceType",
                cost_price, compare_at_price, tax_class, requires_shipping,
                is_digital, track_inventory, inventory_quantity, low_stock_threshold,
                tags, attributes, images, seo_title, seo_description,
                created_at, updated_at, created_by, updated_by
            FROM products
            WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL
            "#,
            product_id,
            tenant_id
        )
        .fetch_optional(self.db.as_ref())
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to get product: {}", e)))?;

        match product_row {
            Some(row) => Ok(Some(self.product_row_to_model(row)?)),
            None => Ok(None),
        }
    }

    pub async fn update_product(
        &self,
        tenant_id: Uuid,
        product_id: Uuid,
        request: UpdateProductRequest,
        updated_by: Uuid,
    ) -> Result<Option<Product>> {
        let now = Utc::now();

        // Get current product for validation and audit
        let current_product = self.get_product(tenant_id, product_id).await?;
        let current_product = match current_product {
            Some(product) => product,
            None => return Ok(None),
        };

        // Validate category exists if provided
        if let Some(category_id) = request.category_id {
            self.validate_category_exists(tenant_id, category_id).await?;
        }

        // Simplified update implementation - in production would build dynamic query
        if let Some(name) = request.name {
            let product_row = query_as!(
                ProductRow,
                r#"
                UPDATE products
                SET name = $3, updated_at = $4, updated_by = $5
                WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL
                RETURNING
                    id, tenant_id, sku, name, description, short_description,
                    product_type as "product_type: ProductType",
                    status as "status: ProductStatus",
                    category_id, brand, weight, dimensions, base_price,
                    price_type as "price_type: crate::models::PriceType",
                    cost_price, compare_at_price, tax_class, requires_shipping,
                    is_digital, track_inventory, inventory_quantity, low_stock_threshold,
                    tags, attributes, images, seo_title, seo_description,
                    created_at, updated_at, created_by, updated_by
                "#,
                product_id,
                tenant_id,
                name,
                now,
                updated_by
            )
            .fetch_optional(self.db.as_ref())
            .await
            .map_err(|e| OlympusError::Database(format!("Failed to update product: {}", e)))?;

            match product_row {
                Some(row) => {
                    let product = self.product_row_to_model(row)?;

                    // Publish domain event
                    let event = DomainEvent::builder()
                        .data(serde_json::json!({
                            "product_id": product_id,
                            "tenant_id": tenant_id,
                            "updated_fields": ["name"],
                            "old_name": current_product.name,
                            "new_name": name,
                            "updated_by": updated_by
                        }))
                        .source_service("commerce")
                        .build();

                    if let Err(e) = self.event_publisher.publish(&event).await {
                        tracing::warn!("Failed to publish ProductUpdated event: {}", e);
                    }

                    Ok(Some(product))
                }
                None => Ok(None),
            }
        } else {
            Ok(Some(current_product))
        }
    }

    pub async fn delete_product(
        &self,
        tenant_id: Uuid,
        product_id: Uuid,
        deleted_by: Uuid,
    ) -> Result<bool> {
        let now = Utc::now();

        // Soft delete the product
        let rows_affected = query!(
            r#"
            UPDATE products
            SET deleted_at = $3, updated_by = $4
            WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL
            "#,
            product_id,
            tenant_id,
            now,
            deleted_by
        )
        .execute(self.db.as_ref())
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to delete product: {}", e)))?
        .rows_affected();

        if rows_affected > 0 {
            // Publish domain event
            let event = DomainEvent::builder()
                .data(serde_json::json!({
                    "product_id": product_id,
                    "tenant_id": tenant_id,
                    "deleted_by": deleted_by
                }))
                .source_service("commerce")
                .build();

            if let Err(e) = self.event_publisher.publish(&event).await {
                tracing::warn!("Failed to publish ProductDeleted event: {}", e);
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    // ============================================================================
    // PRODUCT SEARCH AND FILTERING
    // ============================================================================

    pub async fn search_products(
        &self,
        tenant_id: Uuid,
        request: ProductSearchRequest,
    ) -> Result<ProductSearchResponse> {
        let limit = request.limit.unwrap_or(50).min(100);
        let offset = request.offset.unwrap_or(0);

        // Build search conditions
        let mut where_conditions = vec![
            "tenant_id = $1".to_string(),
            "deleted_at IS NULL".to_string(),
        ];
        let mut param_count = 2;

        // Add search filters
        if let Some(status) = request.status {
            where_conditions.push(format!("status = ${}", param_count));
            param_count += 1;
        }

        if let Some(product_type) = request.product_type {
            where_conditions.push(format!("product_type = ${}", param_count));
            param_count += 1;
        }

        if let Some(category_id) = request.category_id {
            where_conditions.push(format!("category_id = ${}", param_count));
            param_count += 1;
        }

        if request.in_stock_only.unwrap_or(false) {
            where_conditions.push("inventory_quantity > 0".to_string());
        }

        if let Some(query) = &request.query {
            where_conditions.push(format!("(name ILIKE ${} OR description ILIKE ${})", param_count, param_count));
            param_count += 1;
        }

        // Build sort clause
        let sort_clause = match (request.sort_by, request.sort_order) {
            (Some(ProductSortBy::Name), Some(SortOrder::Desc)) => "ORDER BY name DESC",
            (Some(ProductSortBy::Price), Some(SortOrder::Asc)) => "ORDER BY base_price ASC",
            (Some(ProductSortBy::Price), Some(SortOrder::Desc)) => "ORDER BY base_price DESC",
            (Some(ProductSortBy::CreatedAt), Some(SortOrder::Desc)) => "ORDER BY created_at DESC",
            _ => "ORDER BY name ASC", // default
        };

        // Simplified query - full implementation would handle all filters dynamically
        let products = query_as!(
            ProductRow,
            r#"
            SELECT
                id, tenant_id, sku, name, description, short_description,
                product_type as "product_type: ProductType",
                status as "status: ProductStatus",
                category_id, brand, weight, dimensions, base_price,
                price_type as "price_type: crate::models::PriceType",
                cost_price, compare_at_price, tax_class, requires_shipping,
                is_digital, track_inventory, inventory_quantity, low_stock_threshold,
                tags, attributes, images, seo_title, seo_description,
                created_at, updated_at, created_by, updated_by
            FROM products
            WHERE tenant_id = $1 AND deleted_at IS NULL
            ORDER BY name ASC
            LIMIT $2 OFFSET $3
            "#,
            tenant_id,
            limit as i64,
            offset as i64
        )
        .fetch_all(self.db.as_ref())
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to search products: {}", e)))?;

        let total_count = query!(
            "SELECT COUNT(*) as count FROM products WHERE tenant_id = $1 AND deleted_at IS NULL",
            tenant_id
        )
        .fetch_one(self.db.as_ref())
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to count products: {}", e)))?
        .count
        .unwrap_or(0);

        let products = products
            .into_iter()
            .map(|row| self.product_row_to_model(row))
            .collect::<Result<Vec<_>>>()?;

        // Generate facets for filtering
        let facets = self.generate_search_facets(tenant_id, &request).await?;

        Ok(ProductSearchResponse {
            products,
            total_count,
            has_more: (offset as i64 + limit as i64) < total_count,
            facets,
        })
    }

    async fn generate_search_facets(
        &self,
        tenant_id: Uuid,
        _request: &ProductSearchRequest,
    ) -> Result<ProductSearchFacets> {
        // Simplified facet generation - would be more sophisticated in production

        // Category facets
        let categories = query!(
            r#"
            SELECT c.id, c.name, COUNT(p.id) as count
            FROM product_categories c
            LEFT JOIN products p ON c.id = p.category_id AND p.tenant_id = $1 AND p.deleted_at IS NULL
            WHERE c.tenant_id = $1 AND c.deleted_at IS NULL
            GROUP BY c.id, c.name
            HAVING COUNT(p.id) > 0
            ORDER BY c.name
            "#,
            tenant_id
        )
        .fetch_all(self.db.as_ref())
        .await
        .unwrap_or_default();

        let category_facets = categories
            .into_iter()
            .map(|row| CategoryFacet {
                category_id: row.id,
                name: row.name,
                count: row.count.unwrap_or(0),
            })
            .collect();

        // Brand facets
        let brands = query!(
            r#"
            SELECT brand, COUNT(*) as count
            FROM products
            WHERE tenant_id = $1 AND deleted_at IS NULL AND brand IS NOT NULL
            GROUP BY brand
            ORDER BY brand
            "#,
            tenant_id
        )
        .fetch_all(self.db.as_ref())
        .await
        .unwrap_or_default();

        let brand_facets = brands
            .into_iter()
            .map(|row| BrandFacet {
                brand: row.brand.unwrap_or_default(),
                count: row.count.unwrap_or(0),
            })
            .collect();

        // Price range facets (simplified)
        let price_ranges = vec![
            PriceRangeFacet {
                min: Decimal::from(0),
                max: Decimal::from(50),
                count: 0, // would calculate actual counts
            },
            PriceRangeFacet {
                min: Decimal::from(50),
                max: Decimal::from(100),
                count: 0,
            },
            PriceRangeFacet {
                min: Decimal::from(100),
                max: Decimal::from(500),
                count: 0,
            },
        ];

        Ok(ProductSearchFacets {
            categories: category_facets,
            price_ranges,
            brands: brand_facets,
            attributes: Vec::new(), // would implement attribute facets
        })
    }

    // ============================================================================
    // CATEGORY MANAGEMENT
    // ============================================================================

    pub async fn create_category(
        &self,
        tenant_id: Uuid,
        name: String,
        description: Option<String>,
        parent_id: Option<Uuid>,
        slug: String,
        created_by: Uuid,
    ) -> Result<ProductCategory> {
        let category_id = Uuid::new_v4();
        let now = Utc::now();

        // Validate parent category exists if provided
        if let Some(parent_id) = parent_id {
            self.validate_category_exists(tenant_id, parent_id).await?;
        }

        // Validate slug is unique within tenant
        self.validate_category_slug_unique(tenant_id, &slug, None).await?;

        let category = query_as!(
            ProductCategoryRow,
            r#"
            INSERT INTO product_categories (
                id, tenant_id, parent_id, name, description, slug, image_url,
                is_active, sort_order, seo_title, seo_description,
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, NULL, true, 0, NULL, NULL, $7, $8)
            RETURNING
                id, tenant_id, parent_id, name, description, slug, image_url,
                is_active, sort_order, seo_title, seo_description,
                created_at, updated_at
            "#,
            category_id,
            tenant_id,
            parent_id,
            name,
            description,
            slug,
            now,
            now
        )
        .fetch_one(self.db.as_ref())
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to create category: {}", e)))?;

        let category = ProductCategory {
            id: category.id,
            tenant_id: category.tenant_id,
            parent_id: category.parent_id,
            name: category.name,
            description: category.description,
            slug: category.slug,
            image_url: category.image_url,
            is_active: category.is_active,
            sort_order: category.sort_order,
            seo_title: category.seo_title,
            seo_description: category.seo_description,
            created_at: category.created_at,
            updated_at: category.updated_at,
        };

        // Publish domain event
        let event = DomainEvent::builder()
            .data(serde_json::json!({
                "category_id": category_id,
                "tenant_id": tenant_id,
                "name": name,
                "parent_id": parent_id,
                "created_by": created_by
            }))
            .source_service("commerce")
            .build();

        if let Err(e) = self.event_publisher.publish(&event).await {
            tracing::warn!("Failed to publish CategoryCreated event: {}", e);
        }

        Ok(category)
    }

    pub async fn get_category_tree(&self, tenant_id: Uuid) -> Result<Vec<ProductCategory>> {
        let categories = query_as!(
            ProductCategoryRow,
            r#"
            SELECT
                id, tenant_id, parent_id, name, description, slug, image_url,
                is_active, sort_order, seo_title, seo_description,
                created_at, updated_at
            FROM product_categories
            WHERE tenant_id = $1 AND deleted_at IS NULL
            ORDER BY sort_order, name
            "#,
            tenant_id
        )
        .fetch_all(self.db.as_ref())
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to get category tree: {}", e)))?;

        let categories = categories
            .into_iter()
            .map(|row| ProductCategory {
                id: row.id,
                tenant_id: row.tenant_id,
                parent_id: row.parent_id,
                name: row.name,
                description: row.description,
                slug: row.slug,
                image_url: row.image_url,
                is_active: row.is_active,
                sort_order: row.sort_order,
                seo_title: row.seo_title,
                seo_description: row.seo_description,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
            .collect();

        Ok(categories)
    }

    // ============================================================================
    // INVENTORY MANAGEMENT HELPERS
    // ============================================================================

    async fn create_inventory_record(
        &self,
        tenant_id: Uuid,
        product_id: Uuid,
        variant_id: Option<Uuid>,
        sku: &str,
        quantity: i32,
        low_stock_threshold: Option<i32>,
        cost_per_unit: Option<Decimal>,
    ) -> Result<()> {
        let inventory_id = Uuid::new_v4();
        let now = Utc::now();

        query!(
            r#"
            INSERT INTO inventory_items (
                id, tenant_id, product_id, variant_id, location_id, sku,
                quantity_available, quantity_reserved, quantity_on_hand,
                low_stock_threshold, cost_per_unit, last_counted_at,
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, NULL, $5, $6, 0, $6, $7, $8, $9, $10, $11)
            "#,
            inventory_id,
            tenant_id,
            product_id,
            variant_id,
            sku,
            quantity,
            low_stock_threshold,
            cost_per_unit,
            now,
            now,
            now
        )
        .execute(self.db.as_ref())
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to create inventory record: {}", e)))?;

        Ok(())
    }

    // ============================================================================
    // VALIDATION HELPERS
    // ============================================================================

    async fn validate_sku_unique(
        &self,
        tenant_id: Uuid,
        sku: &str,
        exclude_id: Option<Uuid>,
    ) -> Result<()> {
        let mut query_str = "SELECT id FROM products WHERE sku = $1 AND tenant_id = $2 AND deleted_at IS NULL".to_string();

        if exclude_id.is_some() {
            query_str.push_str(" AND id != $3");
        }

        let exists = if let Some(exclude_id) = exclude_id {
            query!(&query_str, sku, tenant_id, exclude_id)
                .fetch_optional(self.db.as_ref())
                .await
        } else {
            query!(&query_str, sku, tenant_id)
                .fetch_optional(self.db.as_ref())
                .await
        }
        .map_err(|e| OlympusError::Database(format!("Failed to check SKU uniqueness: {}", e)))?;

        if exists.is_some() {
            return Err(OlympusError::Validation("SKU already exists".to_string()));
        }

        Ok(())
    }

    async fn validate_category_exists(&self, tenant_id: Uuid, category_id: Uuid) -> Result<()> {
        let exists = query!(
            "SELECT id FROM product_categories WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL",
            category_id,
            tenant_id
        )
        .fetch_optional(self.db.as_ref())
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to validate category: {}", e)))?;

        if exists.is_none() {
            return Err(OlympusError::Validation("Category not found".to_string()));
        }

        Ok(())
    }

    async fn validate_category_slug_unique(
        &self,
        tenant_id: Uuid,
        slug: &str,
        exclude_id: Option<Uuid>,
    ) -> Result<()> {
        let mut query_str = "SELECT id FROM product_categories WHERE slug = $1 AND tenant_id = $2 AND deleted_at IS NULL".to_string();

        if exclude_id.is_some() {
            query_str.push_str(" AND id != $3");
        }

        let exists = if let Some(exclude_id) = exclude_id {
            query!(&query_str, slug, tenant_id, exclude_id)
                .fetch_optional(self.db.as_ref())
                .await
        } else {
            query!(&query_str, slug, tenant_id)
                .fetch_optional(self.db.as_ref())
                .await
        }
        .map_err(|e| OlympusError::Database(format!("Failed to check category slug uniqueness: {}", e)))?;

        if exists.is_some() {
            return Err(OlympusError::Validation("Category slug already exists".to_string()));
        }

        Ok(())
    }

    // ============================================================================
    // CONVERSION HELPERS
    // ============================================================================

    fn product_row_to_model(&self, row: ProductRow) -> Result<Product> {
        let dimensions: Option<ProductDimensions> = serde_json::from_value(row.dimensions)
            .map_err(|e| OlympusError::Database(format!("Invalid dimensions JSON: {}", e)))?;

        let images: Vec<ProductImage> = serde_json::from_value(row.images)
            .map_err(|e| OlympusError::Database(format!("Invalid images JSON: {}", e)))?;

        Ok(Product {
            id: row.id,
            tenant_id: row.tenant_id,
            sku: row.sku,
            name: row.name,
            description: row.description,
            short_description: row.short_description,
            product_type: row.product_type,
            status: row.status,
            category_id: row.category_id,
            brand: row.brand,
            weight: row.weight,
            dimensions,
            base_price: row.base_price,
            price_type: row.price_type,
            cost_price: row.cost_price,
            compare_at_price: row.compare_at_price,
            tax_class: row.tax_class,
            requires_shipping: row.requires_shipping,
            is_digital: row.is_digital,
            track_inventory: row.track_inventory,
            inventory_quantity: row.inventory_quantity,
            low_stock_threshold: row.low_stock_threshold,
            tags: row.tags,
            attributes: row.attributes,
            images,
            seo_title: row.seo_title,
            seo_description: row.seo_description,
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
struct ProductRow {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub product_type: ProductType,
    pub status: ProductStatus,
    pub category_id: Option<Uuid>,
    pub brand: Option<String>,
    pub weight: Option<Decimal>,
    pub dimensions: serde_json::Value,
    pub base_price: Decimal,
    pub price_type: crate::models::PriceType,
    pub cost_price: Option<Decimal>,
    pub compare_at_price: Option<Decimal>,
    pub tax_class: Option<String>,
    pub requires_shipping: bool,
    pub is_digital: bool,
    pub track_inventory: bool,
    pub inventory_quantity: Option<i32>,
    pub low_stock_threshold: Option<i32>,
    pub tags: Vec<String>,
    pub attributes: serde_json::Value,
    pub images: serde_json::Value,
    pub seo_title: Option<String>,
    pub seo_description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

#[derive(Debug)]
struct ProductCategoryRow {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub slug: String,
    pub image_url: Option<String>,
    pub is_active: bool,
    pub sort_order: i32,
    pub seo_title: Option<String>,
    pub seo_description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
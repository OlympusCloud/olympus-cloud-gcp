//! Advanced caching layer with Redis

use async_trait::async_trait;
use redis::{aio::ConnectionManager, AsyncCommands, RedisError};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    collections::HashMap,
    sync::Arc,
    time::Duration,
};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Default TTL in seconds
    pub default_ttl: u64,
    /// Maximum cache size in MB
    pub max_size_mb: usize,
    /// Enable cache warming
    pub warm_on_startup: bool,
    /// Cache key prefix
    pub key_prefix: String,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            default_ttl: 3600, // 1 hour
            max_size_mb: 100,
            warm_on_startup: true,
            key_prefix: "olympus".to_string(),
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub sets: u64,
    pub deletes: u64,
    pub errors: u64,
}

impl CacheStats {
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
}

/// Main cache service
pub struct CacheService {
    redis: ConnectionManager,
    config: CacheConfig,
    stats: Arc<RwLock<CacheStats>>,
    local_cache: Arc<RwLock<HashMap<String, CachedValue>>>,
}

#[derive(Debug, Clone)]
struct CachedValue {
    data: Vec<u8>,
    expires_at: std::time::Instant,
}

impl CacheService {
    pub async fn new(redis_url: &str, config: CacheConfig) -> Result<Self, RedisError> {
        let client = redis::Client::open(redis_url)?;
        let redis = ConnectionManager::new(client).await?;

        let service = Self {
            redis,
            config,
            stats: Arc::new(RwLock::new(CacheStats::default())),
            local_cache: Arc::new(RwLock::new(HashMap::new())),
        };

        if config.warm_on_startup {
            service.warm_cache().await;
        }

        // Start cache cleanup task
        let cache_clone = service.local_cache.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                cleanup_expired_local_cache(&cache_clone).await;
            }
        });

        Ok(service)
    }

    /// Get value from cache
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        let full_key = self.build_key(key);

        // Check local cache first
        if let Some(value) = self.get_local(&full_key).await {
            self.record_hit().await;
            return serde_json::from_slice(&value).ok();
        }

        // Check Redis
        match self.redis.clone().get::<_, Vec<u8>>(&full_key).await {
            Ok(data) => {
                self.record_hit().await;

                // Update local cache
                self.set_local(&full_key, data.clone(), self.config.default_ttl).await;

                serde_json::from_slice(&data).ok()
            }
            Err(_) => {
                self.record_miss().await;
                None
            }
        }
    }

    /// Set value in cache
    pub async fn set<T: Serialize>(&self, key: &str, value: &T, ttl_seconds: Option<u64>) -> Result<(), CacheError> {
        let full_key = self.build_key(key);
        let ttl = ttl_seconds.unwrap_or(self.config.default_ttl);
        let data = serde_json::to_vec(value)?;

        // Set in Redis
        self.redis
            .clone()
            .set_ex(&full_key, &data, ttl)
            .await?;

        // Set in local cache
        self.set_local(&full_key, data, ttl).await;

        self.record_set().await;
        Ok(())
    }

    /// Delete value from cache
    pub async fn delete(&self, key: &str) -> Result<(), CacheError> {
        let full_key = self.build_key(key);

        // Delete from Redis
        self.redis.clone().del(&full_key).await?;

        // Delete from local cache
        self.delete_local(&full_key).await;

        self.record_delete().await;
        Ok(())
    }

    /// Delete multiple keys by pattern
    pub async fn delete_pattern(&self, pattern: &str) -> Result<u64, CacheError> {
        let full_pattern = self.build_key(pattern);

        let keys: Vec<String> = self.redis.clone()
            .keys(&full_pattern)
            .await?;

        if keys.is_empty() {
            return Ok(0);
        }

        let count = keys.len() as u64;

        // Delete from Redis
        self.redis.clone().del::<_, ()>(&keys).await?;

        // Delete from local cache
        for key in &keys {
            self.delete_local(key).await;
        }

        self.record_delete().await;
        Ok(count)
    }

    /// Cache warming on startup
    async fn warm_cache(&self) {
        info!("Starting cache warming...");

        // Warm frequently accessed data
        // This would be customized based on your application's needs

        info!("Cache warming completed");
    }

    /// Build full cache key
    fn build_key(&self, key: &str) -> String {
        format!("{}:{}", self.config.key_prefix, key)
    }

    /// Local cache operations
    async fn get_local(&self, key: &str) -> Option<Vec<u8>> {
        let cache = self.local_cache.read().await;
        cache.get(key).and_then(|v| {
            if v.expires_at > std::time::Instant::now() {
                Some(v.data.clone())
            } else {
                None
            }
        })
    }

    async fn set_local(&self, key: &str, data: Vec<u8>, ttl_seconds: u64) {
        let mut cache = self.local_cache.write().await;
        let expires_at = std::time::Instant::now() + Duration::from_secs(ttl_seconds);
        cache.insert(key.to_string(), CachedValue { data, expires_at });
    }

    async fn delete_local(&self, key: &str) {
        let mut cache = self.local_cache.write().await;
        cache.remove(key);
    }

    /// Statistics tracking
    async fn record_hit(&self) {
        let mut stats = self.stats.write().await;
        stats.hits += 1;
    }

    async fn record_miss(&self) {
        let mut stats = self.stats.write().await;
        stats.misses += 1;
    }

    async fn record_set(&self) {
        let mut stats = self.stats.write().await;
        stats.sets += 1;
    }

    async fn record_delete(&self) {
        let mut stats = self.stats.write().await;
        stats.deletes += 1;
    }

    pub async fn get_stats(&self) -> CacheStats {
        self.stats.read().await.clone()
    }
}

/// Cleanup expired entries from local cache
async fn cleanup_expired_local_cache(cache: &Arc<RwLock<HashMap<String, CachedValue>>>) {
    let mut cache = cache.write().await;
    let now = std::time::Instant::now();
    cache.retain(|_, v| v.expires_at > now);
}

/// Cache error types
#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    #[error("Redis error: {0}")]
    Redis(#[from] RedisError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Cache key not found")]
    KeyNotFound,
}

/// Cacheable trait for automatic caching
#[async_trait]
pub trait Cacheable {
    type Key;
    type Value: Serialize + DeserializeOwned;

    fn cache_key(&self, key: &Self::Key) -> String;
    fn cache_ttl(&self) -> Option<u64> {
        None
    }

    async fn get_from_cache(
        &self,
        cache: &CacheService,
        key: &Self::Key,
    ) -> Option<Self::Value> {
        cache.get(&self.cache_key(key)).await
    }

    async fn set_in_cache(
        &self,
        cache: &CacheService,
        key: &Self::Key,
        value: &Self::Value,
    ) -> Result<(), CacheError> {
        cache.set(&self.cache_key(key), value, self.cache_ttl()).await
    }

    async fn invalidate_cache(
        &self,
        cache: &CacheService,
        key: &Self::Key,
    ) -> Result<(), CacheError> {
        cache.delete(&self.cache_key(key)).await
    }
}

/// Cache patterns for common use cases
pub mod patterns {
    use super::*;

    /// Cache-aside pattern
    pub async fn cache_aside<K, V, F, Fut>(
        cache: &CacheService,
        key: &str,
        ttl: Option<u64>,
        fetch_fn: F,
    ) -> Result<V, Box<dyn std::error::Error>>
    where
        V: Serialize + DeserializeOwned,
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<V, Box<dyn std::error::Error>>>,
    {
        // Try to get from cache
        if let Some(value) = cache.get::<V>(key).await {
            debug!("Cache hit for key: {}", key);
            return Ok(value);
        }

        // Fetch from source
        debug!("Cache miss for key: {}, fetching from source", key);
        let value = fetch_fn().await?;

        // Store in cache
        if let Err(e) = cache.set(key, &value, ttl).await {
            warn!("Failed to cache value: {}", e);
        }

        Ok(value)
    }

    /// Write-through pattern
    pub async fn write_through<K, V, F, Fut>(
        cache: &CacheService,
        key: &str,
        value: &V,
        ttl: Option<u64>,
        persist_fn: F,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        V: Serialize,
        F: FnOnce(&V) -> Fut,
        Fut: std::future::Future<Output = Result<(), Box<dyn std::error::Error>>>,
    {
        // Write to persistent store first
        persist_fn(value).await?;

        // Then update cache
        cache.set(key, value, ttl).await?;

        Ok(())
    }

    /// Cache invalidation helpers
    pub async fn invalidate_tenant_cache(
        cache: &CacheService,
        tenant_id: Uuid,
    ) -> Result<(), CacheError> {
        let pattern = format!("tenant:{}:*", tenant_id);
        let count = cache.delete_pattern(&pattern).await?;
        info!("Invalidated {} cache entries for tenant {}", count, tenant_id);
        Ok(())
    }

    pub async fn invalidate_user_cache(
        cache: &CacheService,
        user_id: Uuid,
    ) -> Result<(), CacheError> {
        let pattern = format!("user:{}:*", user_id);
        let count = cache.delete_pattern(&pattern).await?;
        info!("Invalidated {} cache entries for user {}", count, user_id);
        Ok(())
    }
}

/// Cache middleware for Axum
pub mod middleware {
    use super::*;
    use axum::{
        body::Body,
        extract::Request,
        http::{header, StatusCode},
        middleware::Next,
        response::{IntoResponse, Response},
    };
    use bytes::Bytes;

    pub async fn cache_middleware(
        State(cache): State<Arc<CacheService>>,
        request: Request,
        next: Next,
    ) -> Response {
        // Only cache GET requests
        if request.method() != axum::http::Method::GET {
            return next.run(request).await;
        }

        let cache_key = format!("http:{}", request.uri().path_and_query().map(|pq| pq.as_str()).unwrap_or("/"));

        // Check cache
        if let Some(cached_response) = cache.get::<CachedResponse>(&cache_key).await {
            return (
                StatusCode::OK,
                [(header::CONTENT_TYPE, cached_response.content_type)],
                cached_response.body,
            ).into_response();
        }

        // Execute request
        let response = next.run(request).await;

        // Cache successful responses
        if response.status() == StatusCode::OK {
            let (parts, body) = response.into_parts();
            let body_bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap_or_default();

            let cached = CachedResponse {
                content_type: parts.headers
                    .get(header::CONTENT_TYPE)
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("application/json")
                    .to_string(),
                body: body_bytes.clone(),
            };

            // Cache for 5 minutes
            let _ = cache.set(&cache_key, &cached, Some(300)).await;

            Response::from_parts(parts, Body::from(body_bytes))
        } else {
            response
        }
    }

    #[derive(Serialize, Deserialize)]
    struct CachedResponse {
        content_type: String,
        body: Bytes,
    }
}
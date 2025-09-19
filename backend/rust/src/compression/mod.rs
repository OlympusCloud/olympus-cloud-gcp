//! Request/response compression middleware and utilities

use axum::{
    body::Body,
    extract::Request,
    http::{header, HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use flate2::{write::GzEncoder, Compression as GzCompression};
use std::io::Write;
use tower::ServiceBuilder;
use tower_http::compression::{CompressionLayer, CompressionLevel, Predicate};
use tracing::{debug, error, warn};

/// Compression configuration
#[derive(Debug, Clone)]
pub struct CompressionConfig {
    /// Compression level (0-9 for gzip, 1-22 for brotli)
    pub level: CompressionLevel,
    /// Minimum response size to compress (bytes)
    pub min_size: usize,
    /// Maximum response size to compress (bytes)
    pub max_size: usize,
    /// Enable gzip compression
    pub enable_gzip: bool,
    /// Enable brotli compression
    pub enable_br: bool,
    /// Enable deflate compression
    pub enable_deflate: bool,
    /// Content types to always compress
    pub always_compress: Vec<String>,
    /// Content types to never compress
    pub never_compress: Vec<String>,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            level: CompressionLevel::Default,
            min_size: 1024, // 1KB
            max_size: 10 * 1024 * 1024, // 10MB
            enable_gzip: true,
            enable_br: true,
            enable_deflate: true,
            always_compress: vec![
                "application/json".to_string(),
                "application/xml".to_string(),
                "text/html".to_string(),
                "text/css".to_string(),
                "text/javascript".to_string(),
                "text/plain".to_string(),
                "application/javascript".to_string(),
            ],
            never_compress: vec![
                "image/jpeg".to_string(),
                "image/png".to_string(),
                "image/gif".to_string(),
                "image/webp".to_string(),
                "video/mp4".to_string(),
                "video/webm".to_string(),
                "audio/mpeg".to_string(),
                "application/zip".to_string(),
                "application/gzip".to_string(),
                "application/pdf".to_string(),
            ],
        }
    }
}

/// Smart compression predicate based on content type and size
#[derive(Clone)]
pub struct SmartCompressionPredicate {
    config: CompressionConfig,
}

impl SmartCompressionPredicate {
    pub fn new(config: CompressionConfig) -> Self {
        Self { config }
    }
}

impl<B> Predicate<B> for SmartCompressionPredicate
where
    B: axum::body::HttpBody,
{
    fn should_compress(&self, response: &http::Response<B>) -> bool {
        // Check status code - only compress successful responses
        if !response.status().is_success() {
            return false;
        }

        // Get content type
        let content_type = response
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        // Check if content type is explicitly excluded
        if self.config.never_compress.iter().any(|ct| content_type.starts_with(ct)) {
            debug!("Skipping compression for never-compress content type: {}", content_type);
            return false;
        }

        // Check if content type should always be compressed
        if self.config.always_compress.iter().any(|ct| content_type.starts_with(ct)) {
            debug!("Compressing always-compress content type: {}", content_type);
            return true;
        }

        // Check content length if available
        if let Some(length_header) = response.headers().get(header::CONTENT_LENGTH) {
            if let Ok(length_str) = length_header.to_str() {
                if let Ok(length) = length_str.parse::<usize>() {
                    if length < self.config.min_size {
                        debug!("Skipping compression - content too small: {} bytes", length);
                        return false;
                    }
                    if length > self.config.max_size {
                        debug!("Skipping compression - content too large: {} bytes", length);
                        return false;
                    }
                }
            }
        }

        // Default to compressing text-based content
        content_type.starts_with("text/") ||
        content_type.starts_with("application/json") ||
        content_type.starts_with("application/xml") ||
        content_type.starts_with("application/javascript")
    }
}

/// Compression statistics
#[derive(Debug, Clone, Default)]
pub struct CompressionStats {
    pub total_requests: u64,
    pub compressed_requests: u64,
    pub total_bytes_in: u64,
    pub total_bytes_out: u64,
    pub gzip_requests: u64,
    pub brotli_requests: u64,
    pub deflate_requests: u64,
}

impl CompressionStats {
    /// Calculate compression ratio
    pub fn compression_ratio(&self) -> f64 {
        if self.total_bytes_in == 0 {
            1.0
        } else {
            self.total_bytes_out as f64 / self.total_bytes_in as f64
        }
    }

    /// Calculate space savings percentage
    pub fn space_savings(&self) -> f64 {
        if self.total_bytes_in == 0 {
            0.0
        } else {
            (1.0 - self.compression_ratio()) * 100.0
        }
    }

    /// Calculate compression rate
    pub fn compression_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.compressed_requests as f64 / self.total_requests as f64
        }
    }
}

/// Request decompression middleware
pub async fn decompression_middleware(
    request: Request,
    next: Next,
) -> Result<Response, CompressionError> {
    let (mut parts, body) = request.into_parts();

    // Check if request body is compressed
    let encoding = parts.headers.get(header::CONTENT_ENCODING);

    let new_body = if let Some(encoding_value) = encoding {
        match encoding_value.to_str() {
            Ok("gzip") => {
                debug!("Decompressing gzip request body");
                let body_bytes = axum::body::to_bytes(body, usize::MAX).await
                    .map_err(|e| CompressionError::DecompressionFailed(e.to_string()))?;

                let decompressed = decompress_gzip(&body_bytes)?;
                parts.headers.remove(header::CONTENT_ENCODING);
                parts.headers.insert(
                    header::CONTENT_LENGTH,
                    HeaderValue::from_str(&decompressed.len().to_string()).unwrap(),
                );

                Body::from(decompressed)
            }
            Ok("br") => {
                debug!("Decompressing brotli request body");
                let body_bytes = axum::body::to_bytes(body, usize::MAX).await
                    .map_err(|e| CompressionError::DecompressionFailed(e.to_string()))?;

                let decompressed = decompress_brotli(&body_bytes)?;
                parts.headers.remove(header::CONTENT_ENCODING);
                parts.headers.insert(
                    header::CONTENT_LENGTH,
                    HeaderValue::from_str(&decompressed.len().to_string()).unwrap(),
                );

                Body::from(decompressed)
            }
            Ok("deflate") => {
                debug!("Decompressing deflate request body");
                let body_bytes = axum::body::to_bytes(body, usize::MAX).await
                    .map_err(|e| CompressionError::DecompressionFailed(e.to_string()))?;

                let decompressed = decompress_deflate(&body_bytes)?;
                parts.headers.remove(header::CONTENT_ENCODING);
                parts.headers.insert(
                    header::CONTENT_LENGTH,
                    HeaderValue::from_str(&decompressed.len().to_string()).unwrap(),
                );

                Body::from(decompressed)
            }
            _ => {
                warn!("Unsupported content encoding: {:?}", encoding_value);
                body
            }
        }
    } else {
        body
    };

    let request = Request::from_parts(parts, new_body);
    Ok(next.run(request).await)
}

/// Create compression layer with smart predicate
pub fn create_compression_layer(config: CompressionConfig) -> CompressionLayer<SmartCompressionPredicate> {
    let predicate = SmartCompressionPredicate::new(config.clone());

    let mut layer = CompressionLayer::new()
        .quality(config.level)
        .compress_when(predicate);

    // Configure compression algorithms
    if !config.enable_gzip {
        layer = layer.no_gzip();
    }
    if !config.enable_br {
        layer = layer.no_br();
    }
    if !config.enable_deflate {
        layer = layer.no_deflate();
    }

    layer
}

/// Manual compression utilities
pub mod manual {
    use super::*;
    use brotli::enc::{BrotliEncoderParams, BrotliEncoderMode};

    /// Compress data using gzip
    pub fn compress_gzip(data: &[u8], level: u32) -> Result<Vec<u8>, CompressionError> {
        let mut encoder = GzEncoder::new(Vec::new(), GzCompression::new(level));
        encoder.write_all(data)
            .map_err(|e| CompressionError::CompressionFailed(e.to_string()))?;
        encoder.finish()
            .map_err(|e| CompressionError::CompressionFailed(e.to_string()))
    }

    /// Compress data using brotli
    pub fn compress_brotli(data: &[u8], quality: u32) -> Result<Vec<u8>, CompressionError> {
        let mut params = BrotliEncoderParams::default();
        params.quality = quality as i32;
        params.mode = BrotliEncoderMode::BROTLI_MODE_GENERIC;

        let mut output = Vec::new();
        brotli::BrotliCompress(data, &mut output, &params)
            .map_err(|e| CompressionError::CompressionFailed(format!("Brotli compression failed: {:?}", e)))?;

        Ok(output)
    }

    /// Compress data using deflate
    pub fn compress_deflate(data: &[u8], level: u32) -> Result<Vec<u8>, CompressionError> {
        let mut encoder = flate2::write::DeflateEncoder::new(Vec::new(), GzCompression::new(level));
        encoder.write_all(data)
            .map_err(|e| CompressionError::CompressionFailed(e.to_string()))?;
        encoder.finish()
            .map_err(|e| CompressionError::CompressionFailed(e.to_string()))
    }

    /// Auto-select best compression algorithm and level
    pub fn compress_auto(data: &[u8], target_ratio: f64) -> Result<(Vec<u8>, String), CompressionError> {
        let original_size = data.len();

        // Try different compression methods
        let mut best_result = (data.to_vec(), "none".to_string());
        let mut best_ratio = 1.0;

        // Try gzip with different levels
        for level in [1, 6, 9] {
            if let Ok(compressed) = compress_gzip(data, level) {
                let ratio = compressed.len() as f64 / original_size as f64;
                if ratio < best_ratio && ratio <= target_ratio {
                    best_result = (compressed, "gzip".to_string());
                    best_ratio = ratio;
                }
            }
        }

        // Try brotli with different qualities
        for quality in [1, 6, 11] {
            if let Ok(compressed) = compress_brotli(data, quality) {
                let ratio = compressed.len() as f64 / original_size as f64;
                if ratio < best_ratio && ratio <= target_ratio {
                    best_result = (compressed, "br".to_string());
                    best_ratio = ratio;
                }
            }
        }

        debug!("Best compression: {} (ratio: {:.2})", best_result.1, best_ratio);
        Ok(best_result)
    }
}

/// Decompression utilities
pub fn decompress_gzip(data: &[u8]) -> Result<Vec<u8>, CompressionError> {
    use flate2::read::GzDecoder;
    use std::io::Read;

    let mut decoder = GzDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)
        .map_err(|e| CompressionError::DecompressionFailed(e.to_string()))?;

    Ok(decompressed)
}

pub fn decompress_brotli(data: &[u8]) -> Result<Vec<u8>, CompressionError> {
    let mut output = Vec::new();
    brotli::BrotliDecompress(data, &mut output)
        .map_err(|e| CompressionError::DecompressionFailed(format!("Brotli decompression failed: {:?}", e)))?;

    Ok(output)
}

pub fn decompress_deflate(data: &[u8]) -> Result<Vec<u8>, CompressionError> {
    use flate2::read::DeflateDecoder;
    use std::io::Read;

    let mut decoder = DeflateDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)
        .map_err(|e| CompressionError::DecompressionFailed(e.to_string()))?;

    Ok(decompressed)
}

/// Response size estimation for compression decisions
pub fn estimate_compression_benefit(content_type: &str, size: usize) -> f64 {
    match content_type {
        ct if ct.starts_with("text/html") => 0.3, // HTML compresses well
        ct if ct.starts_with("text/css") => 0.25, // CSS compresses very well
        ct if ct.starts_with("text/javascript") => 0.35, // JS compresses well
        ct if ct.starts_with("application/json") => 0.4, // JSON compresses well
        ct if ct.starts_with("application/xml") => 0.3, // XML compresses well
        ct if ct.starts_with("text/plain") => 0.5, // Text varies
        ct if ct.starts_with("image/svg+xml") => 0.2, // SVG compresses very well
        _ => 0.8, // Conservative estimate for unknown types
    }
}

/// Adaptive compression that adjusts based on performance
pub struct AdaptiveCompression {
    stats: CompressionStats,
    config: CompressionConfig,
    performance_threshold_ms: u128,
}

impl AdaptiveCompression {
    pub fn new(config: CompressionConfig, performance_threshold_ms: u128) -> Self {
        Self {
            stats: CompressionStats::default(),
            config,
            performance_threshold_ms,
        }
    }

    /// Decide whether to compress based on current performance metrics
    pub fn should_compress(&self, content_type: &str, size: usize) -> bool {
        // Base decision on content type and size
        let base_decision = SmartCompressionPredicate::new(self.config.clone())
            .should_compress(&http::Response::builder()
                .header(header::CONTENT_TYPE, content_type)
                .header(header::CONTENT_LENGTH, size.to_string())
                .body(axum::body::Body::empty())
                .unwrap());

        if !base_decision {
            return false;
        }

        // Adaptive logic: reduce compression under high load
        let compression_rate = self.stats.compression_rate();
        let space_savings = self.stats.space_savings();

        // If compression rate is low or savings are minimal, be more selective
        if compression_rate < 0.3 || space_savings < 10.0 {
            return size > self.config.min_size * 2;
        }

        true
    }

    /// Update performance statistics
    pub fn record_compression(&mut self,
                            original_size: usize,
                            compressed_size: usize,
                            algorithm: &str,
                            duration_ms: u128) {
        self.stats.total_requests += 1;
        self.stats.compressed_requests += 1;
        self.stats.total_bytes_in += original_size as u64;
        self.stats.total_bytes_out += compressed_size as u64;

        match algorithm {
            "gzip" => self.stats.gzip_requests += 1,
            "br" => self.stats.brotli_requests += 1,
            "deflate" => self.stats.deflate_requests += 1,
            _ => {}
        }

        // Adjust compression aggressiveness based on performance
        if duration_ms > self.performance_threshold_ms {
            warn!("Compression took {}ms, consider reducing compression level", duration_ms);
        }
    }

    pub fn get_stats(&self) -> &CompressionStats {
        &self.stats
    }
}

/// Compression error types
#[derive(Debug, thiserror::Error)]
pub enum CompressionError {
    #[error("Compression failed: {0}")]
    CompressionFailed(String),

    #[error("Decompression failed: {0}")]
    DecompressionFailed(String),

    #[error("Unsupported compression algorithm: {0}")]
    UnsupportedAlgorithm(String),

    #[error("Invalid compression level: {0}")]
    InvalidLevel(u32),

    #[error("Compression ratio too high: {0}")]
    RatioTooHigh(f64),
}

impl IntoResponse for CompressionError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            CompressionError::UnsupportedAlgorithm(_) | CompressionError::InvalidLevel(_) => {
                (StatusCode::BAD_REQUEST, self.to_string())
            }
            CompressionError::DecompressionFailed(_) => {
                (StatusCode::BAD_REQUEST, "Invalid compressed data".to_string())
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Compression error".to_string()),
        };

        let body = serde_json::json!({
            "error": {
                "code": "COMPRESSION_ERROR",
                "message": message
            }
        });

        (status, axum::Json(body)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_config_default() {
        let config = CompressionConfig::default();
        assert!(config.enable_gzip);
        assert!(config.enable_br);
        assert_eq!(config.min_size, 1024);
    }

    #[test]
    fn test_compression_stats() {
        let mut stats = CompressionStats::default();
        stats.total_requests = 100;
        stats.compressed_requests = 80;
        stats.total_bytes_in = 10000;
        stats.total_bytes_out = 3000;

        assert_eq!(stats.compression_rate(), 0.8);
        assert_eq!(stats.compression_ratio(), 0.3);
        assert_eq!(stats.space_savings(), 70.0);
    }

    #[tokio::test]
    async fn test_manual_compression() {
        let data = b"Hello, world! This is a test string for compression.";

        let gzip_result = manual::compress_gzip(data, 6).unwrap();
        let decompressed = decompress_gzip(&gzip_result).unwrap();
        assert_eq!(data, &decompressed[..]);

        let brotli_result = manual::compress_brotli(data, 6).unwrap();
        let decompressed = decompress_brotli(&brotli_result).unwrap();
        assert_eq!(data, &decompressed[..]);
    }

    #[test]
    fn test_estimate_compression_benefit() {
        assert!(estimate_compression_benefit("text/html", 1000) < 0.5);
        assert!(estimate_compression_benefit("application/json", 1000) < 0.5);
        assert!(estimate_compression_benefit("image/jpeg", 1000) > 0.5);
    }
}
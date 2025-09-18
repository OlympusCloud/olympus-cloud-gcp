# Build stage
FROM rust:1.75-slim AS builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /usr/src/olympus

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY auth ./auth
COPY platform ./platform
COPY commerce ./commerce
COPY shared ./shared
COPY src ./src
COPY migrations ./migrations

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1001 olympus

# Copy the binary from builder
COPY --from=builder /usr/src/olympus/target/release/olympus-rust /usr/local/bin/olympus-rust

# Copy migrations
COPY --from=builder /usr/src/olympus/migrations /app/migrations

# Set working directory
WORKDIR /app

# Change ownership
RUN chown -R olympus:olympus /app

# Switch to non-root user
USER olympus

# Environment variables
ENV PORT=8000
ENV RUST_LOG=info

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8000/health || exit 1

# Expose port
EXPOSE 8000

# Run the binary
CMD ["olympus-rust"]
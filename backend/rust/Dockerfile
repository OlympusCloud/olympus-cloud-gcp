 # --- Builder Stage ---
 # Use the official Rust image as a builder
 FROM rust:1.72-slim-bullseye as builder
 
 # Create a new empty shell project
 WORKDIR /usr/src/app
 
 # Copy over your manifests
 COPY ./Cargo.toml ./Cargo.toml
 COPY ./auth/Cargo.toml ./auth/Cargo.toml
 # COPY ./platform/Cargo.toml ./platform/Cargo.toml
 # COPY ./commerce/Cargo.toml ./commerce/Cargo.toml
 # COPY ./shared/Cargo.toml ./shared/Cargo.toml
 
 # Create dummy source files to cache dependencies
 RUN mkdir -p auth/src && echo "fn main() {}" > auth/src/main.rs
 # RUN mkdir -p platform/src && echo "fn main() {}" > platform/src/main.rs
 # RUN mkdir -p commerce/src && echo "fn main() {}" > commerce/src/main.rs
 # RUN mkdir -p shared/src && echo "fn main() {}" > shared/src/main.rs
 
 # Build only the dependencies to cache them
 RUN cargo build --release
 
 # Copy the actual source code
 COPY ./auth/src ./auth/src
 # COPY ./platform/src ./platform/src
 # COPY ./commerce/src ./commerce/src
 # COPY ./shared/src ./shared/src
 
 # Build the release binary
 RUN cargo build --release
 
 # --- Final Stage ---
 # Use a minimal, non-root base image for security
 FROM gcr.io/distroless/cc-debian11
 
 # Copy the compiled binary from the builder stage
 COPY --from=builder /usr/src/app/target/release/olympus-auth /usr/local/bin/olympus-auth
 
 # Set the entrypoint
 EXPOSE 8080
 ENV ROCKET_ADDRESS=0.0.0.0
 CMD ["/usr/local/bin/olympus-auth"]
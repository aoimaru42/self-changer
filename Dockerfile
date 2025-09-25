# Multi-stage build for Rust application
FROM rust:latest as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-leptos
RUN cargo install cargo-leptos@0.2.44 --locked

# Add WebAssembly target
RUN rustup target add wasm32-unknown-unknown

# Install nightly toolchain with rust-src component
RUN rustup toolchain install nightly --component rust-src
RUN rustup target add wasm32-unknown-unknown --toolchain nightly

# Set nightly as default toolchain
RUN rustup default nightly

# Enable adt_const_params feature for tachys compatibility
ENV RUSTFLAGS="-C target-feature=+crt-static"

# Set working directory
WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY rust-toolchain.toml ./

# Copy source code
COPY app/ ./app/
COPY server/ ./server/
COPY frontend/ ./frontend/
COPY common/ ./common/
COPY style/ ./style/
COPY public/ ./public/

# Build the application with adt_const_params feature
RUN cargo leptos build --release --features ssr,adt_const_params

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false appuser

# Set working directory
WORKDIR /app

# Copy built application from builder stage
COPY --from=builder /app/target/server/release/server /app/server
COPY --from=builder /app/target/site /app/site

# Set ownership
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 3000

# Set environment variables
ENV LEPTOS_OUTPUT_NAME="self-changer"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_RELOAD_PORT="3001"

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/ || exit 1

# Run the application
CMD ["./server"]

FROM rust:1.85 as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo files for dependency caching
COPY Cargo.toml Cargo.lock ./
COPY sui-squard-core/Cargo.toml ./sui-squard-core/
COPY sui-squard-server/Cargo.toml ./sui-squard-server/
COPY sui-squard-bot/Cargo.toml ./sui-squard-bot/

# Create dummy source files to cache dependencies
RUN mkdir -p sui-squard-core/src sui-squard-server/src sui-squard-bot/src && \
    echo "fn main() {}" > sui-squard-core/src/lib.rs && \
    echo "fn main() {}" > sui-squard-server/src/main.rs && \
    echo "fn main() {}" > sui-squard-bot/src/main.rs

# Build dependencies
RUN cargo build --release --bin sui-squard-server

# Copy actual source code
COPY sui-squard-core ./sui-squard-core
COPY sui-squard-server ./sui-squard-server
COPY sui-squard-bot ./sui-squard-bot

# Force rebuild of our code
RUN touch sui-squard-core/src/lib.rs sui-squard-server/src/main.rs sui-squard-bot/src/main.rs

# Build the application
RUN cargo build --release --bin sui-squard-server

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1000 app

# Set working directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/sui-squard-server /app/

# Create directories for volumes
RUN mkdir -p /app/keystore /app/data && \
    chown -R app:app /app

# Switch to app user
USER app

# Expose port
EXPOSE 3200

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://0.0.0.0:3200/ || exit 1

# Run the binary
CMD ["./sui-squard-server"] 
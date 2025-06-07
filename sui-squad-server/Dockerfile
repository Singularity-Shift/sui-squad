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
COPY sui-squad-core/Cargo.toml ./sui-squad-core/
COPY sui-squad-server/Cargo.toml ./sui-squad-server/
COPY sui-squad-bot/Cargo.toml ./sui-squad-bot/

# Create dummy source files to cache dependencies
RUN mkdir -p sui-squad-core/src sui-squad-server/src sui-squad-bot/src && \
    echo "fn main() {}" > sui-squad-core/src/lib.rs && \
    echo "fn main() {}" > sui-squad-server/src/main.rs && \
    echo "fn main() {}" > sui-squad-bot/src/main.rs

# Build dependencies
RUN cargo build --release --bin sui-squad-server

# Copy actual source code
COPY sui-squad-core ./sui-squad-core
COPY sui-squad-server ./sui-squad-server
COPY sui-squad-bot ./sui-squad-bot

# Force rebuild of our code
RUN touch sui-squad-core/src/lib.rs sui-squad-server/src/main.rs sui-squad-bot/src/main.rs

# Build the application
RUN cargo build --release --bin sui-squad-server

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
COPY --from=builder /app/target/release/sui-squad-server /app/

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
CMD ["./sui-squad-server"] 
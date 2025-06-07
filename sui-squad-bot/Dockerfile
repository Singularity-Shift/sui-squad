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
COPY sui-squad-bot/Cargo.toml ./sui-squad-bot/
COPY sui-squad-server/Cargo.toml ./sui-squad-server/

# Create dummy source files to cache dependencies
RUN mkdir -p sui-squad-core/src sui-squad-bot/src sui-squad-server/src && \
    echo "fn main() {}" > sui-squad-core/src/lib.rs && \
    echo "fn main() {}" > sui-squad-bot/src/main.rs && \
    echo "fn main() {}" > sui-squad-server/src/main.rs

# Build dependencies
RUN cargo build --release --bin sui-squad-bot

# Copy actual source code
COPY sui-squad-core ./sui-squad-core
COPY sui-squad-bot ./sui-squad-bot
COPY sui-squad-server/Cargo.toml ./sui-squad-server/

# Force rebuild of our code
RUN touch sui-squad-core/src/lib.rs sui-squad-bot/src/main.rs

# Build the application
RUN cargo build --release --bin sui-squad-bot

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
COPY --from=builder /app/target/release/sui-squad-bot /app/

# Create directories for volumes
RUN mkdir -p /app/keystore /app/data && \
    chown -R app:app /app

# Switch to app user
USER app

# Run the binary
CMD ["./sui-squad-bot"] 
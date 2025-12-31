# Poker Consensus Engine - Docker Build Environment
# This provides a reproducible build environment for the project

# Stage 1: Builder
FROM docker.io/rust:1.75 AS builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    pkg-config \
    libssl-dev \
    clang \
    git \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy source
COPY . .

# Build
RUN cargo build --release --bin poker-node

# Stage 2: Runtime
FROM docker.io/ubuntu:22.04 AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -m -s /bin/bash app

# Copy binary from builder
COPY --from=builder /app/target/release/poker-node /usr/local/bin/

# Set working directory
WORKDIR /home/app

# Create data directory
RUN mkdir -p /data

# Switch to non-root user
USER app

# Expose ports
EXPOSE 30333 9933 9944

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:9933/health || exit 1

# Start command
ENTRYPOINT ["poker-node"]
CMD ["--chain=dev", "--validator"]

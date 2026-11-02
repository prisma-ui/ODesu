# Multi-stage build for minimal production image
# Build stage
FROM rust:1.83-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy Cargo.toml and Cargo.lock first for better caching
COPY Cargo.toml Cargo.lock ./

# Create dummy source to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

# Copy actual source
COPY src ./src
COPY Config.toml.example ./Config.toml

# Build release binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/otakudesu-api /usr/local/bin/otakudesu-api

# Copy config template
COPY Config.toml.example ./Config.toml

# Create non-root user
RUN useradd -r -s /bin/false -u 1001 appuser && chown -R appuser:appuser /app
USER appuser

# Port (Hugging Face Spaces requires 7860, Fly.io uses PORT env var)
ENV PORT=7860
EXPOSE 7860

# Run
CMD ["otakudesu-api"]
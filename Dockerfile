# Build stage
FROM rust:1.86 as builder
RUN apt-get update && apt-get install -y \
    libpq-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /app

# Copy manifests (use lockfile for reproducible builds and caching)
COPY Cargo.toml Cargo.lock ./


# Copy source
COPY src ./src
COPY migrations ./migrations

# Build release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install PostgreSQL client (psql) and SSL libraries
RUN apt-get update && apt-get install -y \
    libpq5 \
    postgresql-client \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/rust_diesel_api /app/rust_diesel_api

# Copy diesel CLI to run migrations in runtime
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# Copy migrations
COPY migrations ./migrations

# Copy seeds
COPY seeds ./seeds

# Expose port
EXPOSE 3030

# Run the binary
CMD ["/app/rust_diesel_api"]

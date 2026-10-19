# ---- Build Stage ----
FROM rust:1.99.1 AS builder

WORKDIR /app

# Copy manifests first for better layer caching
COPY Cargo.toml Cargo.lock ./
COPY migration/Cargo.toml migration/Cargo.toml

# Copy actual source code
COPY . .

# Build the release binary
RUN cargo build --release --bin check

# ---- Runtime Stage ----
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy only the compiled binary from the builder stage
COPY --from=builder /app/target/release/check .

EXPOSE 3000

CMD ["./check"]
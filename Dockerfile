# ---- Builder ----
FROM rust:alpine AS builder

RUN apk add --no-cache \
    build-base \
    openssl-dev \
    openssl-libs-static \
    pkgconfig \
    sqlite-dev \
    ca-certificates \
    tzdata

WORKDIR /app

# Copy manifests first for better Docker layer caching
COPY Cargo.toml Cargo.lock ./

# Copy sources and migrations
COPY src ./src
COPY migrations ./migrations

# Build release (musl)
RUN cargo build --release

# ---- Runtime ----
FROM alpine:3.20 AS runtime

RUN apk add --no-cache \
    ca-certificates \
    tzdata \
    openssl \
    sqlite-libs

# Non-root user
RUN adduser -D -H appuser

WORKDIR /app

# Copy the compiled binary and migrations
COPY --from=builder /app/target/release/investec-transaction-buckets /usr/local/bin/investec-transaction-buckets
COPY --from=builder /app/migrations /app/migrations

# App data directory (for SQLite)
RUN mkdir -p /app/data \
    && chown -R appuser:appuser /app \
    && chown appuser:appuser /usr/local/bin/investec-transaction-buckets

USER appuser

ENV RUST_LOG=info

CMD ["/usr/local/bin/investec-transaction-buckets"]

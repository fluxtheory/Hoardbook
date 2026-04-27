# ── Build stage ──────────────────────────────────────────────────────────────
FROM rust:1-bookworm AS builder

WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/

RUN cargo build --release -p hb-relay

# ── Runtime stage ─────────────────────────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        libsqlite3-0 \
        ca-certificates \
        curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /build/target/release/hb-relay ./

RUN mkdir /data

ENV DATABASE_URL=sqlite:///data/hb-relay.db \
    BIND_ADDR=0.0.0.0:3000

EXPOSE 3000

VOLUME /data

CMD ["/app/hb-relay"]

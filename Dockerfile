# Stage 1: Build the Rust Application

FROM rust:1.99-slim AS builder

RUN apt-get update && apt-get install -y --no-install-recommends curl && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY crates ./crates 

COPY src ./src
COPY migrations ./migrations

RUN cargo build --release


# Stage 2: Runtime Image

FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/Relay /usr/local/bin/Relay

EXPOSE 3000

CMD ["Relay"]
FROM rust:1.82-slim AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml ./

COPY src/ ./src/

RUN cargo build --release --bin string-analyser

FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/target/release/string-analyser /app

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

EXPOSE 8000

ENV RUST_LOG=info

ENTRYPOINT ["/app"]
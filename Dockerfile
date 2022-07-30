FROM rust:1.62.1 as builder

WORKDIR /bot

RUN apt-get update && apt-get install -y cmake && apt-get clean

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "// dummy file" > src/lib.rs && \
    cargo build --release && \
    rm -r src

COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && apt-get upgrade -y && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /bot/target/release/anti_ghost_ping /usr/local/bin/anti_ghost_ping

CMD ["/usr/local/bin/anti_ghost_ping"]
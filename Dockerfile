FROM rust:bookworm AS builder

WORKDIR /app

COPY . .

ENV SQLX_OFFLINE=true

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/rs-rinha /usr/bin/

ENTRYPOINT ["rs-rinha"]

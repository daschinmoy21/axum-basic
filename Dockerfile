FROM rust:1.85 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder /app/target/release/basic /usr/local/bin/app

EXPOSE 4000

CMD ["app"]

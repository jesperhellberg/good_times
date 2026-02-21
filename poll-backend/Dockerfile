FROM rust:1.93.1 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/poll-backend /app/poll-backend
COPY migrations /app/migrations
EXPOSE 3000
CMD ["/app/poll-backend"]

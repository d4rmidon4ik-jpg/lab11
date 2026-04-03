# Этап сборки
FROM rust:1.75 AS builder

WORKDIR /app

# Кэшируем зависимости — собираем пустой main, потом реальный код
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Теперь собираем реальный код
COPY src ./src
RUN touch src/main.rs && cargo build --release

# Финальный образ
FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates wget \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust-service /usr/local/bin/rust-service
EXPOSE 8081
CMD ["rust-service"]
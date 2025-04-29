FROM rust:1.81 as builder

WORKDIR /usr/src/app
COPY Cargo.toml ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim

# Установка необходимых библиотек
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/app/target/release/rust_web_tutorial .

EXPOSE 8080

CMD ["./rust_web_tutorial"] 
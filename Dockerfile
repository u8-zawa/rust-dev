# ビルドステージ
FROM rust:1.76-slim-bookworm AS builder
WORKDIR /app
ARG BIN_NAME=hello_world
COPY . .
RUN cargo build --release --bin ${BIN_NAME}

# 実行ステージ
FROM debian:bookworm-slim
ARG BIN_NAME=hello_world
WORKDIR /app
COPY --from=builder /app/target/release/${BIN_NAME} ./app
CMD ["./app"]

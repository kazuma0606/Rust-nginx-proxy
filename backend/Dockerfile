FROM rustlang/rust:nightly as builder

WORKDIR /app

# .lockファイルを無視してビルド
COPY Cargo.toml .
# Cargo.lockをコピーしない

# ソースコードをコピー
COPY src ./src

# ビルドの実行
RUN cargo build --release

# 実行イメージ
FROM debian:bookworm-slim

# 必要なランタイム依存関係
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# ビルドされたバイナリをコピー
COPY --from=builder /app/target/release/backend /app/

# バックエンドサーバーを起動
CMD ["./backend"]
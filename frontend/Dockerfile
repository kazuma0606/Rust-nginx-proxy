FROM rustlang/rust:nightly as builder

WORKDIR /app

# Trunkとwasm-bindgenをインストール
RUN rustup target add wasm32-unknown-unknown && \
    cargo install trunk && \
    cargo install wasm-bindgen-cli

# コードをコピー
COPY . .

# ビルド実行
RUN trunk build --release

# 最終イメージ - シンプルなnginxを使用
FROM nginx:alpine

# ビルドしたファイルをnginxのhtmlディレクトリにコピー
COPY --from=builder /app/dist /usr/share/nginx/html
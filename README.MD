# Leptos + Axum + NGINX リバースプロキシ構成

このプロジェクトは、Leptos (Rust/WASM) フロントエンド、Axum バックエンド、NGINX リバースプロキシを使用した Web アプリケーションの構築方法を示しています。Docker Compose を使用して、フロントエンド、バックエンド、NGINX の3つのコンテナを調整します。

## 考慮したポイント

1. **CORS 問題の解決**
   - NGINX リバースプロキシを使用して、フロントエンドとバックエンドを同一オリジンで提供
   - フロントエンドからのAPIリクエストを NGINX を経由して Axum バックエンドに転送

2. **Docker コンテナの分離**
   - フロントエンド: Leptos/WASM をビルドして NGINX コンテナに配置
   - バックエンド: Axum サーバーを実行
   - リバースプロキシ: フロントエンドの配信と API リクエストの転送

3. **ビルドプロセスの効率化**
   - マルチステージビルドを使用して、ビルド環境と実行環境を分離
   - 依存関係のキャッシュによりビルド時間を短縮

4. **デプロイフロー**
   - ボリュームマウントとコンテナ間のファイル共有の注意点
   - 開発モードと本番モードの切り替え

## システム構成

```
┌─────────────────┐      ┌─────────────────┐
│                 │      │                 │
│  ブラウザ       │ ◄──► │  NGINX          │
│  (ユーザー)     │      │  (リバースプロキシ) │
│                 │      │                 │
└─────────────────┘      └───────┬─────────┘
                                │
                                │
            ┌──────────────────┴───────────────────┐
            │                                      │
┌───────────▼─────────────┐        ┌───────────────▼───────────┐
│                         │        │                           │
│  Leptos                 │        │  Axum                     │
│  (フロントエンド - WASM) │        │  (バックエンド API サーバー) │
│                         │        │                           │
└─────────────────────────┘        └───────────────────────────┘
```

## 起動方法

### 前提条件

- Docker と Docker Compose がインストールされていること
- Rust ツールチェーン (開発時のみ必要)

### 初期起動手順

1. リポジトリのクローン:
   ```bash
   git clone <repository-url>
   cd <repository-directory>
   ```

2. Docker Compose でビルド・起動:
   ```bash
   docker-compose build
   docker-compose up
   ```

3. フロントエンドコンテナからNGINXコンテナにファイルをコピー:
   ```bash
   # ホストに一時ディレクトリを作成
   mkdir -p temp_html
   
   # フロントエンドコンテナからホストにファイルをコピー
   docker cp leptos_frontend:/usr/share/nginx/html/. ./temp_html/
   
   # ホストからNGINXコンテナにファイルをコピー
   docker cp ./temp_html/. nginx_gateway:/usr/share/nginx/html/
   
   # 一時ディレクトリを削除 (オプション)
   rm -rf temp_html
   ```

4. ブラウザで `http://localhost` にアクセス

### 開発モードでの起動 (ホットリロード)

```bash
# バックエンドを開発モードで起動
cd backend
cargo watch -x run

# フロントエンドを開発モードで起動
cd frontend
trunk serve --proxy-backend=http://localhost:4000/api
```

## プロジェクト構造

```
project-root/
├── backend/
│   ├── Cargo.toml
│   ├── Dockerfile
│   └── src/
│       └── main.rs
├── frontend/
│   ├── Cargo.toml
│   ├── Dockerfile
│   ├── index.html
│   └── src/
│       └── main.rs
├── nginx/
│   └── default.conf
├── docker-compose.yml
└── README.md
```

## ファイル説明

### `docker-compose.yml`

```yaml
version: "3.9"
services:
  frontend:
    build:
      context: ./frontend
      dockerfile: Dockerfile
    container_name: leptos_frontend
    restart: always
    depends_on:
      - backend

  backend:
    build:
      context: ./backend
      dockerfile: Dockerfile
    container_name: rust_backend
    restart: always
    command: ./backend
    environment:
      - RUST_LOG=info
    ports:
      - "4000:4000"

  nginx:
    image: nginx:alpine
    container_name: nginx_gateway
    restart: always
    ports:
      - "80:80"
    volumes:
      - ./nginx/default.conf:/etc/nginx/conf.d/default.conf
    depends_on:
      - frontend
      - backend
```

### `nginx/default.conf`

```nginx
server {
    listen 80;

    location / {
        root /usr/share/nginx/html;
        try_files $uri $uri/ /index.html;
        index index.html;
    }

    location /api/ {
        proxy_pass http://backend:4000/;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # CORS ヘッダーを設定
        add_header 'Access-Control-Allow-Origin' '*' always;
        add_header 'Access-Control-Allow-Methods' 'GET, POST, OPTIONS, PUT, DELETE' always;
        add_header 'Access-Control-Allow-Headers' 'DNT,User-Agent,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Range,Authorization' always;
        
        # OPTIONS リクエスト対応
        if ($request_method = 'OPTIONS') {
            add_header 'Access-Control-Allow-Origin' '*';
            add_header 'Access-Control-Allow-Methods' 'GET, POST, OPTIONS, PUT, DELETE';
            add_header 'Access-Control-Allow-Headers' 'DNT,User-Agent,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Range,Authorization';
            add_header 'Access-Control-Max-Age' 1728000;
            add_header 'Content-Type' 'text/plain; charset=utf-8';
            add_header 'Content-Length' 0;
            return 204;
        }
    }
}
```

## 重要な考慮事項とトラブルシューティング

### ボリュームマウントに関する注意点

現在の設定では、フロントエンドコンテナでビルドされたファイルをNGINXコンテナに手動でコピーする必要があります。これを自動化する方法として以下のオプションが考えられます：

1. **カスタムスクリプトの作成**:
   ```bash
   #!/bin/bash
   docker-compose up -d
   mkdir -p temp_html
   docker cp leptos_frontend:/usr/share/nginx/html/. ./temp_html/
   docker cp ./temp_html/. nginx_gateway:/usr/share/nginx/html/
   rm -rf temp_html
   ```

2. **ビルドとデプロイを統合したカスタムイメージの作成**:
   ```dockerfile
   # Dockerfile.integrated
   FROM rustlang/rust:nightly as builder
   WORKDIR /app
   COPY . .
   RUN rustup target add wasm32-unknown-unknown && \
       cargo install trunk && \
       cargo install wasm-bindgen-cli && \
       trunk build --release

   FROM nginx:alpine
   COPY --from=builder /app/dist /usr/share/nginx/html
   COPY ./nginx/default.conf /etc/nginx/conf.d/default.conf
   ```

### CORS 関連の問題

CORS エラーが発生した場合：

1. NGINXの設定でCORSヘッダーが正しく設定されているか確認
2. ブラウザの開発者ツールでネットワークリクエストを検査
3. 直接バックエンドにアクセスしてAPIが機能しているか確認: `curl http://localhost:4000/data`

### フロントエンドの開発・デバッグ

フロントエンドのデバッグを容易にするため、以下のヒントが役立ちます：

1. コンソールログを活用した問題特定
2. ネットワークリクエストの成功・失敗の監視
3. 開発時は `trunk serve` を使用したホットリロードの活用

## 今後の発展形

### 1. マイクロサービスアーキテクチャへの拡張

```
                            ┌─────────────────┐
                            │                 │
                            │  NGINX          │
                            │  (API Gateway)  │
                            │                 │
                            └─────────────────┘
                                    │
        ┌───────────────────┬───────┴───────┬───────────────────┐
        │                   │               │                   │
┌───────▼───────┐   ┌───────▼───────┐   ┌───▼───────────┐   ┌───▼───────────┐
│               │   │               │   │               │   │               │
│ ユーザーサービス │   │ 認証サービス   │   │ プロダクトサービス │   │ 注文サービス   │
│               │   │               │   │               │   │               │
└───────────────┘   └───────────────┘   └───────────────┘   └───────────────┘
        │                   │                   │                   │
        └───────────────────┴───────────────────┴───────────────────┘
                                    │
                            ┌───────▼───────┐
                            │               │
                            │  データベース   │
                            │               │
                            └───────────────┘
```

- 機能単位でマイクロサービスに分割
- NGINXをAPIゲートウェイとして活用
- サービス間通信には gRPC や JSON-RPC を使用

### 2. スケーラビリティの向上

- Kubernetes でのオーケストレーション
- 水平スケーリングでの負荷分散
- データベースの読み書き分離とシャーディング

### 3. 非同期処理パイプラインの導入

- メッセージキュー (RabbitMQ, Kafka) を使用した非同期処理
- バッチ処理やバックグラウンドタスクの実装
- イベント駆動型アーキテクチャの採用

### 4. セキュリティ強化

- JWT を使用した認証
- API レート制限の実装
- HTTPS の導入と証明書の自動更新

### 5. 開発体験の向上

- Docker 開発環境の最適化
- ホットリロードとライブリロードの統合
- テスト自動化パイプラインの構築

## ライセンス

MIT

## 貢献

貢献は大歓迎です。プルリクエストを送信する前に、イシューで変更内容について議論してください。

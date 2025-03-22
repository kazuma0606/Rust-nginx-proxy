すみません、コンテナ間の直接コピーがサポートされていないようです。その場合、中間ステップとしてホストを経由する必要があります。

以下の手順で実行してみてください：

```bash
# 1. まずフロントエンドコンテナからホストにファイルをコピー
docker cp leptos_frontend:/usr/share/nginx/html/. ./temp_html/

# 2. ホストからNGINXコンテナにファイルをコピー
docker cp ./temp_html/. nginx_gateway:/usr/share/nginx/html/
```

ただし、より永続的な解決策として、以下のいずれかを検討してください：

### 1. カスタムイメージを作成

プロジェクトのルートディレクトリに新しいDockerfileを作成します：

```dockerfile
# Dockerfile.nginx
FROM nginx:alpine
COPY ./nginx/default.conf /etc/nginx/conf.d/default.conf
```

そして、Docker Composeファイルを修正します：

```yaml
services:
  nginx:
    build:
      context: .
      dockerfile: Dockerfile.nginx
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

コンテナが起動したら、上記の方法でファイルをコピーします。

### 2. ボリュームマウントを完全に削除

Docker Composeファイルからボリュームマウントを完全に削除し、フロントエンドコンテナのDockerfileを修正して、ビルド結果をNGINXのデフォルトHTMLディレクトリに直接コピーするようにします。

しかし、短期的には上記の`docker cp`コマンドを使用してファイルをコピーする方法が最も簡単です。
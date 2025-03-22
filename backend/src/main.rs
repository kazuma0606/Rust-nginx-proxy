use axum::{
    routing::get,
    response::Json,
    Router,
};
use serde_json::json;
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    // サーバー起動メッセージ
    println!("Starting Axum backend server...");

    // CORS設定
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // API ルーター
    let app = Router::new()
        .route("/", get(get_data))
        .route("/data", get(get_data))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 4000));
    println!("Backend server running on http://0.0.0.0:4000");
    
    // サーバーを起動
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|e| {
            eprintln!("Server error: {}", e);
            panic!("Server failed to start");
        });
}

async fn get_data() -> Json<serde_json::Value> {
    Json(json!({
        "message": "Hello from Rust backend!",
        "status": "success",
    }))
}
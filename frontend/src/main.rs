use leptos::*;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

// コンソールログ用のヘルパー関数
#[cfg(target_arch = "wasm32")]
fn console_log(msg: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(msg));
}

#[cfg(not(target_arch = "wasm32"))]
fn console_log(msg: &str) {
    println!("{}", msg);
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct ApiResponse {
    message: String,
    status: String,
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (error, set_error) = create_signal(cx, None::<String>);
    
    // リソースを作成
    let data = create_resource(
        cx,
        || (), 
        |_| async {
            fetch_data().await
        }
    );
    
    // データ更新関数
    let refresh = move |_| {
        data.refetch();
    };

    view! { cx,
        <div class="container" style="max-width: 800px; margin: 0 auto; padding: 2rem; font-family: system-ui, sans-serif;">
            <h1>"Leptos + Axum via NGINX"</h1>
            
            {move || error.get().map(|err| view! { cx, 
                <div style="background-color: #f8d7da; color: #721c24; padding: 1rem; border-radius: 0.25rem; margin-bottom: 1rem;">
                    <p>{err}</p>
                </div>
            })}
            
            <button 
                on:click=refresh
                style="background-color: #4CAF50; color: white; padding: 10px 15px; border: none; border-radius: 4px; cursor: pointer;"
            >
                "Refresh Data"
            </button>
            
            <div style="margin-top: 1rem; padding: 1rem; border: 1px solid #ddd; border-radius: 0.25rem;">
                {move || match data.read(cx) {
                    None => view! { cx, <div><p>"Loading..."</p></div> },
                    Some(Ok(response)) => view! { cx,
                        <div>
                            <h3>"Response Data:"</h3>
                            <p><strong>"Message: "</strong>{&response.message}</p>
                            <p><strong>"Status: "</strong>{&response.status}</p>
                            
                        </div>
                    },
                    Some(Err(err)) => {
                        // エラーシグナルを設定
                        set_error.set(Some(err.clone()));
                        view! { cx, <div><p>"Error loading data"</p></div> }
                    }
                }}
            </div>
        </div>
    }
}

// バックエンドからデータを取得する関数
async fn fetch_data() -> Result<ApiResponse, String> {
    console_log("Fetching data from backend via NGINX proxy");
    
    // NGINX プロキシを通してアクセス（CORS を回避）
    // 注意: 絶対 URL ではなく相対 URL を使用
    match Request::get("/api/data")
        .send()
        .await {
        Ok(resp) => {
            console_log(&format!("Response status: {}", resp.status()));
            
            if resp.ok() {
                match resp.json::<ApiResponse>().await {
                    Ok(data) => {
                        console_log("Data successfully parsed");
                        Ok(data)
                    },
                    Err(e) => {
                        console_log(&format!("JSON parse error: {}", e));
                        Err(format!("Failed to parse response: {}", e))
                    }
                }
            } else {
                console_log(&format!("Non-OK response: {}", resp.status()));
                
                // レスポンステキストも表示して診断を容易に
                match resp.text().await {
                    Ok(text) => {
                        console_log(&format!("Response body: {}", text));
                        Err(format!("Error response ({}): {}", resp.status(), text))
                    },
                    Err(_) => Err(format!("Error response: {}", resp.status()))
                }
            }
        },
        Err(e) => {
            console_log(&format!("Network error: {}", e));
            Err(format!("Network error: {}", e))
        }
    }
}

fn main() {
    // パニック時のフックを設定
    _ = console_error_panic_hook::set_once();
    
    // Appコンポーネントをマウント
    mount_to_body(|cx| view! { cx, <App/> });
}
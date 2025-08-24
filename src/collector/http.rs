#[cfg(feature = "http")]
use axum::{Router, routing::get, extract::Path, Json};
use serde_json::Value;
use std::net::SocketAddr;
use std::sync::Arc;

#[cfg(feature = "http")]
pub fn start_server(port: u16, system_info: Value) {
    use tokio::runtime::Runtime;
    
    // Create a new runtime for the HTTP server
    let rt = Runtime::new().unwrap();
    
    // Wrap system_info in an Arc for thread-safe sharing
    let shared_info = Arc::new(system_info);
    
    // Build the application with routes
    let app = Router::new()
        .route("/", get(|| async { "QuickSys API Server" }))
        .route("/api/info", get(get_all_info))
        .route("/api/info/:path", get(get_info_by_path))
        .with_state(shared_info);
    
    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    rt.block_on(async {
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });
}

#[cfg(feature = "http")]
async fn get_all_info(state: axum::extract::State<Arc<Value>>) -> Json<Value> {
    Json(state.as_ref().clone())
}

#[cfg(feature = "http")]
async fn get_info_by_path(
    state: axum::extract::State<Arc<Value>>,
    Path(path): Path<String>,
) -> Json<Value> {
    let info = state.as_ref();
    
    // Split the path by dots
    let parts: Vec<&str> = path.split('.').collect();
    
    // Navigate through the JSON structure
    let mut current = info;
    for part in parts {
        if let Some(next) = current.get(part) {
            current = next;
        } else {
            // Path not found, return empty object
            return Json(serde_json::json!({}));
        }
    }
    
    Json(current.clone())
}
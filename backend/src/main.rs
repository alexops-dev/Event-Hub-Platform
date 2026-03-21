use axum::{
    routing::get,
    Json,
    Router,
};
use serde::Serialize;
use std::net::SocketAddr;

// in C: similar to struct
// in Python: similar to a dataclass or simple object shape
#[derive(Serialize)] // Rust should automatically generate code that can convert this struct into JSON.
struct HealthResponse {
    status: String, // An owned, growable heap-allocated string. struct owns its data
}

 // route handler
async fn root() -> &'static str {
    "Berlin Kids Events Backend is running"
}

// JSON handler serializes value as JSON HTTP response
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

// this is the program entry point
// Tokio should set up the async runtime
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health));

    // Later, for Docker/Kubernetes, will be changed to 0.0.0.0:3000.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    // This binds the port and starts serving requests.
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener"); // expect(...) means:
                                                // if something fails, crash with this message.

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}
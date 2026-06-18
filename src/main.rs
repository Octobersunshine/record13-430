mod models;
mod service;

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use models::{EligibilityRequest, EligibilityResponse, User};
use serde::Serialize;
use service::EligibilityService;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[derive(Serialize)]
struct ApiResponse<T> {
    code: u32,
    message: String,
    data: Option<T>,
}

async fn check_eligibility(
    State(service): State<Arc<EligibilityService>>,
    Json(request): Json<EligibilityRequest>,
) -> (StatusCode, Json<ApiResponse<EligibilityResponse>>) {
    let result = service.check_eligibility(request.user_id, request.activity_id);

    let response = ApiResponse {
        code: 0,
        message: "success".to_string(),
        data: Some(result),
    };

    (StatusCode::OK, Json(response))
}

async fn get_users(
    State(service): State<Arc<EligibilityService>>,
) -> (StatusCode, Json<ApiResponse<Vec<User>>>) {
    let users = service.users.clone();
    let response = ApiResponse {
        code: 0,
        message: "success".to_string(),
        data: Some(users),
    };
    (StatusCode::OK, Json(response))
}

async fn health() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    let service = Arc::new(EligibilityService::new());

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/eligibility/check", post(check_eligibility))
        .route("/api/users", get(get_users))
        .layer(CorsLayer::permissive())
        .with_state(service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}

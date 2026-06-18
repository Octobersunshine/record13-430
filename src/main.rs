mod models;
mod service;

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use models::{Activity, EligibilityRequest, EligibilityResponse, User};
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

#[derive(Serialize)]
struct RegisterResult {
    pub user_id: u64,
    pub activity_id: u64,
    pub total_slots: u32,
    pub registered_count: u32,
    pub remaining_slots: u32,
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

async fn register(
    State(service): State<Arc<EligibilityService>>,
    Json(request): Json<EligibilityRequest>,
) -> (StatusCode, Json<ApiResponse<RegisterResult>>) {
    match service.register_user(request.user_id, request.activity_id) {
        Ok((total_slots, registered_count, remaining_slots)) => {
            let result = RegisterResult {
                user_id: request.user_id,
                activity_id: request.activity_id,
                total_slots,
                registered_count,
                remaining_slots,
            };
            (
                StatusCode::OK,
                Json(ApiResponse {
                    code: 0,
                    message: "报名成功".to_string(),
                    data: Some(result),
                }),
            )
        }
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                code: 1,
                message: err,
                data: None,
            }),
        ),
    }
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

async fn get_activities(
    State(service): State<Arc<EligibilityService>>,
) -> (StatusCode, Json<ApiResponse<Vec<Activity>>>) {
    let activities = service.activities.clone();
    let response = ApiResponse {
        code: 0,
        message: "success".to_string(),
        data: Some(activities),
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
        .route("/api/eligibility/register", post(register))
        .route("/api/users", get(get_users))
        .route("/api/activities", get(get_activities))
        .layer(CorsLayer::permissive())
        .with_state(service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}

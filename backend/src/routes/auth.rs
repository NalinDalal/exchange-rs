use crate::{dto::{SigninRequest, SignupRequest}, AppState};
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde_json::json;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/signup", post(signup))
        .route("/signin", post(signin))
}

async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignupRequest>,
) -> impl IntoResponse {
    match state.auth_service.signup(payload).await {
        Ok(response) => (
            StatusCode::CREATED,
            Json(json!({
                "success": true,
                "data": response
            })),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": e
            })),
        ),
    }
}

async fn signin(
    State(state): State<AppState>,
    Json(payload): Json<SigninRequest>,
) -> impl IntoResponse {
    match state.auth_service.signin(payload).await {
        Ok(response) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "data": response
            })),
        ),
        Err(e) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "success": false,
                "error": e
            })),
        ),
    }
}

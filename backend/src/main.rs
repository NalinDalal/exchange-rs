mod routes;
mod services;
mod dto;

use axum::{routing::get, Router, Json};
use db::{connect, UserRepository};
use routes::auth;
use services::AuthService;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use serde_json::json;

#[derive(Clone)]
pub struct AppState {
    pub auth_service: AuthService,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env
    dotenvy::dotenv().ok();

    // Setup logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,db=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Config
    let database_url = std::env::var("DATABASE_URL")?;
    let jwt_secret = std::env::var("JWT_SECRET")?;
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    // Connect DB
    let pool = connect(&database_url).await?;

    // Create repositories
    let user_repo = UserRepository::new(pool.clone());

    // Create services
    let auth_service = AuthService::new(user_repo, jwt_secret);

    // App state
    let state = AppState { auth_service };

    // Build router
    let app = Router::new()
        .route("/", get(health_check))
        .nest("/auth", auth::routes())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Start server
    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    tracing::info!("🚀 Server running on http://{}", addr);
    
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "message": "Exchange API is running",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}


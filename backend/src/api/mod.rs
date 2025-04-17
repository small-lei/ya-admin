use axum::Router;
use tower_http::cors::{CorsLayer, Any};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

mod auth;
mod orders;

use crate::middleware::auth::AuthContext;
use orders::AppState;

pub fn api_routes() -> Router<DatabaseConnection> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .nest("/api/auth", auth::auth_routes())
        .layer(cors)
}

// Function to create an AppState router with both DB and Auth context
pub fn api_routes_with_auth(db: DatabaseConnection, auth: Arc<AuthContext>) -> Router {
    let app_state = AppState {
        db: db.clone(),
        auth,
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .nest("/api/orders", orders::orders_routes())
        .with_state(app_state)
        .layer(cors)
}
use axum::extract::Json;
use axum::routing::post;
use axum::Router;
use bcrypt::verify;
use serde::{Deserialize, Serialize};
use sea_orm::*;
use axum::extract::State;

use crate::db::entities::user;
use crate::error::{AppError, Result};
use crate::middleware::auth::generate_token;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String,
}

pub fn auth_routes() -> Router<DatabaseConnection> {
    Router::new().route("/login", post(login))
}

async fn login(
    State(db): State<DatabaseConnection>,
    Json(login_req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(login_req.username))
        .one(&db)
        .await?
        .ok_or_else(|| AppError::AuthError("Invalid username or password".to_string()))?;

    // if !verify(&login_req.password, &user.password_hash)
    //     .map_err(|err| AppError::InternalError(format!("Password verification error: {}", err)))? {
    //     return Err(AppError::AuthError("Invalid username or password".to_string()));
    // }
    if login_req.password != user.password_hash {
        return Err(AppError::AuthError("Invalid username or password".to_string()));
    }

    let token = generate_token(user.id)?;

    Ok(Json(LoginResponse { token }))
}
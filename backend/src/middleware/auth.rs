use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::config::CONFIG;
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,  // user id
    pub exp: usize,  // expiration time
}

#[derive(Clone)]
pub struct AuthContext {
    pub user_id: i32,
}

pub async fn auth_middleware<B>(
    State(state): State<Arc<AuthContext>>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| AppError::AuthError("Missing authorization header".to_string()))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::AuthError("Invalid authorization header".to_string()));
    }

    let token = &auth_header[7..];
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(CONFIG.jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ).map_err(|_| AppError::AuthError("Invalid token".to_string()))?;

    let auth_context = AuthContext {
        user_id: token_data.claims.sub,
    };

    request.extensions_mut().insert(auth_context);

    Ok(next.run(request).await)
}

pub fn generate_token(user_id: i32) -> Result<String, AppError> {
    use jsonwebtoken::{encode, EncodingKey, Header};
    use std::time::{SystemTime, UNIX_EPOCH};

    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize + 24 * 3600; // 24 hours from now

    let claims = Claims {
        sub: user_id,
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(CONFIG.jwt_secret.as_bytes()),
    )
    .map_err(|err| AppError::InternalError(format!("Token encoding error: {}", err)))
}
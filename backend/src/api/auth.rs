use axum::{Json, extract::State};
use serde::Deserialize;
use sqlx::PgPool;

use crate::service::auth_service::{self, AuthError, JwtToken};

#[derive(Deserialize)]
pub struct SignInDto {
    pub email: String,
    pub password: String,
}

pub async fn sign_in(
    State(pool): State<PgPool>,
    Json(dto): Json<SignInDto>,
) -> Result<Json<JwtToken>, AuthError> {
    let token = auth_service::sign_in(&pool, dto.email, dto.password).await?;

    Ok(Json(token))
}

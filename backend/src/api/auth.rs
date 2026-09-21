use axum::{Json, extract::State};
use sqlx::PgPool;

use crate::{
    dto::sign_in_request::SignInRequset,
    service::auth_service::{self, AuthError, JwtToken},
};

pub async fn sign_in(
    State(pool): State<PgPool>,
    Json(dto): Json<SignInRequset>,
) -> Result<Json<JwtToken>, AuthError> {
    let token = auth_service::sign_in(&pool, dto.email, dto.password).await?;

    Ok(Json(token))
}

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use serde::Serialize;
use sqlx::PgPool;

use crate::model::app_user::AppUser;

#[derive(Serialize)]
pub struct JwtToken {
    jwt: String,
}

impl JwtToken {
    pub fn new() -> Self {
        JwtToken {
            jwt: "".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum AuthError {
    UserNotFound,
    WrongPassword,
    Database(sqlx::Error),
}

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::UserNotFound => (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()),

            AuthError::WrongPassword => (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()),

            AuthError::Database(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        }
        .into_response()
    }
}

pub async fn sign_in(pool: &PgPool, email: String, password: String) -> Result<JwtToken, AuthError> {
    let result = sqlx::query_as::<_, AppUser>(
        r#"
        SELECT id, normalized_email, password_hash
        FROM app_user
        WHERE normalized_email = $1
        LIMIT 1
        "#,
    )
    .bind(email.to_uppercase())
    .fetch_optional(pool)
    .await
    .map_err(AuthError::Database)?;

    let user = match result {
        Some(user) => user,
        None => {
            return Err(AuthError::UserNotFound);
        }
    };

    let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|_| AuthError::WrongPassword)?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| AuthError::WrongPassword)?;

    Ok(JwtToken::new())
}

// pub async fn sign_in_gmail() {}

// pub async fn sign_in_mobile() { // using code sent to mobile app just like github
// }

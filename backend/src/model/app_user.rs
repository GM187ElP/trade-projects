use argon2::{Argon2, password_hash::PasswordHasher};
use serde::Deserialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Deserialize, FromRow)]
pub struct AppUser {
    pub id: String,
    pub normalized_email: String,
    pub password_hash: String,
}

impl AppUser {
    pub fn new(email: &str, password: &str) -> Self {
        let argon = Argon2::default();
        let password_hash = argon.hash_password(password.as_bytes());
        AppUser {
            id: Uuid::new_v4().to_string(),
            normalized_email: email.to_uppercase(),
            password_hash: password_hash.unwrap().to_string(),
        }
    }
}

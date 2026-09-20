use argon2::{Argon2, PasswordHasher};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn register_admin(pool:&PgPool) -> String {
    let argon = Argon2::default();

    let pass_hash = argon
        .hash_password(
            std::env::var("ADMIN_PASS")
                .expect("ADMIN_PASS must be set")
                .as_bytes(),
        )
        .unwrap()
        .to_string();

    let result = sqlx::query(
        r#"
        INSERT INTO app_user (id, normalized_email, password_hash)
        VALUES ($1, $2, $3)
        ON CONFLICT (normalized_email) DO NOTHING
        "#,
    )
    .bind(Uuid::new_v4().to_string())
    .bind("ADMIN")
    .bind(pass_hash)
    .execute(pool)
    .await;

    match result {
        Ok(query_result) => {
            if query_result.rows_affected() == 1 {
                "Admin user is created".to_string()
            } else {
                "Admin already exists".to_string()
            }
        }

        Err(e) => {
            format!("Database error: {}", e)
        }
    }
}

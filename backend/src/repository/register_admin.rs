use argon2::{Argon2, PasswordHasher};
use uuid::Uuid;

pub async fn register_admin<'a, E>(executor: E) -> Result<String, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
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
    .execute(executor)
    .await?;

    if result.rows_affected() == 1 {
        Ok("Admin user is created".to_string())
    } else {
        Ok("Admin already exists".to_string())
    }
}

use sqlx::PgPool;

use crate::model::app_user::AppUser;

pub async fn add_app_user(pool: &PgPool, user: &AppUser) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO app_user (id, normalized_email, password_hash)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(&user.id)
    .bind(&user.normalized_email)
    .bind(&user.password_hash)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

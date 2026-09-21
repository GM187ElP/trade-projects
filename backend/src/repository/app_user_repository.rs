use crate::model::app_user::AppUser;

pub async fn add_app_user<'a, E>(executor: E, user: &AppUser) -> Result<u64, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let result = sqlx::query(
        r#"
        INSERT INTO app_user (id, normalized_email, password_hash)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(&user.id)
    .bind(&user.normalized_email)
    .bind(&user.password_hash)
    .execute(executor)
    .await?;

    Ok(result.rows_affected())
}

pub async fn get_user_by_email<'a, E>(executor: E, email: &str) -> Result<AppUser, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as::<_, AppUser>(
        r#"
        SELECT id, normalized_email, password_hash
        FROM app_user
        WHERE normalized_email = $1
        LIMIT 1
        "#,
    )
    .bind(email)
    .fetch_one(executor)
    .await
}

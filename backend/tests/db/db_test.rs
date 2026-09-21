use sqlx::{PgPool, Postgres, Transaction};

pub async fn create_pool_test() -> PgPool {
    dotenvy::dotenv().ok();

    let url = std::env::var("DATABASE_URL_TEST").expect("DATABASE_URL_TEST missing");

    PgPool::connect(&url).await.unwrap()
}

pub async fn begin_transaction(pool: &PgPool) -> Transaction<'_, Postgres> {
    pool.begin().await.unwrap()
}

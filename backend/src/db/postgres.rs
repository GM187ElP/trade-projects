use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new().connect(database_url).await
}

// pub async fn test_connection() {
//     let pool = create_pool(url).await.expect("Failed to connect");

//     let result = sqlx::query("SELECT 1").execute(&pool).await;

//     match result {
//         Ok(_) => println!("Postgres is working"),
//         Err(e) => println!("Database error: {}", e),
//     }
// }

use axum::{Router, routing::post};
use backend::{
    api::{self},
    db::db::create_pool,
    repository::register_admin::register_admin,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = create_pool(&database_url)
        .await
        .expect("Could not connect to PostgreSQL");

    let admin_creation_result = register_admin(&pool).await;
    println!("{:?}", admin_creation_result);

    let app = Router::new()
        .route("/auth/sign-in", post(api::auth::sign_in))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

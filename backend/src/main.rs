use backend::db::postgres::test_connection;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    test_connection().await;
}

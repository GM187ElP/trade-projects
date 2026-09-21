use backend::{
    repository::{app_user_repository::get_user_by_email, register_admin::register_admin},
    service::normalize_email::normalize_email,
};

use crate::db::db_test::{begin_transaction, create_pool_test};

#[tokio::test]
pub async fn register_admin_test() {
    let pool = create_pool_test().await;
    let mut tx = begin_transaction(&pool).await;

    let _ = register_admin(&mut *tx).await;

    let normalized_email = normalize_email("admin".to_string());

    let user = get_user_by_email(&mut *tx, &normalized_email)
        .await
        .unwrap();

    assert_eq!(user.normalized_email, "ADMIN".to_string());

    tx.rollback().await.unwrap();
}

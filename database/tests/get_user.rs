use mayhem_db::Client;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;

#[tokio::test]
pub async fn test_get_invalid_user() {
    let db = Database::connect("sqlite::memory:").await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    let client = Client::of(db);
    let user = client
        .query
        .user
        .find_user_by_id(999)
        .await
        .unwrap()
        .unwrap();

    println!("{:?}", user);
}

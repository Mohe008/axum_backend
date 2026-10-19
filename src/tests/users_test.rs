use serde_json::json;
use sea_orm::{ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter};
use axum_test::TestServer;

use crate::entities::{prelude::*, users};


async fn setup() -> (TestServer, DatabaseConnection) {
    dotenvy::dotenv().ok();

    let test_db_url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must be set on .env");

    let db = Database::connect(&test_db_url)
        .await
        .expect("Failed to connect to test database");

    let app = crate::routes::app(db.clone());

    (TestServer::new(app), db)
}

async fn cleanup(db: &DatabaseConnection, email: &str) {
    Users::delete_many()
        .filter(users::Column::Email.eq(email))
        .exec(db)
        .await
        .ok();
}

#[tokio::test]
async fn test_create_user() {
    let (server, db) = setup().await;
    let test_email = "test@gmail.com";

    cleanup(&db, test_email).await;

    let response = server
        .post("/users")
        .json(&json!({
            "name": "Test User",
            "email": test_email,
            "password": "test1234",
            "role": "student"
        }))
        .await;

    response.assert_status_ok();

    let body: serde_json::Value = response.json();
    assert_eq!(body["name"], "Test User");
    assert_eq!(body["email"], "test@gmail.com");
}

#[tokio::test]
async fn test_get_users() {
    let (server, _db) = setup().await;

    let response = server.get("/users").await;
    response.assert_status_ok();
}

#[tokio::test]
async fn test_get_nonexistent_user() {
    let (server, _db) = setup().await;
    let response = server.get("/users/999999").await;

    response.assert_status_not_found();
}
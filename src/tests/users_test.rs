use serde_json::json;
use sea_orm::Database;
use axum_test::TestServer;
use crate::routes;

async fn setup() -> TestServer {
    dotenvy::dotenv().ok();

    let test_db_url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must be set on .env");

    let db = Database::connect(&test_db_url)
        .await
        .expect("Failed to connect to test database");

    let app = routes::app(db);

    TestServer::new(app)
}

#[tokio::test]
async fn test_create_user() {
    let server = setup().await;

    let response = server
        .post("/users")
        .json(&json!({
            "name": "Test User",
            "email": "test@gmail.com",
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
    let server = setup().await;

    let response = server.get("/users").await;
    response.assert_status_ok();
}

#[tokio::test]
async fn test_get_nonexistent_user() {
    let server = setup().await;
}
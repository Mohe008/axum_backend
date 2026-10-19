use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn connect() -> DatabaseConnection {
    let database_url = env::var("DATABASE_URL").expect("Database url must be set in .env");

    Database::connect(&database_url)
        .await
        .expect("Could not connect to database")
}

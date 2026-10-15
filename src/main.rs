use dotenvy::dotenv;
use tracing::info;


#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .compact()
        .init();

    let db = check::db::connect().await;

    info!("Connected to PostgreSQL via SeaORM");

    let app = check::routes::app(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");

    info!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

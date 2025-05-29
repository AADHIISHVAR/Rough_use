use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub async fn connect_to_db() -> sqlx::PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}

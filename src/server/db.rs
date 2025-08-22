use std::env;

use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing::info;

pub async fn init_db_pool() -> PgPool {
    // read the DATABASE_UTL environment variable
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Configure and establish a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    info!("Connected to PostgreSQL at {}", database_url);

    pool
}

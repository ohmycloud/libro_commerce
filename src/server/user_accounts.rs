use secrecy::{ExposeSecret, SecretString};
use sqlx::PgPool;

use crate::{common::model::User, server::auth::hash_password};

pub async fn register_user(
    pool: &PgPool,
    username: &str,
    email: &str,
    password: SecretString,
) -> Result<User, sqlx::Error> {
    // 1. Hash the password
    let password_hash = hash_password(password).expect("Password hashing failed");

    // 2. Insert user record with hashed password
    let user = sqlx::query_as(
        r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, username, email
        "#,
    )
    .bind(username)
    .bind(email)
    .bind(password_hash.expose_secret())
    .fetch_one(pool)
    .await?;

    Ok(user)
}

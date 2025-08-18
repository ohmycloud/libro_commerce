use common::models::Book;
use sqlx::PgPool;
use tracing::instrument;

/// Retrieves all books from the database.
#[instrument(skip(pool))]
pub async fn list_books(pool: &PgPool) -> sqlx::Result<Vec<Book>> {
    sqlx::query_as!(
        Book,
        r#"
            SELECT id, title, author, price
            FROM books
            ORDER BY title
        "#
    )
    .fetch_all(pool)
    .await
}

// Searches books by title keyword.
#[instrument(skip(pool))]
pub async fn search_books(pool: &PgPool, keyword: &str) -> sqlx::Result<Vec<Book>> {
    let pattern = format!("%{}%", keyword);

    sqlx::query_as!(
        Book,
        r#"
            SELECT id, title, author, price
            FROM books
            WHERE title ILIKE $1
            ORDER BY title
        "#,
        pattern
    )
    .fetch_all(pool)
    .await
}

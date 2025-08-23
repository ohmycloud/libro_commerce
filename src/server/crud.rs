use sqlx::PgPool;
use tracing::instrument;

use crate::common::model::{Book, NewBook, UpdateBook};

#[instrument(skip(pool))]
pub async fn create_book(pool: &PgPool, new: NewBook) -> Result<Book, sqlx::Error> {
    // metadata is required when returning
    let book: Book = sqlx::query_as(
        r#"
        INSERT INTO books (title, author, price, stock)
        VALUES ($1, $2, $3, $4)
        RETURNING id, title, author, price, stock, metadata
        "#,
    )
    .bind(new.title)
    .bind(new.author)
    .bind(new.price)
    .bind(new.stock)
    .fetch_one(pool)
    .await?;

    Ok(book)
}

/// Retrieves a single book by ID.
#[instrument(skip(pool))]
pub async fn get_book(pool: &PgPool, id: i32) -> Result<Book, sqlx::Error> {
    // metadata is required when returning
    let book = sqlx::query_as(
        r#"
        SELECT id, title, author, price, stock, metadata
        FROM books
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(book)
}

/// Updates a book's field and returns the updated record.
#[instrument(skip(pool))]
pub async fn update_book(pool: &PgPool, id: i32, upd: UpdateBook) -> Result<Book, sqlx::Error> {
    // Fetch current values
    let mut book = get_book(pool, id).await?;
    if let Some(title) = upd.title {
        book.title = title
    };
    if let Some(author) = upd.author {
        book.author = author
    };
    if let Some(price) = upd.price {
        book.price = price
    };
    if let Some(stock) = upd.stock {
        book.stock = stock
    };

    let book = sqlx::query_as(
        r#"
        UPDATE books
        SET title = $2, author = $3, price = $4, stock = $5
        WHERE id = $1
        RETURNING id, title, author, price, stock, metadata
        "#,
    )
    .bind(id)
    .bind(book.title)
    .bind(book.author)
    .bind(book.price)
    .bind(book.stock)
    .fetch_one(pool)
    .await?;

    Ok(book)
}

/// Delete a book, returning the number of effected rows.
#[instrument(skip(pool))]
pub async fn delete_book(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM books where id = $1", id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}

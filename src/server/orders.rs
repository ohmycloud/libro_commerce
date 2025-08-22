use crate::common::model::Order;
use sqlx::{PgPool, Postgres, Transaction};
use tracing::instrument;

/// Creates an order: inserts row, updates inventory, returns Order.
#[instrument(skip(pool))]
pub async fn create_order(
    pool: &PgPool,
    user_id: i32,
    book_ids: Vec<i32>,
) -> Result<Order, sqlx::Error> {
    let mut tx: Transaction<'_, Postgres> = pool.begin().await?;

    // Calculate total price
    let mut total = 0.0;
    for &book_id in &book_ids {
        let rec: (f64,) = sqlx::query_as("SELECT price FROM books WHERE id = $1")
            .bind(book_id)
            .fetch_one(&mut *tx)
            .await?;
        total += rec.0;

        // Decrement stock
        sqlx::query("UPDATE books SET stock = stock - 1 WHERE id = $1 AND stock > 0")
            .bind(book_id)
            .execute(&mut *tx)
            .await?;
    }

    // Insert order record
    let order: Order = sqlx::query_as(
        r#"
        INSERT INTO orders (user_id, book_ids, total)
        VALUES ($1, $2, $3)
        RETURNING id, user_id, book_ids, total, status
        "#,
    )
    .bind(user_id)
    .bind(&book_ids)
    .bind(total)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(order)
}

// Retireves an existing order by ID.
#[instrument(skip(pool))]
pub async fn get_order(pool: &PgPool, order_id: i32) -> Result<Order, sqlx::Error> {
    let order = sqlx::query_as(
        r#"
        SELECT id, user_id, book_ids, total, status
        FROM orders
        WHERE id = $1
        "#,
    )
    .bind(order_id)
    .fetch_one(pool)
    .await?;

    Ok(order)
}

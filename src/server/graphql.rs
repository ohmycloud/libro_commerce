use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject};
use sqlx::PgPool;

use crate::common::model::{Book, Order};

#[derive(SimpleObject)]
pub struct GqlBook {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub price: f64,
    pub stock: i32,
}

#[derive(SimpleObject)]
pub struct GqlOrder {
    pub id: i32,
    pub user_id: i32,
    pub book_ids: Vec<i32>,
    pub total: f64,
    pub transaction_id: Option<String>,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn books(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<GqlBook>> {
        let pool = ctx.data_unchecked::<PgPool>();
        let rows: Vec<Book> = sqlx::query_as(
            r#"
            SELECT id, title, author, price, stock
            FROM books ORDER BY title
            "#,
        )
        .fetch_all(pool)
        .await?;

        let ret = rows
            .into_iter()
            .map(|b| GqlBook {
                id: b.id,
                title: b.title,
                author: b.author,
                price: b.price,
                stock: b.stock,
            })
            .collect();
        Ok(ret)
    }

    async fn book(&self, ctx: &Context<'_>, id: i32) -> async_graphql::Result<GqlBook> {
        let pool = ctx.data_unchecked::<PgPool>();
        let book: Book = sqlx::query_as(
            r#"
            SELECT id, title, author, price, stock FROM books WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        let ret = GqlBook {
            id: book.id,
            title: book.title,
            author: book.author,
            price: book.price,
            stock: book.stock,
        };

        Ok(ret)
    }

    async fn order(&self, ctx: &Context<'_>, id: i32) -> async_graphql::Result<GqlOrder> {
        let pool = ctx.data_unchecked::<PgPool>();
        let order: Order = sqlx::query_as(
            r#"
            SELECT id, user_id, book_ids, total, transaction_id FROM orders
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        let req = GqlOrder {
            id: order.id,
            user_id: order.user_id,
            book_ids: order.book_ids,
            total: order.total,
            transaction_id: None,
        };
        Ok(req)
    }
}

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

/// Builds the GraphQL schema with shared data.
pub fn create_schema(pool: PgPool) -> AppSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(pool)
        .finish()
}

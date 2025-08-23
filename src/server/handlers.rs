use crate::{
    common::model::{Book, NewBook, Order, UpdateBook},
    server::{
        crud::{create_book, delete_book, get_book, update_book},
        inventory,
        orders::{caculate_total, create_order},
        payments,
    },
    user_accounts,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use reqwest::Client;
use sqlx::PgPool;
use tracing::{info, instrument};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub http_client: Client,
}

#[instrument(skip_all)]
pub async fn create_book_handler(
    State(state): State<AppState>,
    Json(payload): Json<NewBook>,
) -> Result<Json<Book>, StatusCode> {
    create_book(&state.db_pool, payload)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[instrument(skip_all)]
pub async fn get_book_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Book>, StatusCode> {
    get_book(&state.db_pool, id)
        .await
        .map(Json)
        .map_err(|_| StatusCode::NOT_FOUND)
}

#[instrument(skip_all)]
pub async fn update_book_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateBook>,
) -> Result<Json<Book>, StatusCode> {
    update_book(&state.db_pool, id, payload)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[instrument(skip_all)]
pub async fn delete_book_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    delete_book(&state.db_pool, id)
        .await
        .map(|rows| {
            if rows > 0 {
                StatusCode::NO_CONTENT
            } else {
                StatusCode::NOT_FOUND
            }
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// GET /api/books
#[instrument(skip_all)]
pub async fn list_books_handler(State(state): State<AppState>) -> Json<Vec<Book>> {
    let books = inventory::list_books(&state.db_pool)
        .await
        .unwrap_or_default();

    Json(books)
}

// POST /api/register/{username}/{email}
#[instrument(skip(username, email))]
pub async fn register_handler(Path((username, email)): Path<(String, String)>) -> Json<String> {
    info!(user = %username, "Registering new user");

    let result = user_accounts::register_user(&username, &email);
    info!("User registration successful");

    Json(result)
}

// POST /api/order/{user_id}/{book_ids}
#[instrument(skip_all)]
pub async fn order_handler(
    State(state): State<AppState>,
    Json(payload): Json<(i32, Vec<i32>, String)>,
) -> Result<Json<Order>, StatusCode> {
    let (user_id, book_ids, card_token) = payload;

    // 1. Process payment
    let transaction = payments::process_payment(
        &state.http_client,
        caculate_total(&state.db_pool, &book_ids).await,
        &card_token,
    )
    .await
    .map_err(|_| StatusCode::PAYMENT_REQUIRED)?;

    // 2. Create order record
    let order = create_order(&state.db_pool, user_id, book_ids)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(order))
}

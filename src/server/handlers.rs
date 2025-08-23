use crate::{
    common::model::{Book, NewBook, NewOrderPayload, Order, TokenResponse, UpdateBook, UserLogin},
    server::{
        auth::verify_password,
        auth_oauth::oauth_client,
        crud::{create_book, delete_book, get_book, update_book},
        inventory,
        orders::{caculate_total, create_order, get_order},
        payments,
    },
    user_accounts,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
};
use reqwest::Client;
use secrecy::SecretString;
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

#[instrument(skip_all)]
pub async fn create_order_handler(
    State(state): State<AppState>,
    Json(payload): Json<NewOrderPayload>,
) -> Result<Json<Order>, StatusCode> {
    // 1. Process payment
    /*
    let transaction = payments::process_payment(
        &state.http_client,
        caculate_total(&state.db_pool, &payload.book_ids).await,
        &payload.card_token,
    )
    .await
    .map_err(|_| StatusCode::PAYMENT_REQUIRED)?;*/

    // 2. Create order record
    let order = create_order(&state.db_pool, payload.user_id, payload.book_ids)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(order))
}

#[instrument(skip_all)]
pub async fn get_order_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Order>, StatusCode> {
    let order = get_order(&state.db_pool, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(order))
}

#[instrument(skip(username, email))]
pub async fn register_handler(Path((username, email)): Path<(String, String)>) -> Json<String> {
    info!(user = %username, "Registering new user");

    let result = user_accounts::register_user(&username, &email);
    info!("User registration successful");

    Json(result)
}

/// Verifying Passwords on Login
#[instrument(skip(state, credentials))]
pub async fn login_handler(
    State(state): State<AppState>,
    Json(credentials): Json<UserLogin>,
) -> Result<Json<TokenResponse>, StatusCode> {
    let record = sqlx::query!(
        r#"
        SELECT id, password_hash FROM users WHERE username = $1
        "#,
        credentials.username
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if verify_password(
        SecretString::new(Box::from(credentials.password)),
        SecretString::new(Box::from(record.password_hash)),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        // Issue JWT token
        Ok(Json(TokenResponse {
            token: "jwt".into(),
        }))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

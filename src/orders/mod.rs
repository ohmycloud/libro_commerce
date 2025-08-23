use axum::extract::State;

use crate::server::handlers::AppState;

pub async fn create_order(
    State(state): State<AppState>,
    user_id: u32,
    book_ids: Vec<u32>,
) -> String {
    format!(
        "Order created for user {} with books: {:?}",
        user_id, book_ids
    )
}

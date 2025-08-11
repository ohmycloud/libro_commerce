use axum::{Json, extract::Path};
use common::models::{Book, Order, User};
use inventory;
use tracing::{error, info, instrument};
use user_accounts;

// GET /api/books
#[instrument(skip_all)]
pub async fn books_handler() -> Json<Vec<String>> {
    info!("Fetching book list from inventory module");

    // calls inventory::list_books()
    match inventory::list_books() {
        books => {
            info!(count = books.len(), "Successfully retrieved books");
            Json(books)
        }
    }
}

// POST /api/register/:username/:email
#[instrument(skip(Path))]
pub async fn register_handler(Path((username, email)): Path<(String, String)>) -> Json<String> {
    info!(user = %username, "Registering new user");

    let result = user_accounts::register_user(&username, &email);
    info!("User registration successful");

    Json(result)
}

// POST /api/order/:user_id/:book_ids
#[instrument(skip(Path))]
pub async fn order_handler(Path((user_id, book_ids)): Path<(u32, String)>) -> Json<String> {
    info!(user_id, "Creating order");

    // parse comma-separated book IDs
    let ids = book_ids
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<u32>>();

    // create order
    let order = orders::create_order(user_id, ids);
    info!(order = %order, "Order created");

    Json(order)
}

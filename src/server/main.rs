use axum::{
    Json, Router,
    extract::Path,
    routing::{get, post},
};
use std::net::SocketAddr;
use tokio;
use tower_http::trace::TraceLayer;
use tracing_error::ErrorLayer;
use tracing_subscriber::Registry;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;

async fn root() -> &'static str {
    "Welcome to LibroCommerce!"
}

async fn get_books(Path(id): Path<i32>) -> Json<Book> {
    // Simulate fetching from inventory module
    let book = inventory::find_book(id).await;
    Json(book)
}

async fn list_books() -> Json<Vec<Book>> {
    let books = inventory::list_books().await;
    Json(books)
}

#[tokio::main]
async fn main() {
    // 1. Set up environment filter(LOG_LEVEL env var; default INFO).
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    // 2. Build a subscriber with formatted, JSON and filter layers.
    fmt()
        .json() // JSON output for structured logs
        .with_env_filter(filter) // Filter based on RUST_LOG
        .init(); // Install as global default

    // Build a subscriber with an error layer
    let subscriber = Registry::default()
        .with(filter)
        .with(fmt::layer().json())
        .with(ErrorLayer::default());

    // Initialize the subscriber
    subscriber.init();

    // 3. Define routes
    let app = Router::new()
        .route("/", get(root))
        .route("/api/books", get(handlers::books_handler))
        .route("/api/book/:id", get(get_book))
        .route(
            "/api/register/:username/:email",
            post(handlers::register_handler),
        )
        .route(
            "/api/order/:user_id/:book_ids",
            post(handlers::order_handler),
        )
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &http::Request<_>| {
                tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri())
            }),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

use axum::{
    Router, http,
    middleware::from_fn,
    routing::{delete, get, post, put},
};
use libro_commerce::server::{
    db,
    handlers::{self, AppState},
    middleware::auth_middleware,
};
use reqwest::Client;
use std::net::SocketAddr;
use tokio;
use tower_http::trace::TraceLayer;
use tracing_error::ErrorLayer;
use tracing_subscriber::Registry;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
pub async fn main() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // Build a subscriber with an error layer
    let subscriber = Registry::default()
        .with(filter)
        .with(fmt::layer().json())
        .with(ErrorLayer::default());

    // Initialize the subscriber
    subscriber.init();

    // Initialize database pool and HTTP client
    let db_pool = db::init_db_pool().await;
    let http_client = Client::new();

    let app_state = AppState {
        db_pool,
        http_client,
    };

    let protected_routes = Router::new()
        .route("/order", post(handlers::order_handler))
        .layer(from_fn(auth_middleware));

    // Define routes
    let app = Router::new()
        .route("/api/books", post(handlers::create_book_handler))
        .route("/api/books", get(handlers::list_books_handler))
        .route("/api/books/{id}", get(handlers::get_book_handler))
        .route("/api/books/{id}", put(handlers::update_book_handler))
        .route("/api/books/{id}", delete(handlers::delete_book_handler))
        .route(
            "/api/register/{username}/{email}",
            post(handlers::register_handler),
        )
        // mount protected routes under /api
        .nest("/api", protected_routes)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &http::Request<_>| {
                tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri())
            }),
        )
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

use axum::{
    Router, http,
    routing::{get, post},
};
use libro_commerce::server::handlers;
use std::net::SocketAddr;
use tokio;
use tower_http::trace::TraceLayer;
use tracing_error::ErrorLayer;
use tracing_subscriber::Registry;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

async fn root() -> &'static str {
    "Welcome to LibroCommerce!"
}

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

    // 3. Define routes
    let app = Router::new()
        .route("/", get(root))
        .route("/api/books", get(handlers::books_handler))
        .route(
            "/api/register/{username}/{email}",
            post(handlers::register_handler),
        )
        .route(
            "/api/order/{user_id}/{book_ids}",
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
    println!("Server running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

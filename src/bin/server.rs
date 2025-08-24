use libro_commerce::create_app;
use std::net::SocketAddr;
use tokio;
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

    let app = create_app().await;
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

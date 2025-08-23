use async_graphql_axum::{GraphQLBatchRequest, GraphQLResponse};
use axum::{
    Router,
    extract::Extension,
    http,
    middleware::from_fn,
    response::Html,
    routing::{delete, get, post, put},
};
use dotenv::dotenv;
use libro_commerce::server::{
    db,
    graphql::{AppSchema, create_schema},
    handlers::{self, AppState, create_order_handler, get_order_handler},
    middleware::auth_middleware,
};
use reqwest::Client;
use std::env;
use std::net::SocketAddr;
use tokio;
use tower_http::trace::TraceLayer;
use tracing_error::ErrorLayer;
use tracing_subscriber::Registry;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
pub async fn main() {
    // loads .env into env vars
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let backup_path = env::var("BACKUP_PATH").expect("BACKUP_PATH not set");

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
        .route("/order", post(handlers::create_order_handler))
        .layer(from_fn(auth_middleware));

    let order_routes = Router::new()
        .route("/orders", post(create_order_handler))
        .route("/orders", get(get_order_handler))
        .layer(from_fn(auth_middleware));

    let schema = create_schema(app_state.db_pool.clone());

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
        .nest("/api", order_routes)
        .route(
            "/graphql",
            post(
                |schema: Extension<AppSchema>, req: GraphQLBatchRequest| async move {
                    GraphQLResponse(schema.execute_batch(req.into_inner()).await)
                },
            ),
        )
        .route(
            "/graphiql",
            get(|| async {
                Html(async_graphql::http::playground_source(
                    async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
                ))
            }),
        )
        .layer(Extension(schema))
        .with_state(app_state)
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

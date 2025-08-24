use crate::server::{
    db,
    graphql::{AppSchema, create_schema},
    handlers::{self, AppState, create_order_handler, get_order_handler},
    middleware::auth_middleware,
};
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
use reqwest::Client;
use tower_http::trace::TraceLayer;

pub async fn create_app() -> Router {
    // loads .env into env vars
    dotenv().ok();
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
    app
}

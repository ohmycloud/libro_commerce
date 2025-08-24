use axum::{body::Body, http::Request};
use http_body_util::BodyExt;
use libro_commerce::create_app;
use reqwest::StatusCode;
use tower::ServiceExt;

#[tokio::test]
async fn test_list_books() {
    let app = create_app().await;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/books")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("Failed to read response body")
        .to_bytes();

    // At least return an array (could be empty in fresh DB)
    let books: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert!(!books.is_empty());
}

#[tokio::test]
async fn test_create_and_get_book() {
    let app = create_app().await;
    // Create a new book
    let payload = serde_json::json!(
        {
            "title": "Test Driven Development",
            "author": "Kent Beck",
            "price": 29.99,
            "stock": 10
        }
    );

    let req = Request::builder()
        .method("POST")
        .uri("/api/books")
        .header("Content-Type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("Failed to read response body")
        .to_bytes();

    let book: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let id = book.get("id").unwrap().as_i64().unwrap();

    // Retrieve the same book
    let req = Request::builder()
        .method("GET")
        .uri(format!("/api/books/{}", id))
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("Failed to read response body")
        .to_bytes();

    let book: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let id = book.get("id").unwrap().as_i64().unwrap();
    let req = Request::builder()
        .method("GET")
        .uri(format!("/api/books/{}", id))
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("Failed to read response body")
        .to_bytes();

    let book: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let title = book.get("title").unwrap();
    assert_eq!(title, "Test Driven Development");
}

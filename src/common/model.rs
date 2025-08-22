use serde::{Deserialize, Serialize};
use sqlx::types::Json;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub price: f64,
    #[serde(default)]
    pub stock: i32, // Default to 0 if missing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Json<BookMetadata>>, // Optional nested metadata
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct BookMetadata {
    #[serde(rename = "publish_date")]
    pub published: Option<String>,
    #[serde(default)]
    pub pages: i32, // Default 0 if missing
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    #[serde(rename = "email_address")]
    pub email: String, // Maps to "email_address" in JSON
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>, // Nested optional struct
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub postal_code: String,
    pub country: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Order {
    pub id: i32,
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(rename = "bookIds")]
    pub book_ids: Vec<i32>,
    pub total: f64,
    #[serde(default = "default_status")]
    pub status: OrderStatus, // Uses default function
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "order_status", rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Shipped,
    Cancelled,
}

fn default_status() -> OrderStatus {
    OrderStatus::Pending
}

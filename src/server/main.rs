use leptos::*;
use tokio::runtime::Runtime;

// Simple simupated server function that returns a greeting message.

pub async fn fetch_greeting() -> String {
    "Hello, LibroCommerce from the server!".to_string()
}

fn main() {
    let rt = Runtime::new.unwrap();
    let greeting = rt.block_on(fetch_greeting());
    println!("{}", greeting);
}

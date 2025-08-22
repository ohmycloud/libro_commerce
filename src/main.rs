use std::vec;

mod client;
mod common;
mod inventory;
mod orders;
mod server;
mod user_accounts;

fn main() {
    // Use the inventory module to list books
    let books = inventory::list_books();
    println!("Available Books: {:?}", books);

    // Simulate registering a new user with the user_accounts module.
    let registration_message = user_accounts::register_user("Alice", "alice@example.com");
    println!("{}", registration_message);

    // Create a sample order using the orders module.
    let order = orders::create_order(1, vec![101, 102]);
    println!("{}", order);
}

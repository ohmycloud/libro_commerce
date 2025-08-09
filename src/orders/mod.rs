pub fn create_order(user_id: u32, book_ids: Vec<u32>) -> String {
    format!(
        "Order created for user {} with books: {:?}",
        user_id, book_ids
    )
}

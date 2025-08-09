pub fn register_user(username: &str, email: &str) -> String {
    format!("User {} registered with email {}", username, email)
}

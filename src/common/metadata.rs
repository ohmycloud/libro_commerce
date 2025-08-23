use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthorRef {
    pub key: String,
}

#[derive(Debug, Deserialize)]
pub struct CoverUrls {
    pub small: Option<String>,
    pub medium: Option<String>,
    pub large: Option<String>,
}

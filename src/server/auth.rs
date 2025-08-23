use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, errors::Error};
use serde::{Deserialize, Serialize};
use tracing::instrument;

static SECRET: &[u8] = b"your-256-bit-secret"; // Replace with env var in production

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // subject(e.g. user ID)
    pub exp: usize,  // expiration as UTC timestamp
}

#[instrument(skip(token))]
pub fn validate_jwt(token: &str) -> Result<Claims, Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(SECRET), &validation)?;
    Ok(token_data.claims)
}

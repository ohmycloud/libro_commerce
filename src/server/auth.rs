use argon2::{
    Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version,
    password_hash::SaltString,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, errors::Error};
use secrecy::{ExposeSecret, SecretString};
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

/// Hashes a plaintext password, returning salt and hash concatenated.
#[instrument(skip(password))]
pub fn hash_password(password: SecretString) -> Result<SecretString, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::new(
        argon2::Algorithm::Argon2i,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password(password.expose_secret().as_bytes(), &salt)?
    .to_string();

    Ok(SecretString::new(Box::from(password_hash)))
}

/// Verifies a plaintext password against the stored hash.
#[instrument(skip(expected_password_hash, password_candidate))]
pub fn verify_password(
    expected_password_hash: SecretString,
    password_candidate: SecretString,
) -> Result<bool, argon2::Error> {
    let expected_password_hash = PasswordHash::new(password_candidate.expose_secret())
        .expect("Failed to parse hash in PHC string format.");

    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .expect("Invalid password.");
    Ok(true)
}

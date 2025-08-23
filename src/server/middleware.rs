use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use tracing::error;

use crate::server::auth::validate_jwt;

pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    // Extract Authorization header
    let auth_header = if let Some(h) = req.headers().get("authorization") {
        h.to_str().map_err(|_| StatusCode::BAD_REQUEST)?
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    // Expect "Bear <token>"
    let token = auth_header
        .strip_prefix("Bearer")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Validate JWT
    match validate_jwt(token) {
        Ok(claims) => {
            tracing::info!(user = %claims.sub, "JWT validated");
            // Optionally store claims in request extensions
            req.extensions_mut().insert(claims);
            Ok(next.run(req).await)
        }
        Err(e) => {
            error!(error = %e, "JWT validation failed");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

use std::env;

use axum::{
    Json,
    extract::Query,
    response::{IntoResponse, Redirect},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use oauth2::{
    AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    EndpointNotSet, EndpointSet, RedirectUrl, RevocationErrorResponseType, Scope,
    StandardErrorResponse, StandardRevocableToken, StandardTokenIntrospectionResponse,
    StandardTokenResponse, TokenResponse, TokenUrl,
    basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
};
use serde::Deserialize;

use crate::{common::model::User, server::auth::Claims};

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Deserialize)]
struct GithubUser {
    login: String,
    id: u64,
    email: Option<String>,
}

pub fn oauth_client() -> Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
> {
    let github_client_id = env::var("GITHUB_CLIENT_ID").unwrap();
    let github_client_secret = env::var("GITHUB_CLIENT_SECRET").unwrap();
    let github_redirect_url = env::var("GITHUB_REDIRECT_URL").unwrap();

    let client_id = ClientId::new(github_client_id);
    let client_secret = ClientSecret::new(github_client_secret);
    let redirect_url = RedirectUrl::new(github_redirect_url).expect("Invalid redirect URL");
    let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
        .expect("Invalid token endpoint URL");

    BasicClient::new(client_id)
        .set_client_secret(client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(redirect_url)
}

pub async fn github_auth_redirect() -> Redirect {
    let (authorize_url, _csrf_token) = oauth_client()
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("read:user".to_string()))
        .url();

    Redirect::to(authorize_url.as_ref())
}

pub async fn github_auth_callback(Query(params): Query<AuthRequest>) -> impl IntoResponse {
    let token = oauth_client()
        .exchange_code(AuthorizationCode::new(params.code))
        .request_async(&reqwest::Client::new())
        .await;

    if let Ok(token) = token {
        let access_token = token.access_token().secret();
        let client = reqwest::Client::new();
        let user: GithubUser = client
            .get("https://api.github.com/user")
            .bearer_auth(access_token)
            .header("User-Agent", "libro_commerce")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        // Here you would create or fetch a local user record in our database.
        let local_user = User {
            id: user.id as i32,
            username: user.login.clone(),
            email: user.email.clone().unwrap_or_default(),
            address: None,
        };
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("invalid timestamp")
            .timestamp() as usize;
        let claims = Claims {
            sub: local_user.id.to_string(),
            exp: expiration,
        };
        let jwt = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref()),
        )
        .unwrap();

        // Return the JWT to the client; you could also set a cookie here
        Json(serde_json::json!({"token": jwt})).into_response()
    } else {
        (axum::http::StatusCode::UNAUTHORIZED, "OAuth failed").into_response()
    }
}

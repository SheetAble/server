use axum::{
  async_trait,
  extract::FromRequestParts,
  http::{request::Parts, StatusCode},
  response::{IntoResponse, Response},
  routing::{get, post},
  Json, RequestPartsExt,
};
use axum_extra::{
  headers::{authorization::Bearer, Authorization},
  TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Display;
use std::env;
use dotenvy::dotenv;



static KEYS: Lazy<Keys> = Lazy::new(|| {
  let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set"); // TODO: use .env for this
  Keys::new(secret.as_bytes())
});

pub async fn protected(claims: Claims) -> Result<String, AuthError> {
  // Send the protected data to the user
  Ok(format!(
    "Welcome to the protected area :)\nYour data:\n{claims}",
  ))
}

pub async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
  // Check if the user sent the credentials
  if payload.client_id.is_empty() || payload.client_secret.is_empty() {
    return Err(AuthError::MissingCredentials);
  }
  // Here you can check the user credentials from a database
  if payload.client_id != "foo" || payload.client_secret != "bar" {
    return Err(AuthError::WrongCredentials);
  }
  let claims = Claims {
    sub: "b@b.com".to_owned(),
    company: "ACME".to_owned(),
    // Mandatory expiry time as UTC timestamp
    exp: 2000000000, // May 2033
  };
  // Create the authorization token

  dotenv().ok();

  let token =
    encode(&Header::default(), &claims, &KEYS.encoding).map_err(|_| AuthError::TokenCreation)?;

  // Send the authorized token
  Ok(Json(AuthBody::new(token)))
}

impl Display for Claims {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Email: {}\nCompany: {}", self.sub, self.company)
  }
}

impl AuthBody {
  fn new(access_token: String) -> Self {
    Self {
      access_token,
      token_type: "Bearer".to_string(),
    }
  }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
  S: Send + Sync,
{
  type Rejection = AuthError;

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
    // Extract the token from the authorization header
    let TypedHeader(Authorization(bearer)) = parts
      .extract::<TypedHeader<Authorization<Bearer>>>()
      .await
      .map_err(|_| AuthError::InvalidToken)?;
    // Decode the user data
    dotenv().ok();

    let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
      .map_err(|_| AuthError::InvalidToken)?;

    Ok(token_data.claims)
  }
}

impl IntoResponse for AuthError {
  fn into_response(self) -> Response {
    let (status, error_message) = match self {
      AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
      AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
      AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
      AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
    };
    let body = Json(json!({
        "error": error_message,
    }));
    (status, body).into_response()
  }
}

struct Keys {
  encoding: EncodingKey,
  decoding: DecodingKey,
}

impl Keys {
  fn new(secret: &[u8]) -> Self {
    Self {
      encoding: EncodingKey::from_secret(secret),
      decoding: DecodingKey::from_secret(secret),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  sub: String,
  company: String,
  exp: usize,
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
  access_token: String,
  token_type: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
  client_id: String,
  client_secret: String,
}

#[derive(Debug)]
pub enum AuthError {
  WrongCredentials,
  MissingCredentials,
  TokenCreation,
  InvalidToken,
}

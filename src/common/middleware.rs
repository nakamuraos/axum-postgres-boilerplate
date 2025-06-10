use std::time::Duration;

use axum::{
  body::Body,
  extract::Request,
  http::{self, HeaderName, Response, StatusCode},
  middleware::Next,
};
use chrono::{Duration as ChronoDuration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use tower_http::{
  cors::{AllowHeaders, Any, CorsLayer},
  normalize_path::NormalizePathLayer,
  request_id::{MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
  timeout::TimeoutLayer,
};

use crate::common::api_error::ApiError;

#[derive(Clone, Default)]
pub struct Id;

impl MakeRequestId for Id {
  fn make_request_id<B>(&mut self, _: &Request<B>) -> Option<RequestId> {
    let id = uuid::Uuid::now_v7().to_string().parse().unwrap();
    Some(RequestId::new(id))
  }
}

/// Sets the 'x-request-id' header with a randomly generated UUID v7.
///
/// SetRequestId will not override request IDs if they are already present
/// on requests or responses.
pub fn request_id_layer() -> SetRequestIdLayer<Id> {
  let x_request_id = HeaderName::from_static("x-request-id");
  SetRequestIdLayer::new(x_request_id.clone(), Id)
}

// Propagates 'x-request-id' header from the request to the response.
///
/// PropagateRequestId wont override request ids if its already
/// present on requests or responses.
pub fn propagate_request_id_layer() -> PropagateRequestIdLayer {
  let x_request_id = HeaderName::from_static("x-request-id");
  PropagateRequestIdLayer::new(x_request_id)
}

/// Layer that applies the Cors middleware which adds headers for CORS.
pub fn cors_layer() -> CorsLayer {
  CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(AllowHeaders::mirror_request())
    .max_age(Duration::from_secs(600))
}

/// Layer that applies the Timeout middleware which apply a timeout to requests.
/// The default timeout value is set to 15 seconds.
pub fn timeout_layer() -> TimeoutLayer {
  TimeoutLayer::new(Duration::from_secs(15))
}

/// Middleware that normalizes paths.
///
/// Any trailing slashes from request paths will be removed. For example, a request with `/foo/`
/// will be changed to `/foo` before reaching the inner service.
pub fn normalize_path_layer() -> NormalizePathLayer {
  NormalizePathLayer::trim_trailing_slash()
}

#[derive(Serialize, Deserialize)]
pub struct Cliams {
  pub exp: usize,
  pub iat: usize,
  pub email: String,
}

/// Layer that applies the Authorize middleware
pub async fn authorize_layer(mut req: Request, next: Next) -> Result<Response<Body>, ApiError> {
  let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);
  let auth_header = match auth_header {
    Some(header) => header
      .to_str()
      .map_err(|_| ApiError::Forbidden("Empty header is not allowed".to_string()))?,
    None => {
      return Err(ApiError::Forbidden(
        "Please add the JWT token to the header".to_string(),
      ))
    }
  };
  let mut header = auth_header.split_whitespace();
  let (bearer, token) = (header.next(), header.next());
  if bearer != Some("Bearer") {
    return Err(ApiError::Forbidden(
      "Authorization method not supported".to_string(),
    ));
  }
  let token_data = match decode_jwt(token.unwrap().to_string()) {
    Ok(data) => data,
    Err(_) => return Err(ApiError::Unauthorized("Unable to decode token".to_string())),
  };
  // Fetch the user details from the database
  let current_user = match retrieve_user_by_email(&token_data.claims.email) {
    Some(user) => user,
    None => {
      return Err(ApiError::Unauthorized(
        "You are not an authorized user".to_string(),
      ))
    }
  };
  req.extensions_mut().insert(current_user);
  Ok(next.run(req).await)
}

pub fn encode_jwt(email: String) -> Result<String, StatusCode> {
  let jwt_token: String = "randomstring".to_string();

  let now = Utc::now();
  let expire: chrono::TimeDelta = ChronoDuration::hours(24);
  let exp: usize = (now + expire).timestamp() as usize;
  let iat: usize = now.timestamp() as usize;

  let claim = Cliams { iat, exp, email };
  let secret = jwt_token.clone();

  encode(
    &Header::default(),
    &claim,
    &EncodingKey::from_secret(secret.as_ref()),
  )
  .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Cliams>, StatusCode> {
  let secret = "randomstring".to_string();

  let result: Result<TokenData<Cliams>, StatusCode> = decode(
    &jwt,
    &DecodingKey::from_secret(secret.as_ref()),
    &Validation::default(),
  )
  .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
  result
}

#[derive(Clone)]
pub struct CurrentUser {
  pub email: String,
  pub first_name: String,
  pub last_name: String,
  pub password_hash: String,
}

fn retrieve_user_by_email(_: &str) -> Option<CurrentUser> {
  let current_user: CurrentUser = CurrentUser {
    email: "myemail@gmail.com".to_string(),
    first_name: "Eze".to_string(),
    last_name: "Sunday".to_string(),
    // $2b$12$Gwf0uvxH3L7JLfo0CC/NCOoijK2vQ/wbgP.LeNup8vj6gg31IiFkm
    password_hash: "hashed".to_string(),
  };
  Some(current_user)
}

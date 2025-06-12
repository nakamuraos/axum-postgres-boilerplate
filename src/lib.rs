pub mod common;
pub mod database;
pub mod doc;
pub mod modules;

mod query_root;

use async_graphql::{dynamic, http::GraphiQLSource};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
  body::Body,
  extract::State,
  response::{Html, IntoResponse, Response},
  routing::{get, post},
  Router,
};
use base64::{engine::general_purpose, Engine};
use common::{cfg::Config, middleware, telemetry};
use database::Db;
use hyper::StatusCode;
use utoipa::OpenApi;
use utoipa_swagger_ui::{BasicAuth, Config as SwaggerConfig, SwaggerUi};

use crate::modules::auth::guards::auth_guard;

#[derive(Clone)]
pub struct AppState {
  pub db: Db,
  pub cfg: Config,
}

pub fn router(cfg: Config, db: Db) -> Router {
  let app_state = AppState { db, cfg };

  // Middleware that adds high level tracing to a Service.
  // Trace comes with good defaults but also supports customizing many aspects of the output:
  // https://docs.rs/tower-http/latest/tower_http/trace/index.html
  let trace_layer = telemetry::trace_layer();

  // Sets 'x-request-id' header with randomly generated uuid v7.
  let request_id_layer = middleware::request_id_layer();

  // Propagates 'x-request-id' header from the request to the response.
  let propagate_request_id_layer = middleware::propagate_request_id_layer();

  // Layer that applies the Cors middleware which adds headers for CORS.
  let cors_layer = middleware::cors_layer();

  // Layer that applies the Timeout middleware, which sets a timeout for requests.
  // The default value is 15 seconds.
  let timeout_layer = middleware::timeout_layer();

  // Any trailing slashes from request paths will be removed. For example, a request with `/foo/`
  // will be changed to `/foo` before reaching the internal service.
  let normalize_path_layer = middleware::normalize_path_layer();

  // Create the router with the routes.
  let router = modules::router();

  // Create the API documentation using OpenAPI and Swagger UI.
  let api_doc = SwaggerUi::new(app_state.cfg.swagger_endpoint.clone())
    .url(
      app_state.cfg.swagger_endpoint.clone() + "/api-doc/openapi.json",
      doc::ApiDoc::openapi(),
    )
    .config({
      let mut config = SwaggerConfig::default().persist_authorization(true);
      if !app_state.cfg.swagger_basic_auth.is_empty() {
        let parts: Vec<&str> = app_state.cfg.swagger_basic_auth.split(':').collect();
        if parts.len() == 2 {
          config = config.basic_auth(BasicAuth {
            username: parts[0].to_string(),
            password: parts[1].to_string(),
          });
        } else {
          panic!("Invalid format for swagger_basic_auth. Expected 'username:password'.");
        }
      }
      config
    });

  // Create the GraphQL schema using the query root.
  let schema = query_root::schema(app_state.db.conn.clone(), None, None).unwrap();
  let graphql_router = Router::new().nest(
    &app_state.cfg.graphql_endpoint,
    Router::new()
      .merge({
        let mut router = Router::new().route("/", get(graphql_playground));
        if !app_state.cfg.graphql_basic_auth.is_empty() {
          let parts: Vec<&str> = app_state.cfg.graphql_basic_auth.split(':').collect();
          if parts.len() == 2 {
            router = router.layer(axum::middleware::from_fn({
              let auth_config = app_state.clone();
              move |req, next| {
                let auth_config = auth_config.clone();
                async move { basic_auth_layer(State(auth_config), req, next).await }
              }
            }));
          } else {
            panic!("Invalid format for graphql_basic_auth. Expected 'username:password'.");
          }
        }
        router
      })
      .merge(
        Router::new()
          .route("/", post(graphql_handler))
          .with_state(schema)
          .layer(axum::middleware::from_fn(auth_guard)),
      ),
  );

  // Combine all the routes and apply the middleware layers.
  // The order of the layers is important. The first layer is the outermost layer.
  Router::new()
    .merge(router)
    .merge(api_doc)
    .merge(graphql_router)
    .layer(normalize_path_layer)
    .layer(cors_layer)
    .layer(timeout_layer)
    .layer(propagate_request_id_layer)
    .layer(trace_layer)
    .layer(request_id_layer)
    .with_state(app_state)
}

async fn graphql_handler(
  schema: axum::extract::State<dynamic::Schema>,
  req: GraphQLRequest,
) -> GraphQLResponse {
  schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground(State(state): State<AppState>) -> Html<String> {
  let endpoint = &state.cfg.graphql_endpoint;
  Html(GraphiQLSource::build().endpoint(endpoint).finish())
}

/// Middleware that applies basic authentication.
pub async fn basic_auth_layer(
  State(state): State<crate::AppState>,
  req: axum::http::Request<Body>,
  next: axum::middleware::Next,
) -> Result<Response<Body>, StatusCode> {
  let auth_header = req.headers().get("authorization");

  if let Some(header_value) = auth_header {
    if let Ok(auth_str) = header_value.to_str() {
      if auth_str.starts_with("Basic ") {
        let encoded = &auth_str[6..];
        if let Ok(decoded) = general_purpose::STANDARD.decode(encoded) {
          if let Ok(decoded_str) = String::from_utf8(decoded) {
            let parts: Vec<&str> = decoded_str.splitn(2, ':').collect();
            let config_parts: Vec<&str> = state.cfg.graphql_basic_auth.split(':').collect();
            let username = config_parts[0].to_string();
            let password = config_parts[1].to_string();
            if parts.len() == 2 && parts[0] == username && parts[1] == password {
              return Ok(next.run(req).await);
            }
          }
        }
      }
    }
  }

  let mut response = StatusCode::UNAUTHORIZED.into_response();
  response.headers_mut().insert(
    "WWW-Authenticate",
    "Basic realm=\"Restricted\"".parse().unwrap(),
  );
  Ok(response)
}

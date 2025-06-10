pub mod common;
pub mod database;
pub mod doc;
pub mod modules;

mod query_root;

use async_graphql::{dynamic, http::GraphiQLSource};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{response::Html, routing::get, Router};
use common::{cfg::Config, middleware, telemetry};
use database::Db;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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
  let api_doc = SwaggerUi::new("/docs").url("/api-doc/openapi.json", doc::ApiDoc::openapi());

  // Create the GraphQL schema using the query root.
  let schema = query_root::schema(app_state.db.conn.clone(), None, None).unwrap();
  let graphql_router = Router::new()
    .route("/graphql", get(graphql_playground).post(graphql_handler))
    .with_state(schema.clone());

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

async fn graphql_playground() -> Html<String> {
  Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

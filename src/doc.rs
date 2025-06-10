use utoipa::{
  openapi::security::{ApiKey, ApiKeyValue, Http, HttpAuthScheme, SecurityScheme},
  Modify, OpenApi,
};
use utoipauto::utoipauto;

#[utoipauto]
#[derive(OpenApi)]
#[openapi(
  modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
  fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
    // We can unwrap safely since there already is components registered.
    let components = openapi.components.as_mut().unwrap();

    // Add security schemes to the OpenAPI components
    components.add_security_scheme(
      "bearerAuth",
      SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
    );

    // Add API key security scheme to the OpenAPI components
    components.add_security_scheme(
      "api_key",
      SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("api_key"))),
    )
  }
}

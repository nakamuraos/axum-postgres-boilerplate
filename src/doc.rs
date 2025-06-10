use utoipa::OpenApi;
use utoipauto::utoipauto;

#[utoipauto]
#[derive(OpenApi)]
#[openapi()]
pub struct ApiDoc;

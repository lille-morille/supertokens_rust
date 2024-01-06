use std::time::Duration;

///////////////
// ENDPOINTS //
///////////////
pub(crate) const ENDPOINT_CORE_JWKS: &str = ".well-known/jwks.json";
pub(crate) const ENDPOINT_CORE_API_VERSION: &str = "apiversion";
pub(crate) const ENDPOINT_JWT: &str = "recipe/jwt";
pub(crate) const ENDPOINT_RECIPE_SIGNIN: &str = "recipe/signin";

////////////////
//   VALUES   //
////////////////
pub(crate) const DEFAULT_JWT_EXPIRATION_TIME: Duration = Duration::new(86400, 0);

/////////////////
// IDENTIFIERS //
/////////////////
pub(crate) const HEADER_RID: &str = "rid";
pub(crate) const HEADER_API_KEY: &str = "api-key";
pub(crate) const HEADER_CDI_VERSION: &str = "cdi-version";

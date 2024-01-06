use std::time::Duration;

pub(crate) const ENDPOINT_CORE_JWKS: &str = ".well-known/jwks.json";
pub(crate) const ENDPOINT_CORE_API_VERSION: &str = "apiversion";
pub(crate) const ENDPOINT_JWT: &str = "recipe/jwt";
pub(crate) const DEFAULT_JWT_EXPIRATION_TIME: Duration = Duration::new(86400, 0);
pub(crate) const ENDPOINT_RECIPE_SIGNIN: &str = "recipe/signin";

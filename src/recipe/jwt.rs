use crate::constants::{DEFAULT_JWT_EXPIRATION_TIME, ENDPOINT_CORE_JWKS, ENDPOINT_JWT};
use crate::{Recipe, SuperTokens};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Creates a JWT on behalf of a user and returns it
pub async fn create_token<T: Serialize>(
    st: &SuperTokens,
    payload: T,
    expiration_time: Option<Duration>,
) -> Result<String, JwtCreationError> {
    let body = JwtCreationRequest::new(
        payload,
        &st.core_domain,
        expiration_time
            .unwrap_or(DEFAULT_JWT_EXPIRATION_TIME)
            .as_secs() as u32,
    );

    let resp = reqwest::Client::new()
        .post(&st.get_url(ENDPOINT_JWT))
        .headers(st.get_headers(Some(Recipe::Jwt)))
        .json(&body)
        .send()
        .await;

    if let Err(err) = resp {
        return match err.status() {
            None => Err(JwtCreationError::Unknown),
            Some(status) => match status.as_u16() {
                400 => Err(JwtCreationError::BadRequest("Bad Request".to_string())),
                404 => Err(JwtCreationError::NotFound),
                500 => Err(JwtCreationError::InternalError),
                _ => Err(JwtCreationError::Unknown),
            },
        };
    }
    let resp_payload = resp
        .expect("Error here is not possible")
        .json::<JwtResponsePayload>()
        .await
        .expect("Json for status 200");

    // from docs at https://app.swaggerhub.com/apis/supertokens/CDI/4.0.2#/JWT%20Recipe/createSignedJWT
    // check the status field on the response

    if resp_payload.status == "OK" {
        return Ok(resp_payload.jwt.expect("Is here since status is OK"));
    }
    return Err(JwtCreationError::UnsupportedAlgorithm);
}

/// Payload to create a jwt token for a user, with the custom claims type `T`
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JwtCreationRequest<T> {
    /// The payload of the JWT, should be a JSON object.
    payload: T,

    /// The algorithm to use when creating the JWT.
    algorithm: String,

    /// This is used as the value for the issuer claim in the JWT payload.
    ///
    /// Example: https://api.test.com/
    jwks_domain: String,

    /// Duration in seconds, used to calculate JWT expiry
    ///
    /// Example: example: 86400
    validity: u32,

    /// Decides if the token should be signed with a dynamic or static key, defaults to true
    use_static_signing_key: bool,
}

impl<T: Serialize> JwtCreationRequest<T> {
    pub fn new(payload: T, domain: &str, expiration_time_seconds: u32) -> Self {
        Self {
            algorithm: String::from("RS256"),
            validity: expiration_time_seconds,
            jwks_domain: domain.to_string(),
            use_static_signing_key: true,
            payload,
        }
    }
}

#[derive(Debug)]
pub enum JwtCreationError {
    BadRequest(String),
    UnsupportedAlgorithm,
    NotFound,
    InternalError,
    Unknown,
}

/// Payload returned from calling the JWT api at supertokens_core
#[derive(Deserialize, Debug)]
struct JwtResponsePayload {
    status: String,
    jwt: Option<String>,
}

/// Retrieve JWKs for JWT verification, containing both static and dynamic keys.
///
/// Terminology:
/// - JWK : Json Web Key
/// - JWT : Json Web Token
/// - JWKS : Json Web Key-Set
///
/// For more info, see [JWT](https://jwt.io/introduction) or
/// [JWKS](https://auth0.com/docs/secure/tokens/json-web-tokens/json-web-key-sets)
pub async fn get_jwks(core_url: &str) -> Result<Jwks, JwksError> {
    let resp = reqwest::get(core_url.to_owned() + ENDPOINT_CORE_JWKS).await;

    match resp {
        Ok(r) => match r.json::<Jwks>().await {
            Ok(v) => Ok(v),
            Err(_e) => Err(JwksError::ResponseFormat),
        },
        Err(e) => {
            if let Some(s) = e.status() {
                if s.is_server_error() {
                    return Err(JwksError::Internal);
                }
            }
            Err(JwksError::Unknown)
        }
    }
}

/// Response object after fetching jwks from supertokens_core
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Jwks {
    keys: Vec<Jwk>,
}

/// A JWK that can be used to verify a JWT
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Jwk {
    /// The specific cryptographic algorithm used with the key.
    /// Usually RS256
    alg: String,

    /// The family of cryptographic algorithms used with the key.
    kty: String,

    #[serde(rename = "use")]
    /// How the key was meant to be used; sig represents the signature.
    key_use: String,

    /// The unique identifier for the key.
    kid: String,

    /// The x.509 certificate chain.
    ///
    /// The first entry in the array is the certificate to use for token verification;
    /// the other certificates can be used to verify this first certificate.
    x5c: Vec<String>,
}

/// Possible error states of fetching jwks from the
/// super_tokens core api
#[derive(Debug)]
pub enum JwksError {
    NotFound,
    /// Internal server error
    /// Code 500 range
    Internal,

    /// The format of the response did not match the
    /// `Jwks` struct
    ResponseFormat,

    /// Unknown error origin
    Unknown,
}

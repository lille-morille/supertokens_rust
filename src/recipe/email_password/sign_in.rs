use crate::constants::ENDPOINT_RECIPE_SIGNIN;
use crate::{Recipe, SuperTokens};
use reqwest::{Error, StatusCode};
use serde::{Deserialize, Serialize};

/// Signs a user in using email and password
///
/// *Example*
/// ```
/// use supertokens_rust::{SuperTokens, recipe::email_password};
/// let st = SuperTokens::default();
/// let result = email_password::sign_in(&st, "hello@mail.com", "pwd123");
/// ```
pub async fn sign_in(
    st: &SuperTokens,
    email: &str,
    password: &str,
) -> Result<SignInSuccess, SignInError> {
    let resp = reqwest::Client::new()
        .post(st.get_url_with_tenant(ENDPOINT_RECIPE_SIGNIN))
        .headers(st.get_headers(Some(Recipe::EmailPassword)))
        .json(&SignInRequest {
            email: email.to_string(),
            password: password.to_string(),
        })
        .send()
        .await?;

    if resp.status() == StatusCode::OK {
        // check for wrong credentials, as per documentation
        // (blame the API for returning errors with code 200 *sigh*)
        let json = resp
            .json::<SignInResponseRaw>()
            .await
            .expect("Invalid JSON struct");

        if json.status != "OK" {
            return Err(SignInError::WrongCredentials);
        }

        return Ok(SignInSuccess {
            user_id: json.recipe_user_id.expect("Is valid here"),
            user: json.user.expect("Is valid here"),
        });
    }

    Err(match resp.status().as_u16() {
        400 => SignInError::BadRequest(resp.text().await.unwrap_or("Bad Request".to_string())),
        401 => SignInError::InvalidApiKey,
        404 => SignInError::NotFound,
        500 => SignInError::InternalError,
        _ => SignInError::Unknown,
    })
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SignInRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub enum SignInError {
    BadRequest(String),
    WrongCredentials,
    InvalidApiKey,
    NotFound,
    InternalError,
    Unknown,
}

impl From<Error> for SignInError {
    fn from(_value: Error) -> Self {
        SignInError::Unknown
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInResponseRaw {
    status: String,
    user: Option<User>,
    recipe_user_id: Option<String>,
}

pub struct SignInSuccess {
    user: User,
    user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: String,
    is_primary_user: bool,
    tenant_ids: Vec<String>,
    time_joined: i64,
    emails: Vec<String>,
    phone_numbers: Vec<String>,
    third_party: Vec<ThirdParty>,
    login_methods: Vec<LoginMethod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginMethod {
    tenant_ids: Vec<String>,
    recipe_user_id: String,
    verified: bool,
    time_joined: i64,
    recipe_id: String,
    email: String,
    phone_number: String,
    third_party: ThirdParty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThirdParty {
    id: String,
    user_id: String,
}

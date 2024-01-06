use serde::{Deserialize, Serialize};

const ADD_ROLE_TO_USER_ENDPOINT: &str = "recipe/user/role";

/// Adds the given role to the given user
///
/// *Success*: Returns whether this role was already on the user, prior to the request
///
/// *Failure*: Returns all possible variants of failure
pub async fn add_to_user(
    user_id: &str,
    role_id: &str,
    core_url: &str,
) -> Result<bool, AddRoleToUserError> {
    let request_body = AddRoleToUserRequest::new(role_id, user_id);

    let resp = reqwest::Client::new()
        .put(core_url.to_owned() + ADD_ROLE_TO_USER_ENDPOINT)
        .json(&request_body)
        .send()
        .await;

    if resp.is_err() {
        // Find all possible error states and return as err variant
        let err = resp.unwrap_err();
        return if let Some(status) = err.status() {
            let code = status.as_u16();
            Err(match code {
                400 => AddRoleToUserError::BadRequest(err.to_string()),
                401 => AddRoleToUserError::InvalidApiKey,
                404 => AddRoleToUserError::UserNotFound,
                500 => AddRoleToUserError::InternalError,
                _ => AddRoleToUserError::Unknown,
            })
        } else {
            Err(AddRoleToUserError::Unknown)
        };
    }

    // From the docs https://app.swaggerhub.com/apis/supertokens/CDI/4.0.2#/User%20Roles%20Recipe/addUserRole
    // we can still have error with code 200 *sigh*, if this is the case, also return as error

    // response is 200 from here
    let resp_ok = resp
        .expect("Is Ok at this point")
        .json::<AddRoleToUserResponse>()
        .await
        .expect("Should match struct");

    if resp_ok.status != "OK" {
        return Err(AddRoleToUserError::UnknownRole);
    }

    Ok(resp_ok
        .did_user_already_have_role
        .expect("Field is defined for this state"))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddRoleToUserRequest {
    user_id: String,
    role: String,
}

impl AddRoleToUserRequest {
    pub fn new(role: &str, user_id: &str) -> Self {
        Self {
            role: role.to_string(),
            user_id: user_id.to_string(),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddRoleToUserResponse {
    status: String,
    did_user_already_have_role: Option<bool>,
}

pub enum AddRoleToUserError {
    BadRequest(String),
    InvalidApiKey,
    UserNotFound,
    UnknownRole,
    InternalError,
    Unknown,
}

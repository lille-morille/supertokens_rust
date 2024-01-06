use reqwest::header::{HeaderMap, HeaderValue};

pub(crate) mod constants;
pub mod recipe;
pub mod roles;

/// This is the API exposed by the SuperTokens Core. To be consumed by a backend only.
pub struct SuperTokens {
    /// Application ID
    ///
    /// default : `"public"`
    ///
    /// ```
    /// use supertokens_rust::SuperTokens;
    /// assert_eq!(SuperTokens::default().app_id, "public");
    /// ```
    pub app_id: String,

    /// Tenant ID
    ///
    /// default : `"public"`
    ///
    /// ```
    /// use supertokens_rust::SuperTokens;
    /// assert_eq!(SuperTokens::default().tenant_id, "public");
    /// ```
    pub tenant_id: String,

    /// Url domain for the SuperTokens Core instance
    pub core_domain: String,

    /// Authentication API-Key
    pub api_key: String,

    /// Contexts and Dependency Injection for Java : version
    ///
    /// default : "4.0"
    ///
    /// ```
    /// use supertokens_rust::SuperTokens;
    /// assert_eq!(SuperTokens::default().cdi_version, "4.0");
    /// ```
    pub cdi_version: String,
}

impl Default for SuperTokens {
    /// Provides default values defined by the Open API Spec at SuperTokens
    ///
    /// [Docs](https://app.swaggerhub.com/apis/supertokens/CDI/4.0.2#/)
    fn default() -> Self {
        Self {
            app_id: "public".to_string(),
            tenant_id: "public".to_string(),
            core_domain: "".to_string(),
            api_key: "".to_string(),
            cdi_version: "4.0".to_string(),
        }
    }
}

impl SuperTokens {
    /// Returns the full path to the API endpoint url, using relevant config data
    ///
    /// *example*
    /// ```
    /// use supertokens_rust::SuperTokens;
    ///
    /// let super_tokens = SuperTokens::new();
    /// let url = super_tokens.get_url("/recipe/user/metadata");
    /// // prints "localhost:8080/appid-public/recipe/user/metadata";
    /// ```
    pub(crate) fn get_url(&self, endpoint: &str) -> String {
        // Make sure that we don't end up with double / in the url
        // TODO ask Jonathan for macro to catch this at compile time :)
        assert_ne!(endpoint.to_owned().chars().next().unwrap(), '/');
        format!("{}/appid-{}/{}", self.core_domain, self.app_id, endpoint)
    }

    /// Returns the full path to the API endpoint url, using relevant config data
    /// Includes the `tenant_id` url parameter
    ///
    /// *example*
    /// ```
    /// use supertokens_rust::SuperTokens;
    ///
    /// let super_tokens = SuperTokens::default();
    /// let url = super_tokens.get_url_with_tenant("/recipe/user/metadata");
    /// assert_eq!(url, "/appid-public/public/recipe/user/metadata");
    /// ```
    pub(crate) fn get_url_with_tenant(&self, endpoint: &str) -> String {
        // Make sure that we don't end up with double / in the url
        // TODO ask Jonathan for macro to catch this at compile time :)
        assert_ne!(endpoint.to_owned().chars().next().unwrap(), '/');
        format!(
            "{}/appid-{}/{}/{}",
            self.core_domain, self.app_id, self.tenant_id, endpoint
        )
    }

    /// Returns the headers relevant for the given recipe
    ///
    /// Creates a `reqwest::HeaderMap` consisting of a recipe_id, api_key and cdi_version
    ///
    /// *Example*
    /// ```
    /// use supertokens_rust::{Recipe, SuperTokens};
    /// let st = SuperTokens::default();
    /// let headers = st.get_headers(Some(Recipe::EmailPassword));
    /// ```
    pub(crate) fn get_headers(&self, recipe: Option<Recipe>) -> HeaderMap {
        let mut headers = HeaderMap::new();

        if let Some(recipe) = recipe {
            headers.insert("rid", recipe.into());
        }

        let api_key = HeaderValue::from_str(&self.api_key).expect("Should be valid");
        headers.insert("api-key", api_key);

        let cdi_version = HeaderValue::from_str(&self.cdi_version).expect("Should be valid");
        headers.insert("cdi-version", cdi_version);

        headers
    }
}

pub(crate) enum Recipe {
    EmailPassword,
    PasswordLess,
    ThirdParty,
    Jwt,
}

impl From<Recipe> for HeaderValue {
    fn from(value: Recipe) -> Self {
        let rid = match value {
            Recipe::EmailPassword => "emailpassword",
            Recipe::PasswordLess => "passwordless",
            Recipe::ThirdParty => "thirdparty",
            Recipe::Jwt => "jwt",
        };
        HeaderValue::from_str(rid).expect("Should be valid")
    }
}

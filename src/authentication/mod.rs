//! # Authentication Module
//!
//! Provides bindings for the NiFi "authentication" API endpoints.
//!
//! This module is primarily used to query the server's authentication
//! configuration, such as whether login is supported or if an external
//! login flow is required.

use crate::common::client::HttpClient;
use crate::common::config::Config;
use serde::Deserialize;
use std::sync::Arc;

/// A service for interacting with NiFi's authentication configuration endpoints.
///
/// It is instantiated with shared (`Arc`) instances of `HttpClient` and `Config`.
pub struct Authentication {
    client: Arc<HttpClient>,
    config: Arc<Config>,
}

/// A top-level wrapper entity for the authentication configuration response.
///
/// This struct directly maps to the JSON response from the
/// `/authentication/configuration` endpoint.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationConfigurationEntity {
    /// The nested configuration DTO.
    pub authentication_configuration: Option<AuthenticationConfigurationDTO>,
}

/// Data Transfer Object (DTO) containing the actual authentication configuration details.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationConfigurationDTO {
    /// Whether an external login flow is required.
    pub external_login_required: Option<bool>,
    /// Whether this NiFi instance supports login.
    pub login_supported: Option<bool>,
    /// The URI to redirect to for login, if external login is used.
    pub login_uri: Option<String>,
    /// The URI to redirect to for logout.
    pub logout_uri: Option<String>,
}

impl Authentication {
    /// Creates a new instance of the `Authentication` service.
    ///
    /// # Arguments
    ///
    /// * `client` - The shared `HttpClient` to be used for requests.
    /// * `config` - The application configuration (containing `api_base_url`).
    pub fn new(client: Arc<HttpClient>, config: Arc<Config>) -> Self {
        Self { client, config }
    }

    /// Fetches the authentication configuration from the NiFi instance.
    ///
    /// This method calls the `GET /authentication/configuration` endpoint.
    ///
    /// # Errors
    ///
    /// Returns `HttpClientError` if the request fails (e.g., network error,
    /// HTTP status error, or a JSON parsing error).
    pub async fn get_authentication_configuration(
        &self,
    ) -> anyhow::Result<AuthenticationConfigurationEntity> {
        let response = self
            .client
            .get_json::<AuthenticationConfigurationEntity>(&format!(
                "{}/authentication/configuration",
                self.config.api_base_url
            ))
            .await?;
        Ok(response)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::access::Access;
    use tracing::debug;
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn test_get_authentication_configuration() {
        // --- 1. Setup ---
        let client = Arc::new(HttpClient::new());
        let config = Arc::new(Config::default()); // Assumes correct credentials
        let access = Access::new(client.clone(), config.clone());
        let _ = access.get_access_token().await;

        // --- 2. Check Configuration Connection ---
        let authentication = Authentication::new(client.clone(), config.clone());
        let auth_configuration = authentication.get_authentication_configuration().await;
        assert!(
            auth_configuration.is_ok(),
            "test_get_authentication_configuration call error: {:?}",
            auth_configuration
        );

        // --- 3. Assert over object ---
        let auth_configuration = auth_configuration.unwrap();
        assert!(
            auth_configuration.authentication_configuration.is_some(),
        );
        debug!("{:#?}", auth_configuration);
    }
}
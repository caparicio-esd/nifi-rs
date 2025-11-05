//! # Access Module
//!
//! Provides high-level bindings for the NiFi "access" API.
//!
//! This module encapsulates the logic for authenticating (getting a token)
//! and logging out (invalidating and clearing the token).

// Note: These `use` statements are assumed to be correct based on your project's structure.
use crate::common::client::HttpClient;
use crate::common::config::Config;
use serde_json::json;
use std::sync::Arc;
use tracing::debug;
use tracing_test::traced_test; // This is only used by tests, but placed at the module root.

/// A service for interacting with NiFi's access and authentication endpoints.
///
/// It is instantiated with shared (`Arc`) instances of `HttpClient` and `Config`.
/// Actions performed here (like `get_access_token`) will affect the state
/// of the shared `HttpClient`.
pub struct Access {
    client: Arc<HttpClient>,
    config: Arc<Config>,
}

impl Access {
    /// Creates a new instance of the `Access` service.
    ///
    /// # Arguments
    ///
    /// * `client` - The shared `HttpClient` to be used for requests.
    /// * `config` - The application configuration (containing `api_base_url`, `username`, etc.).
    pub fn new(client: Arc<HttpClient>, config: Arc<Config>) -> Self {
        Access { client, config }
    }

    /// Attempts to authenticate against the NiFi API using credentials from `Config`.
    ///
    /// Sends a `POST` to `/access/token` with the username and password.
    ///
    /// On success, it **atomically updates the shared `HttpClient`** with the new
    /// token, so all future API requests will use it.
    ///
    /// # Errors
    ///
    /// Returns `HttpClientError` if the request fails (e.g., `HttpError` 401
    /// for bad credentials, or `RequestError` if the server is unreachable).
    pub async fn get_access_token(&self) -> anyhow::Result<String> {
        debug!("{:?}", &self.config);
        let response = self
            .client
            .post_form::<_, String>(
                &format!("{}/access/token", self.config.api_base_url),
                &json!({
                    "username": self.config.username,
                    "password": self.config.password,
                }),
            )
            .await?;

        // Store the token in the shared client
        self.client.set_auth_token(response.clone()).await?;

        Ok(response)
    }

    /// Logs out of the NiFi API.
    ///
    /// Sends a `DELETE` request to `/access/logout`.
    ///
    /// On success, it **atomically clears the token from the shared `HttpClient`**,
    /// effectively logging the client out.
    ///
    /// # Errors
    ///
    /// Returns `HttpClientError` if the `DELETE` request fails.
    pub async fn logout(&self) -> anyhow::Result<()> {
        // Call the logout endpoint. We expect an empty '()' response.
        let response = self
            .client
            .delete::<()>(&format!("{}/access/logout", self.config.api_base_url))
            .await?;

        // Clear the token from the shared client
        self.client.clear_auth_token().await?;

        Ok(response)
    }
}

/// Integration tests for the Access module.
///
/// These tests perform real network calls to a local NiFi endpoint.
/// (Ensure NiFi is running locally and `Config::default()`
/// has the correct credentials).
#[cfg(test)]
mod test {
    use super::*;
    // Note: `use crate::common::config::Config` might be needed here
    // if `Config::default()` isn't in scope.

    #[tokio::test]
    #[traced_test]
    async fn test_get_access_token() {
        let client = Arc::new(HttpClient::new());
        let config = Arc::new(Config::default()); // Assumes correct credentials
        let access = Access::new(client.clone(), config.clone());

        let access_token = access.get_access_token().await;

        tracing::info!("{:#?}", access_token);
        assert!(
            access_token.is_ok(),
            "test_get_access_token call error: {:?}",
            access_token
        );
    }

    #[tokio::test]
    #[traced_test]
    async fn test_get_access_token_fail() {
        let client = Arc::new(HttpClient::new());
        let config = Arc::new(Config {
            password: "false_password".to_string(),
            ..Default::default()
        });
        let access = Access::new(client.clone(), config.clone());

        let access_token = access.get_access_token().await;

        tracing::info!("{:#?}", access_token);
        // We expect this to fail (is_err())
        assert!(
            access_token.is_err(),
            "test_get_access_token should have failed, but returned Ok: {:?}",
            access_token
        );
    }

    #[tokio::test]
    #[traced_test]
    async fn test_logout() {
        // --- 1. Setup ---
        let config = Arc::new(Config::default());
        let client = Arc::new(HttpClient::new());
        let access = Access::new(client.clone(), config.clone());

        // --- 2. Check initial state (no token) ---
        let initial_token_result = client.get_auth_token().await;
        assert!(initial_token_result.is_ok(), "get_auth_token failed: {:?}", initial_token_result);
        assert!(initial_token_result.unwrap().is_none(), "Token should be None at start");
        tracing::info!("Initial state: Logged-out (OK)");

        // --- 3. Log in ---
        let login_result = access.get_access_token().await;
        assert!(login_result.is_ok(), "Login failed: {:?}", login_result);
        let token_str = login_result.unwrap();
        tracing::info!("Login successful, token [REDACTED]");

        // --- 4. Check post-login state (has token) ---
        let token_after_login_result = client.get_auth_token().await;
        assert!(token_after_login_result.is_ok(), "get_auth_token failed: {:?}", token_after_login_result);
        let token_after_login = token_after_login_result.unwrap();
        assert!(token_after_login.is_some(), "Token should be Some after login");
        assert_eq!(token_after_login.unwrap(), token_str, "Stored token does not match");

        // --- 5. Log out ---
        let logout_result = access.logout().await;
        assert!(logout_result.is_ok(), "Logout failed: {:?}", logout_result);
        tracing::info!("Logout successful");

        // --- 6. Check final state (no token) ---
        let final_token_result = client.get_auth_token().await;
        assert!(final_token_result.is_ok(), "get_auth_token failed: {:?}", final_token_result);
        assert!(final_token_result.unwrap().is_none(), "Token should be None after logout");
        tracing::info!("Final state: Logged-out (OK)");
    }
}
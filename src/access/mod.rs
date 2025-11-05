//! # Access module
//!
//! NiFi bindings for Rust
//!
//!

use crate::common::client::HttpClient;
use crate::common::config::Config;
use serde_json::json;
use std::sync::Arc;
use tracing_test::traced_test;

pub struct Access {
    client: Arc<HttpClient>,
    config: Arc<Config>,
}

impl Access {
    pub fn new(client: Arc<HttpClient>, config: Arc<Config>) -> Self {
        Access { client, config }
    }
    pub async fn get_access_token(&self) -> anyhow::Result<String> {
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
        Ok(response)
    }
}

mod test {
    use super::*;
    #[tokio::test]
    #[traced_test]
    async fn test_get_access_token() {
        let client = Arc::new(HttpClient::new("".into()));
        let config = Arc::new(Config::default());
        let access = Access::new(client.clone(), config.clone());
        let access_token = access.get_access_token().await;
        assert!(
            access_token.is_ok(),
            "test_get_access_token call error: {:?}",
            access_token
        );
        tracing::info!("{:#?}", access_token);
    }
    #[tokio::test]
    #[traced_test]
    async fn test_get_access_token_fail() {
        let client = Arc::new(HttpClient::new("".into()));
        let config = Arc::new(Config {
            password: "false_password".to_string(),
            ..Default::default()
        });
        let access = Access::new(client.clone(), config.clone());
        let access_token = access.get_access_token().await;
        assert!(
            access_token.is_err(),
            "test_get_access_token call error: {:?}",
            access_token
        );
        tracing::info!("{:#?}", access_token);
    }
}

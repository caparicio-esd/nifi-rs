//! # Access module
//!
//! NiFi bindings for Rust
//!
//!

use std::sync::Arc;
use serde_json::json;
use tracing_test::traced_test;
use crate::common::client::HttpClient;
use crate::common::config::Config;

pub struct Access {
    client: Arc<HttpClient>,
    config: Arc<Config>
}

impl Access {
    pub fn new(client: Arc<HttpClient>, config: Arc<Config>) -> Self {
        Access { client, config }
    }
    pub async fn get_access_token(&self) -> anyhow::Result<String> {
        let response = self.client.post_form::<_, String>(
            &format!("{}/access/token", self.config.api_base_url),
            &json!({
                "username": "nifi",
                "password": "nifinifinifinifi",
            })
        ).await?;
        Ok(response)
    }
}

mod test {
    use super::*;
    #[tokio::test]
    #[traced_test]
    async fn test_get_access_token() {
        let client = Arc::new(HttpClient::new("a".into()));
        let config = Arc::new(Config::default());
        let access = Access::new(client.clone(), config.clone());
        let access_token = access.get_access_token().await;
        assert!(
            access_token.is_ok(),
            "La llamada a get_access_token() ha fallado con el error: {:?}",
            access_token
        );
        tracing::info!("{:#?}", access_token);    }
}


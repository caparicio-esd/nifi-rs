//! # Parameter Provider Controller
//!
//! This module defines the data structures and controller logic for managing
//! "Parameter Providers" within the system. It includes:
//!
//! * The main `Controller` struct for interacting with the API.
//! * Data Transfer Objects (Dtos) like `ParameterProviderEntity` and `ParameterProviderDto`
//!     which map directly to the JSON aPI.
//! * Various supporting enums and Dtos that describe properties, allowable values,
//!     and component status.
//!
//! The primary entities are serialized and deserialized with `serde` using `camelCase`
//! conventions to match the target JSON API.

use crate::api::{ParameterProviderEntity};
use crate::common::client::HttpClient;
use crate::common::config::Config;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Manages operations related to Parameter Providers.
///
/// This controller holds a shared `HttpClient` and `Config` to make
/// API requests to the controller endpoints.
#[derive(Debug)]
pub struct Controller {
    /// A thread-safe, shared HTTP client for making API requests.
    client: Arc<HttpClient>,
    /// A thread-safe, shared configuration object, primarily for the API base URL.
    config: Arc<Config>,
}

impl Controller {
    /// Creates a new `Controller` instance.
    ///
    /// # Arguments
    ///
    /// * `client` - An `Arc<HttpClient>` used to make API requests.
    /// * `config` - An `Arc<Config>` containing the API base URL.
    pub fn new(client: Arc<HttpClient>, config: Arc<Config>) -> Self {
        Self { client, config }
    }

    /// Creates a new Parameter Provider on the controller.
    ///
    /// This function performs an HTTP POST to the `/controller/parameter-providers`
    /// endpoint.
    ///
    /// # Arguments
    ///
    /// * `payload` - A `ParameterProviderEntity` describing the new provider.
    ///               The `Default::default()` implementation is a good
    ///               starting point.
    ///
    /// # Errors
    ///
    /// Returns an `anyhow::Error` if the HTTP request fails, if the API
    /// returns an error status code, or if the response cannot be
    /// deserialized into a `ParameterProviderEntity`.
    pub async fn post_parameter_providers(
        &self,
        payload: &ParameterProviderEntity,
    ) -> anyhow::Result<ParameterProviderEntity> {
        let response = self
            .client
            .post_json::<ParameterProviderEntity, ParameterProviderEntity>(
                &format!(
                    "{}/controller/parameter-providers",
                    self.config.api_base_url
                ),
                payload,
            )
            .await?;
        Ok(response)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::access::Access;
    use crate::api::{BundleDto, ParameterProviderDto, RevisionDto};
    use crate::common::client::HttpClient;
    use crate::common::config::Config;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn test_post_parameter_providers() {
        // ME QUEDO AQUÏ,
        // por alguna razón el response de la api no cuadra con los tipos de nifi...
        // me gustaría ver los errores

        // --- 1. Setup ---
        let client = Arc::new(HttpClient::new());
        let config = Arc::new(Config::default()); // Assumes correct credentials
        let access = Access::new(client.clone(), config.clone());
        let _ = access.get_access_token().await;

        let controller = Controller::new(client.clone(), config.clone());
        let mut fake_parameter_provider = ParameterProviderEntity::default();
        fake_parameter_provider.revision = Some(RevisionDto {
            client_id: None,
            last_modifier: None,
            version: Some(0),
        });
        fake_parameter_provider.component = Some(ParameterProviderDto {
            affected_components: None,
            annotation_data: None,
            bundle: Some(BundleDto {
                artifact: Some("nifi-standard-nar".to_string()),
                group: Some("org.apache.nifi".to_string()),
                version: Some("2.6.0".to_string()),
            }),
            comments: None,
            custom_ui_url: None,
            deprecated: None,
            descriptors: HashMap::new(),
            extension_missing: None,
            id: None,
            multiple_versions_available: None,
            name: Some(uuid::Uuid::new_v4().to_string()),
            parameter_group_configurations: Vec::new(),
            parameter_status: None,
            parent_group_id: None,
            persists_state: None,
            position: None,
            properties: HashMap::new(),
            referencing_parameter_contexts: None,
            restricted: None,
            type_: Some(
                "org.apache.nifi.parameter.EnvironmentVariableParameterProvider".to_string(),
            ),
            validation_errors: Vec::new(),
            validation_status: None,
            versioned_component_id: None,
        });
        tracing::debug!(
            "\n{}\n",
            serde_json::to_string_pretty(&fake_parameter_provider).unwrap()
        );
        let parameter_provider_entity = controller
            .post_parameter_providers(&fake_parameter_provider)
            .await;
        assert!(
            parameter_provider_entity.is_ok(),
            "test_post_parameter_providers call error: {:?}",
            parameter_provider_entity
        );
        tracing::debug!(
            "\n{}\n",
            serde_json::to_string_pretty(&parameter_provider_entity.unwrap()).unwrap()
        );
    }
}

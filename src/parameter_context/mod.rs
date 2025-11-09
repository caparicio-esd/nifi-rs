//! # Parameter Context Module
//!
//! Provides high-level bindings for the NiFi "parameter-contexts" API endpoint.
//!
//! This module allows for creating, reading, and updating Parameter Contexts,
//! which are collections of parameters that can be shared across Process Groups.

use crate::common::client::{HttpClient, JsonResponse};
use crate::common::config::Config;
use anyhow::bail;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::api::ParameterContextEntity;

/// A service for interacting with NiFi's Parameter Context endpoints.
///
/// This service is instantiated with shared (`Arc`) instances of `HttpClient` and `Config`.
#[derive(Debug)]
pub struct ParameterContext {
    client: Arc<HttpClient>,
    config: Arc<Config>,
}

impl ParameterContext {
    /// Creates a new instance of the `ParameterContext` service.
    ///
    /// # Arguments
    ///
    /// * `client` - The shared `HttpClient` to be used for requests.
    /// * `config` - The application configuration (containing `api_base_url`).
    pub fn new(client: Arc<HttpClient>, config: Arc<Config>) -> Self {
        Self { client, config }
    }

    /// Creates a new Parameter Context.
    ///
    /// Sends a `POST` request to `/parameter-contexts`.
    ///
    /// # Arguments
    ///
    /// * `payload` - A `ParameterContextEntity` describing the new context.
    ///   Use `ParameterContextEntity::default()` as a base for creation.
    ///
    /// # Errors
    /// Returns `HttpClientError` if the request fails.
    pub async fn post_parameter_contexts(
        &self,
        payload: &ParameterContextEntity,
    ) -> anyhow::Result<ParameterContextEntity> {
        let response = self
            .client
            .post_json::<ParameterContextEntity, ParameterContextEntity>(
                &format!("{}/parameter-contexts", self.config.api_base_url),
                payload,
            )
            .await?;
        Ok(response)
    }

    /// Retrieves a Parameter Context by its ID.
    ///
    /// Sends a `GET` request to `/parameter-contexts/{id}`.
    ///
    /// # Arguments
    ///
    /// * `id` - The UUID of the Parameter Context to fetch.
    ///
    /// # Errors
    /// Returns `HttpClientError` if the request fails (e.g., 404 Not Found).
    pub async fn get_parameter_contexts(&self, id: &str) -> anyhow::Result<ParameterContextEntity> {
        let response = self
            .client
            .get_json::<ParameterContextEntity>(&format!(
                "{}/parameter-contexts/{}",
                self.config.api_base_url, id
            ))
            .await?;
        Ok(response)
    }

    /// Updates an existing Parameter Context.
    ///
    /// Sends a `PUT` request to `/parameter-contexts/{id}`.
    /// The `payload` must contain a valid `RevisionDTO` (with the current version).
    ///
    /// # Arguments
    ///
    /// * `id` - The UUID of the Parameter Context to update.
    /// * `payload` - The complete `ParameterContextEntity` with updates.
    ///
    /// # Errors
    /// Returns `HttpClientError` if the request fails (e.g., 409 Conflict on bad version).
    pub async fn put_parameter_contexts(
        &self,
        id: &str,
        payload: &ParameterContextEntity,
    ) -> anyhow::Result<ParameterContextEntity> {
        let response = self
            .client
            .put_json::<ParameterContextEntity, ParameterContextEntity>(
                &format!("{}/parameter-contexts/{}", self.config.api_base_url, id),
                payload,
            )
            .await?;
        Ok(response)
    }

    pub async fn delete_parameter_contexts(
        &self,
        id: &str,
    ) -> anyhow::Result<ParameterContextEntity> {
        let response = self
            .client
            .get_json::<ParameterContextEntity>(&format!(
                "{}/parameter-contexts/{}",
                self.config.api_base_url, id
            ))
            .await?;
        let version = match response.revision {
            Some(revision) => match revision.version {
                Some(version) => version,
                None => bail!("Revision was None"),
            },
            None => bail!("Revision was None"),
        };

        let response = self
            .client
            .delete::<JsonResponse<ParameterContextEntity>>(&format!(
                "{}/parameter-contexts/{}?version={}",
                self.config.api_base_url, id, version
            ))
            .await?;
        Ok(response.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::access::Access;
    use tracing_test::traced_test;
    use crate::api::{ParameterContextDto, RevisionDto};

    #[tokio::test]
    #[traced_test]
    async fn test_post_parameter_contexts() {
        // --- 1. Setup ---
        let client = Arc::new(HttpClient::new());
        let config = Arc::new(Config::default()); // Assumes correct credentials
        let access = Access::new(client.clone(), config.clone());
        let _ = access.get_access_token().await;

        // --- 2. Check Configuration Connection ---
        let parameter_context = ParameterContext::new(client.clone(), config.clone());
        let mut fake_parameter_context = ParameterContextEntity::default();
        fake_parameter_context.revision = Some(RevisionDto {
            client_id: None,
            last_modifier: None,
            version: Some(0),
        });
        fake_parameter_context.component = Some(ParameterContextDto {
            bound_process_groups: None,
            description: None,
            id: None,
            inherited_parameter_contexts: vec![],
            name: Some(uuid::Uuid::new_v4().to_string()),
            parameter_provider_configuration: None,
            parameters: None,
        });

        tracing::debug!(
            "\n{}\n",
            serde_json::to_string_pretty(&fake_parameter_context).unwrap()
        );
        let parameter_contexts = parameter_context
            .post_parameter_contexts(&fake_parameter_context)
            .await;
        assert!(
            parameter_contexts.is_ok(),
            "test_post_parameter_contexts call error: {:?}",
            parameter_contexts
        );

        // --- 3. Assert over object ---
        let parameter_contexts = parameter_contexts.unwrap();
        assert!(
            parameter_contexts.revision.is_some(),
            "test_post_parameter_contexts call error: {:?}",
            parameter_contexts.revision
        );
        assert_eq!(
            parameter_contexts.revision.unwrap().version.unwrap(),
            1,
            "version should be 1"
        );
    }

    #[tokio::test]
    #[traced_test]
    async fn test_get_parameter_contexts() {
        // --- 1. Setup ---
        let client = Arc::new(HttpClient::new());
        let config = Arc::new(Config::default()); // Assumes correct credentials
        let access = Access::new(client.clone(), config.clone());
        let _ = access.get_access_token().await;

        // --- 2. Check Configuration Connection ---
        let parameter_context = ParameterContext::new(client.clone(), config.clone());
        let mut fake_parameter_context = ParameterContextEntity::default();
        fake_parameter_context.revision = Some(RevisionDto {
            client_id: None,
            last_modifier: None,
            version: Some(0),
        });
        fake_parameter_context.component = Some(ParameterContextDto {
            bound_process_groups: None,
            description: None,
            id: None,
            inherited_parameter_contexts: vec![],
            name: Some(uuid::Uuid::new_v4().to_string()),
            parameter_provider_configuration: None,
            parameters: None,
        });
        tracing::debug!(
            "\n{}\n",
            serde_json::to_string_pretty(&fake_parameter_context).unwrap()
        );
        let parameter_contexts = parameter_context
            .post_parameter_contexts(&fake_parameter_context)
            .await;
        assert!(
            parameter_contexts.is_ok(),
            "test_post_parameter_contexts call error: {:?}",
            parameter_contexts
        );

        // --- 3. Assert over id ---
        let parameter_contexts = parameter_contexts.unwrap();
        assert!(
            parameter_contexts.id.is_some(),
            "test_post_parameter_contexts call error: {:?}",
            parameter_contexts.id
        );
        let id = parameter_contexts.id.unwrap();

        // --- 4. Assert over id ---
        let parameter_context_to_assert =
            parameter_context.get_parameter_contexts(id.as_str()).await;
        assert!(
            parameter_context_to_assert.is_ok(),
            "test_post_parameter_contexts parameter_context_to_assert call error: {:?}",
            parameter_context_to_assert
        );
        tracing::debug!(
            "\n{}\n",
            serde_json::to_string_pretty(&parameter_context_to_assert.unwrap()).unwrap()
        );
    }

    #[tokio::test]
    #[traced_test]
    async fn test_put_parameter_contexts() {
        // --- 1. Setup ---
        let client = Arc::new(HttpClient::new());
        let config = Arc::new(Config::default()); // Assumes correct credentials
        let access = Access::new(client.clone(), config.clone());
        let _ = access.get_access_token().await;

        // --- 2. Check Configuration Connection ---
        let parameter_context = ParameterContext::new(client.clone(), config.clone());
        let mut fake_parameter_context = ParameterContextEntity::default();
        fake_parameter_context.revision = Some(RevisionDto {
            client_id: None,
            last_modifier: None,
            version: Some(0),
        });
        fake_parameter_context.component = Some(ParameterContextDto {
            bound_process_groups: None,
            description: None,
            id: None,
            inherited_parameter_contexts: vec![],
            name: Some(uuid::Uuid::new_v4().to_string()),
            parameter_provider_configuration: None,
            parameters: None,
        });
        let parameter_contexts = parameter_context
            .post_parameter_contexts(&fake_parameter_context)
            .await;
        assert!(
            parameter_contexts.is_ok(),
            "test_post_parameter_contexts call error: {:?}",
            parameter_contexts
        );

        // --- 3. Assert over id ---
        let mut parameter_contexts = parameter_contexts.unwrap();
        assert!(
            parameter_contexts.id.is_some(),
            "test_post_parameter_contexts call error: {:?}",
            parameter_contexts.id
        );
        let id = parameter_contexts.id.as_ref().unwrap();

        // --- 4. Test put operation ---
        let name = parameter_contexts
            .component
            .as_ref()
            .unwrap()
            .name
            .clone()
            .unwrap();
        parameter_contexts.component.as_mut().unwrap().name = Some(format!("{} foo changed", name));
        parameter_contexts.component.as_mut().unwrap().description =
            Some("bar changed".to_string());
        let parameter_context_to_assert = parameter_context
            .put_parameter_contexts(id.as_str(), &parameter_contexts)
            .await;

        // --- 4. Assert over id ---
        assert!(
            parameter_context_to_assert.is_ok(),
            "test_put_parameter_contexts parameter_context_to_assert call error: {:?}",
            parameter_context_to_assert
        );
        tracing::debug!(
            "\n{}\n",
            serde_json::to_string_pretty(&parameter_context_to_assert.unwrap()).unwrap()
        );
    }

    #[tokio::test]
    #[traced_test]
    async fn test_delete_parameter_contexts() {
        // --- 1. Setup ---
        let client = Arc::new(HttpClient::new());
        let config = Arc::new(Config::default()); // Assumes correct credentials
        let access = Access::new(client.clone(), config.clone());
        let _ = access.get_access_token().await;

        // --- 2. Check Configuration Connection ---
        let parameter_context = ParameterContext::new(client.clone(), config.clone());
        let fake_parameter_context = &mut ParameterContextEntity::default();
        fake_parameter_context.revision = Some(RevisionDto {
            client_id: None,
            last_modifier: None,
            version: Some(0),
        });
        fake_parameter_context.component = Some(ParameterContextDto {
            bound_process_groups: None,
            description: None,
            id: None,
            inherited_parameter_contexts: vec![],
            name: Some(uuid::Uuid::new_v4().to_string()),
            parameter_provider_configuration: None,
            parameters: None,
        });
        let parameter_contexts = parameter_context
            .post_parameter_contexts(fake_parameter_context)
            .await;
        assert!(
            parameter_contexts.is_ok(),
            "test_delete_parameter_contexts call error: {:?}",
            parameter_contexts
        );

        // --- 3. Assert over id ---
        let parameter_contexts = parameter_contexts.unwrap();
        assert!(
            parameter_contexts.id.is_some(),
            "test_delete_parameter_contexts call error: {:?}",
            parameter_contexts.id
        );
        let id = parameter_contexts.id.as_ref().unwrap();
        tracing::debug!("\n{}\n", id);

        // --- 4. Test delete operation ---
        let parameter_contexts = parameter_context
            .delete_parameter_contexts(id.as_str())
            .await;
        assert!(
            parameter_contexts.is_ok(),
            "test_delete_parameter_contexts call error: {:?}",
            parameter_contexts
        );
        tracing::debug!(
            "\n{}\n",
            serde_json::to_string_pretty(&parameter_contexts.unwrap()).unwrap()
        );
    }
}

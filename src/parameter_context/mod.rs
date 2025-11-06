//! # Parameter Context Module
//!
//! Provides high-level bindings for the NiFi "parameter-contexts" API endpoint.
//!
//! This module allows for creating, reading, and updating Parameter Contexts,
//! which are collections of parameters that can be shared across Process Groups.

use crate::common::bulletins::BulletinEntity;
use crate::common::client::HttpClient;
use crate::common::config::Config;
use crate::common::types::{PermissionsDTO, PositionDTO, RevisionDTO};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// A service for interacting with NiFi's Parameter Context endpoints.
///
/// This service is instantiated with shared (`Arc`) instances of `HttpClient` and `Config`.
#[derive(Debug)]
pub struct ParameterContext {
    client: Arc<HttpClient>,
    config: Arc<Config>,
}

/// Represents a full Parameter Context entity, including component, revision, and permissions.
///
/// This is the main object used for creating, fetching, and updating contexts.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextEntity {
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<ParameterContextDTO>,
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    pub permissions: Option<PermissionsDTO>,
    pub position: Option<PositionDTO>,
    pub revision: Option<RevisionDTO>,
    pub uri: Option<String>,
}

/// The core data for a Parameter Context.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextDTO {
    pub bound_process_groups: Option<Vec<ProcessGroupEntity>>,
    pub description: Option<String>,
    pub id: Option<String>,
    pub inherited_parameter_contexts: Option<Vec<ParameterContextReferenceEntity>>,
    pub name: Option<String>,
    pub parameter_provider_configuration: Option<ParameterProviderConfigurationEntity>,
    pub parameters: Option<Vec<ParameterEntity>>,
}

/// A reference to another Parameter Context.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextReferenceEntity {
    pub component: Option<ParameterContextReferenceDTO>,
    pub id: Option<String>,
    pub permissions: Option<PermissionsDTO>,
}

/// The core data for a Parameter Context reference.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextReferenceDTO {
    pub id: Option<String>,
    pub name: Option<String>,
}

/// Configuration for a Parameter Provider.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderConfigurationEntity {
    pub component: Option<ParameterProviderConfigurationDTO>,
    pub id: Option<String>,
    pub permissions: Option<PermissionsDTO>,
}

/// The core data for a Parameter Provider's configuration.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderConfigurationDTO {
    pub parameter_group_name: Option<String>,
    pub parameter_provider_id: Option<String>,
    pub parameter_provider_name: Option<String>,
    pub synchronized: Option<bool>,
}

/// An entity wrapper for a Parameter.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterEntity {
    pub can_write: Option<bool>,
    pub parameter: Option<ParameterDTO>,
}

/// The core data for a Parameter, including its value, description, and state.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterDTO {
    pub description: Option<String>,
    pub inherited: Option<bool>,
    pub name: Option<String>,
    pub parameter_context: Option<ParameterContextReferenceEntity>,
    pub provided: Option<bool>,
    pub referenced_assets: Option<Vec<AssetReferenceDTO>>,
    pub referencing_components: Option<Vec<AffectedComponentEntity>>,
    pub sensitive: Option<bool>,
    pub value: Option<String>,
    pub value_removed: Option<bool>,
}

/// A reference to an Asset.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetReferenceDTO {
    pub id: Option<String>,
    pub name: Option<String>,
}

/// An entity that is affected by a Parameter.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedComponentEntity {
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<AffectedComponentDTO>,
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    pub permissions: Option<PermissionsDTO>,
    pub position: Option<PositionDTO>,
    pub process_group: Option<ProcessGroupNameDTO>,
    pub reference_type: Option<ReferenceTypes>,
    pub uri: Option<String>,
}

/// Represents the type of component (e.g., Processor, Controller Service).
#[derive(Debug, Deserialize, Serialize)]
pub enum ReferenceTypes {
    #[serde(rename = "PROCESSOR")]
    Processor,
    #[serde(rename = "CONTROLLER_SERVICE")]
    ControllerService,
    #[serde(rename = "INPUT_PORT")]
    InputPort,
    #[serde(rename = "OUTPUT_PORT")]
    OutputPort,
    #[serde(rename = "REMOTE_INPUT_PORT")]
    RemoteInputPort,
    #[serde(rename = "REMOTE_OUTPUT_PORT")]
    RemoteOutputPort,
}

/// The core data for an affected component.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedComponentDTO {
    pub active_thread_count: Option<i32>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub process_group_id: Option<String>,
    pub reference_type: Option<ReferenceTypes>,
    pub state: Option<String>,
    pub validation_errors: Option<Vec<String>>,
}

/// A DTO for a Process Group's name and ID.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupNameDTO {
    pub id: Option<String>,
    pub name: Option<String>,
}

/// Represents a full Process Group entity, often used when listing bound groups.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupEntity {
    pub active_remote_port_count: Option<i32>,
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<serde_json::Value>, // Using Value for flexibility
    pub disabled_count: Option<i32>,
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    pub inactive_remote_port_count: Option<i32>,
    pub input_port_count: Option<i32>,
    pub invalid_count: Option<i32>,
    pub local_input_port_count: Option<i32>,
    pub local_output_port_count: Option<i32>,
    pub locally_modified_and_stale_count: Option<i32>,
    pub locally_modified_count: Option<i32>,
    pub output_port_count: Option<i32>,
    pub parameter_context: Option<ParameterContextReferenceEntity>,
    pub permissions: Option<PermissionsDTO>,
    pub position: Option<PositionDTO>,
    pub process_group_update_strategy: Option<ProcessGroupUpdateStrategies>,
    pub public_input_port_count: Option<i32>,
    pub public_output_port_count: Option<i32>,
    pub revision: Option<RevisionDTO>,
    pub running_count: Option<i32>,
    pub stale_count: Option<i32>,
    pub status: Option<ProcessGroupStatusDTO>,
    pub stopped_count: Option<i32>,
    pub sync_failure_count: Option<i32>,
    pub up_to_date_count: Option<i32>,
    pub uri: Option<String>,
    pub versioned_flow_snapshot: Option<bool>,
    pub versioned_flow_state: Option<VersionedFlowStates>,
}

/// The status of a Process Group.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupStatusDTO {
    pub aggregate_snapshot: Option<serde_json::Value>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub node_snapshots: Option<serde_json::Value>,
    pub stats_last_refreshed: Option<String>,
}

/// Strategy for updating a Process Group.
#[derive(Debug, Deserialize, Serialize)]
pub enum ProcessGroupUpdateStrategies {
    #[serde(rename = "CURRENT_GROUP")]
    CurrentGroup,
    #[serde(rename = "CURRENT_GROUP_WITH_CHILDREN")]
    CurrentGroupWithChildren,
}

/// The state of a versioned flow.
#[derive(Debug, Deserialize, Serialize)]
pub enum VersionedFlowStates {
    #[serde(rename = "LOCALLY_MODIFIED")]
    LocallyModified,
    #[serde(rename = "STALE")]
    Stale,
    #[serde(rename = "LOCALLY_MODIFIED_AND_STALE")]
    LocallyModifiedAndOrStale,
    #[serde(rename = "UP_TO_DATE")]
    UpToDate,
    #[serde(rename = "SYNC_FAILURE")]
    SyncFailure,
}

/// Creates a default `ParameterContextEntity` for creating a new context.
///
/// This populates a new `RevisionDTO` with version 0 and assigns a random
/// UUID v4 as the name for the component.
impl Default for ParameterContextEntity {
    fn default() -> Self {
        Self {
            bulletins: None,
            component: Some(ParameterContextDTO {
                bound_process_groups: None,
                description: None,
                id: None,
                inherited_parameter_contexts: None,
                name: Some(uuid::Uuid::new_v4().to_string()),
                parameter_provider_configuration: None,
                parameters: None,
            }),
            disconnected_node_acknowledged: None,
            id: None,
            permissions: None,
            position: None,
            revision: Some(RevisionDTO {
                client_id: None,
                last_modifier: None,
                version: Some(0),
            }),
            uri: None,
        }
    }
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
    pub async fn get_parameter_contexts(
        &self,
        id: &str,
    ) -> anyhow::Result<ParameterContextEntity> {
        let response = self
            .client
            .get_json::<ParameterContextEntity>(
                &format!("{}/parameter-contexts/{}", self.config.api_base_url, id),
            )
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
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::access::Access;
    use tracing_test::traced_test;

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
        let fake_parameter_context = &ParameterContextEntity::default();
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
        let fake_parameter_context = &ParameterContextEntity::default();
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
        let parameter_context_to_assert = parameter_context
            .get_parameter_contexts(id.as_str())
            .await;
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
        let fake_parameter_context = &mut ParameterContextEntity::default();
        let parameter_contexts = parameter_context
            .post_parameter_contexts(fake_parameter_context)
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
}
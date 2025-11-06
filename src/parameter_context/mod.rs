use crate::common::bulletins::BulletinEntity;
use crate::common::client::HttpClient;
use crate::common::config::Config;
use crate::common::types::{PermissionsDTO, PositionDTO, RevisionDTO};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug)]
pub struct ParameterContext {
    client: Arc<HttpClient>,
    config: Arc<Config>,
}

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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextReferenceEntity {
    pub component: Option<ParameterContextReferenceDTO>,
    pub id: Option<String>,
    pub permissions: Option<PermissionsDTO>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextReferenceDTO {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderConfigurationEntity {
    pub component: Option<ParameterProviderConfigurationDTO>,
    pub id: Option<String>,
    pub permissions: Option<PermissionsDTO>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderConfigurationDTO {
    pub parameter_group_name: Option<String>,
    pub parameter_provider_id: Option<String>,
    pub parameter_provider_name: Option<String>,
    pub synchronized: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterEntity {
    pub can_write: Option<bool>,
    pub parameter: Option<ParameterDTO>,
}

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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetReferenceDTO {
    pub id: Option<String>,
    pub name: Option<String>,
}

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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupNameDTO {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupEntity {
    pub active_remote_port_count: Option<i32>,
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<serde_json::Value>,
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupStatusDTO {
    pub aggregate_snapshot: Option<serde_json::Value>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub node_snapshots: Option<serde_json::Value>,
    pub stats_last_refreshed: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ProcessGroupUpdateStrategies {
    #[serde(rename = "CURRENT_GROUP")]
    CurrentGroup,
    #[serde(rename = "CURRENT_GROUP_WITH_CHILDREN")]
    CurrentGroupWithChildren,
}

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
    /// Creates a new instance of the `Access` service.
    ///
    /// # Arguments
    ///
    /// * `client` - The shared `HttpClient` to be used for requests.
    /// * `config` - The application configuration (containing `api_base_url`, `username`, etc.).
    pub fn new(client: Arc<HttpClient>, config: Arc<Config>) -> Self {
        Self { client, config }
    }
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
    pub async fn put_parameter_contexts(
        &self,
        id: &str,
        payload: &ParameterContextEntity
    ) -> anyhow::Result<ParameterContextEntity> {
        let response = self
            .client
            .put_json::<ParameterContextEntity, ParameterContextEntity>(
                &format!("{}/parameter-contexts/{}", self.config.api_base_url, id),
                payload
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
        assert_eq!(parameter_contexts.revision.unwrap().version.unwrap(), 1, "version should be 1");
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
        let name = parameter_contexts.component.as_ref().unwrap().name.clone().unwrap();
        parameter_contexts.component.as_mut().unwrap().name = Some(format!("{} foo changed", name));
        parameter_contexts.component.as_mut().unwrap().description = Some("bar changed".to_string());
        let parameter_context_to_assert = parameter_context
            .put_parameter_contexts(
                id.as_str(),
                &parameter_contexts)
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

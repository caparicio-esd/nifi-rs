//! # Parameter Provider Controller
//!
//! This module defines the data structures and controller logic for managing
//! "Parameter Providers" within the system. It includes:
//!
//! * The main `Controller` struct for interacting with the API.
//! * Data Transfer Objects (DTOs) like `ParameterProviderEntity` and `ParameterProviderDTO`
//!     which map directly to the JSON aPI.
//! * Various supporting enums and DTOs that describe properties, allowable values,
//!     and component status.
//!
//! The primary entities are serialized and deserialized with `serde` using `camelCase`
//! conventions to match the target JSON API.

use std::collections::HashMap;
use crate::common::bulletins::BulletinEntity;
use crate::common::client::HttpClient;
use crate::common::config::Config;
use crate::common::types::{PermissionsDTO, PositionDTO, RevisionDTO};
use crate::parameter_context::{AffectedComponentEntity, AssetReferenceDTO, ParameterContextReferenceEntity};
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

/// Represents a Parameter Provider entity, which includes the component itself
/// along with its metadata, permissions, and position on the canvas.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderEntity {
    /// Any bulletins (warnings, errors) associated with this parameter provider.
    pub bulletins: Option<Vec<BulletinEntity>>,
    /// The core DTO for the parameter provider component.
    pub component: Option<ParameterProviderDTO>,
    /// The unique identifier for the parameter provider.
    pub id: Option<String>,
    /// Permissions for accessing and modifying this entity.
    pub permissions: Option<PermissionsDTO>,
    /// The position of the component on the UI canvas.
    pub position: Option<PositionDTO>,
    /// The revision information for optimistic locking.
    pub revision: Option<RevisionDTO>,
    /// The URI for this entity, for direct API access.
    pub uri: Option<String>,
}

/// The core Data Transfer Object for a Parameter Provider.
///
/// This structure contains the configuration, properties, and status
/// of a specific parameter provider component.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderDTO {
    /// Components that are affected by this parameter provider.
    pub affected_components: Option<Vec<AffectedComponentEntity>>,
    /// Any annotation data associated with the component.
    pub annotation_data: Option<String>,
    /// The bundle (group, artifact, version) that defines this component's type.
    pub bundle: Option<BundleDTO>,
    /// User-supplied comments.
    pub comments: Option<String>,
    /// An optional URL for a custom configuration UI.
    pub custom_ui_url: Option<String>,
    /// Whether this component is deprecated.
    pub deprecated: Option<bool>,
    /// Descriptions of the properties this component supports.
    pub descriptors: Option<HashMap<String, PropertyDescriptorDTO>>,
    /// Whether the extension (NAR) for this component is missing.
    pub extension_missing: Option<bool>,
    /// The unique identifier for this component.
    pub id: Option<String>,
    /// Whether multiple versions of this component's bundle are available.
    pub multiple_versions_available: Option<bool>,
    /// The user-defined name of the component.
    pub name: Option<String>,
    /// Configuration for parameter groups, mapping groups to parameter contexts.
    pub parameter_group_configurations: Option<Vec<ParameterGroupConfigurationEntity>>,
    /// The status of parameters provided by this component.
    pub parameter_status: Option<ParameterStatusDTO>,
    /// The ID of the parent process group.
    pub parent_group_id: Option<String>,
    /// Whether this component persists its state.
    pub persists_state: Option<bool>,
    /// The component's position on the canvas.
    pub position: Option<PositionDTO>,
    /// The configured properties (key-value pairs) for this component.
    pub properties: Option<HashMap<String, Option<String>>>,
    /// Parameter contexts that reference this provider.
    pub referencing_parameter_contexts: Option<Vec<ParameterProviderReferencingComponentEntity>>,
    /// Whether this component has restricted functionality.
    pub restricted: Option<bool>,
    /// The fully-qualified class name of the component's implementation.
    #[serde(rename = "type")]
    pub _type: Option<String>,
    /// Any validation errors associated with the component's configuration.
    pub validation_errors: Option<Vec<String>>,
    /// The current validation status (e.g., VALID, INVALID).
    pub validation_status: Option<ValidationStatus>,
    /// The ID of this component in a version-controlled flow.
    pub versioned_component_id: Option<String>,
}

/// Defines the configuration for a parameter group, including which
/// parameter context to use.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterGroupConfigurationEntity {
    /// The name of the parameter group.
    pub group_name: Option<String>,
    /// The name of the parameter context this group is bound to.
    pub parameter_context_name: Option<String>,
    /// Defines the sensitivity override for parameters in this group.
    pub parameter_sensitivities: Option<HashMap<String, Option<String>>>,
    /// Whether the group is synchronized.
    pub synchronized: Option<bool>,
}

/// Describes the status of a specific parameter.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterStatusDTO {
    /// The parameter entity being described.
    pub parameter: Option<ParameterEntity>,
    /// The status of the parameter (e.g., NEW, CHANGED, REMOVED).
    pub status: Option<StatusType>,
}

/// Represents a parameter, including its write permissions and core DTO.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterEntity {
    /// Whether the current user can write to (modify) this parameter.
    pub can_write: Option<bool>,
    /// The core DTO for the parameter.
    pub parameter: Option<ParameterDTO>,
}

/// The core Data Transfer Object for a Parameter.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterDTO {
    /// A description of the parameter.
    pub description: Option<String>,
    /// Whether this parameter is inherited from an upstream source.
    pub inherited: Option<bool>,
    /// The name of the parameter.
    pub name: Option<String>,
    /// A reference to the parameter context this parameter belongs to.
    pub parameter_context: Option<ParameterContextReferenceEntity>,
    /// Whether this parameter is provided (e.g., by a provider) vs. user-entered.
    pub provided: Option<bool>,
    /// A list of assets (files) referenced by this parameter.
    pub referenced_assets: Option<Vec<AssetReferenceDTO>>,
    /// A list of components that reference this parameter.
    pub referencing_components: Option<Vec<AffectedComponentEntity>>,
    /// Whether this parameter is sensitive (its value should be hidden).
    pub sensitive: Option<bool>,
    /// The value of the parameter. This may be `None` if sensitive.
    pub value: Option<String>,
    /// Whether the value was recently removed.
    pub value_removed: Option<bool>
}

/// An enum representing the status of a parameter in relation to a
/// parameter context.
#[derive(Debug, Deserialize, Serialize)]
pub enum StatusType {
    /// The parameter is new and does not exist in the context.
    #[serde(rename="NEW")]
    New,
    /// The parameter exists in the context but its value has changed.
    #[serde(rename="CHANGED")]
    Changed,
    /// The parameter has been removed.
    #[serde(rename="REMOVED")]
    Removed,
    /// The parameter is missing but is still referenced.
    #[serde(rename="MISSING_BUT_REFERENCED")]
    MissingButReferenced,
    /// The parameter exists and its value is unchanged.
    #[serde(rename="UNCHANGED")]
    Unchanged,
}

/// A Data Transfer Object representing a NiFi Archive (NAR) bundle.
///
/// This uniquely identifies a bundle of extensions (e.g., processors, services).
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleDTO {
    /// The artifact ID (e.g., "nifi-standard-nar").
    pub artifact: Option<String>,
    /// The group ID (e.g., "org.apache.nifi").
    pub group: Option<String>,
    /// The version (e.g., "2.6.0").
    pub version: Option<String>,
}

/// Describes a configurable property for a component.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDescriptorDTO {
    /// A list of allowed values for this property.
    pub allowable_values: Option<Vec<AllowableValueEntity>>,
    /// The default value for this property.
    pub default_value: Option<String>,
    /// Dependencies on other properties.
    pub dependencies: Option<Vec<PropertyDependencyDTO>>,
    /// A description of what the property is for.
    pub description: Option<String>,
    /// The user-friendly name displayed in the UI.
    pub display_name: Option<String>,
    /// Whether this property can be modified while the component is running.
    pub dynamic: Option<bool>,
    /// The scope of Expression Language (EL) supported (e.g., "FLOWFILE_ATTRIBUTES").
    pub expression_language_scope: Option<String>,
    /// If this property identifies a Controller Service, this holds its class name.
    pub identifies_controller_service: Option<String>,
    /// The bundle for the identified Controller Service.
    pub identifies_controller_service_bundle: Option<BundleDTO>,
    /// The internal, non-localized name of the property.
    pub name: Option<String>,
    /// Whether this property is required to be set.
    pub required: Option<bool>,
    /// Whether this property is sensitive (its value should be hidden).
    pub sensitive: Option<bool>,
    /// Whether this property supports Expression Language (EL).
    pub supports_el: Option<bool>
}

/// Defines a dependency one property has on another.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDependencyDTO {
    /// The values of the other property that trigger this dependency.
    pub dependant_values: Option<Vec<String>>,
    /// The name of the property this property depends on.
    pub property_name: Option<String>,
}

/// Represents an allowable value for a property, including read permissions.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllowableValueEntity {
    /// The core DTO for the allowable value.
    pub allowable_value: Option<AllowableValueDTO>,
    /// Whether the current user can read this value.
    pub can_read: Option<bool>,
}

/// The core Data Transfer Object for an allowable value.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllowableValueDTO {
    /// A description of what this value represents.
    pub description: Option<String>,
    /// The user-friendly name displayed in the UI.
    pub display_name: Option<String>,
    /// The actual value to be submitted.
    pub value: Option<String>
}

/// An enum representing the validation status of a component.
#[derive(Debug, Deserialize, Serialize)]
pub enum ValidationStatus {
    /// The component's configuration is valid.
    #[serde(rename = "VALID")]
    Valid,
    /// The component's configuration is invalid.
    #[serde(rename = "INVALID")]
    Invalid,
    /// The component is currently being validated.
    #[serde(rename = "VALIDATING")]
    Validating
}

/// Represents a component that references a parameter provider.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderReferencingComponentEntity {
    /// Bulletins associated with this referencing component.
    pub bulletins: Option<Vec<BulletinEntity>>,
    /// The core DTO of the referencing component.
    pub component: Option<ParameterProviderReferencingComponentDTO>,
    /// Whether the user has acknowledged this node is disconnected.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The unique ID of the referencing component.
    pub id: Option<String>,
    /// Permissions for this component.
    pub permissions: Option<PermissionsDTO>,
    /// The position of this component on the canvas.
    pub position: Option<PositionDTO>,
    /// The revision for this component.
    pub revision: Option<RevisionDTO>,
    /// The URI for this component.
    pub uri: Option<String>,
}

/// The core DTO for a component that references a parameter provider.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderReferencingComponentDTO {
    /// The unique ID of the component.
    pub id: Option<String>,
    /// The name of the component.
    pub name: Option<String>,
}

/// Provides a default, empty `ParameterProviderEntity`.
///
/// This is useful for building a new entity to be sent to the API.
/// It pre-populates the `revision` with a default version of `0`
/// and includes an empty `component` DTO.
impl Default for ParameterProviderEntity {
    fn default() -> Self {
        Self {
            bulletins: None,
            component: Some(ParameterProviderDTO {
                affected_components: None,
                annotation_data: None,
                bundle: None,
                comments: None,
                custom_ui_url: None,
                deprecated: None,
                descriptors: None,
                extension_missing: None,
                id: None,
                multiple_versions_available: None,
                name: None,
                parameter_group_configurations: None,
                parameter_status: None,
                parent_group_id: None,
                persists_state: None,
                position: None,
                properties: None,
                referencing_parameter_contexts: None,
                restricted: None,
                _type: None,
                validation_errors: None,
                validation_status: None,
                versioned_component_id: None,
            }),
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
                &format!("{}/controller/parameter-providers", self.config.api_base_url),
                payload,
            )
            .await?;
        Ok(response)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Arc;
    use tracing_test::traced_test;
    use crate::access::Access;
    use crate::common::client::HttpClient;
    use crate::common::config::Config;

    #[tokio::test]
    #[traced_test]
    async fn test_post_parameter_providers() {
        // --- 1. Setup ---
        let client = Arc::new(HttpClient::new());
        let config = Arc::new(Config::default()); // Assumes correct credentials
        let access = Access::new(client.clone(), config.clone());
        let _ = access.get_access_token().await;

        let controller = Controller::new(client.clone(), config.clone());
        let mut fake_parameter_provider = ParameterProviderEntity::default();
        fake_parameter_provider.component = Some(ParameterProviderDTO {
            affected_components: None,
            annotation_data: None,
            bundle: Some(BundleDTO {
                artifact: Some("nifi-standard-nar".to_string()),
                group: Some("org.apache.nifi".to_string()),
                version: Some("2.6.0".to_string()),
            }),
            comments: None,
            custom_ui_url: None,
            deprecated: None,
            descriptors: None,
            extension_missing: None,
            id: None,
            multiple_versions_available: None,
            name: Some(uuid::Uuid::new_v4().to_string()),
            parameter_group_configurations: None,
            parameter_status: None,
            parent_group_id: None,
            persists_state: None,
            position: None,
            properties: None,
            referencing_parameter_contexts: None,
            restricted: None,
            _type: Some("org.apache.nifi.parameter.EnvironmentVariableParameterProvider".to_string()),
            validation_errors: None,
            validation_status: None,
            versioned_component_id: None,
        });

        let parameter_provider_entity = controller.post_parameter_providers(&fake_parameter_provider).await;
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
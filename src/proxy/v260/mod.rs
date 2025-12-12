use crate::proxy::v260::api::{
    ExternalControllerServiceReference, ParameterProviderReference, VersionedParameterContext,
    VersionedProcessGroup,
};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

#[allow(warnings)]
pub mod api {
    include!(concat!(env!("OUT_DIR"), "/openapi_codegen.rs"));
}
pub mod access;
pub mod authentication;
pub mod controller;
pub mod flow;
pub mod parameter_context;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FlowSnapshot {
    pub flow_contents: VersionedProcessGroup,
    pub external_controller_services: HashMap<String, ExternalControllerServiceReference>,
    pub parameter_contexts: HashMap<String, VersionedParameterContext>,
    pub parameter_providers: HashMap<String, ParameterProviderReference>,
    pub flow_encoding_version: String,
    pub latest: bool,
}

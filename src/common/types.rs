use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionsDTO {
    pub can_read: Option<bool>,
    pub can_write: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevisionDTO {
    pub client_id: Option<String>,
    pub last_modifier: Option<String>,
    pub version: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionDTO {
    pub x: Option<f32>,
    pub y: Option<f32>,
}

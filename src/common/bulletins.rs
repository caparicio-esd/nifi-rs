use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulletinEntity {
    bulletin: Option<BulletinDTO>,
    can_read: Option<bool>,
    group_id: Option<String>,
    id: Option<i64>,
    node_address: Option<String>,
    source_id: Option<String>,
    timestamp: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulletinDTO {
    category: Option<String>,
    group_id: Option<String>,
    id: Option<i64>,
    level: Option<String>,
    message: Option<String>,
    node_address: Option<String>,
    source_id: Option<String>,
    source_name: Option<String>,
    source_type: Option<String>,
    timestamp: Option<String>,
}

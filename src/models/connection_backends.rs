//
// /api/monitoring/connections/backends
//

pub struct BackendConnections {
    pub url: String,
    pub payload: Option<BackendConnectionsRoot>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackendConnectionsRoot {
    pub subaccounts: Vec<Subaccount>,
    pub version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subaccount {
    pub backend_connections: Vec<BackendConnection>,
    pub region_host: String,
    pub subaccount: String,
    #[serde(rename = "locationID")]
    pub location_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackendConnection {
    pub virtual_backend: String,
    pub internal_backend: String,
    pub protocol: String,
    pub idle: i64,
    pub active: i64,
}

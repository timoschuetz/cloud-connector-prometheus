//
// Models for /api/monitoring/subaccounts
//

pub enum SubaccountStatus {
    Exists,
    Error,
}
pub struct SubaccountResponse {
    pub status: SubaccountStatus,
    pub url: String,
    pub payload: Option<MonitSub>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitSub {
    pub subaccounts: Vec<MSubaccount>,
    pub version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MSubaccount {
    pub description: String,
    pub display_name: String,
    pub tunnel: Tunnel,
    pub region_host: String,
    pub subaccount: String,
    #[serde(rename = "locationID")]
    pub location_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SCertificate {
    pub not_after_time_stamp: i64,
    pub not_before_time_stamp: i64,
    pub subject_d_n: String,
    pub issuer: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tunnel {
    pub state: String,
    pub connected_since_time_stamp: i64,
    pub connections: i64,
    pub application_connections: Vec<ApplicationConnection>,
    pub service_channels: Vec<::serde_json::Value>,
    pub user: String,
    pub subaccount_certificate: SCertificate,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationConnection {
    pub connection_count: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

//
// /api/monitoring/performance/toptimeconsumers
//

pub struct TopTimeConsumers {
    pub url: String,
    pub payload: Option<Root>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub subaccounts: Vec<Subaccount>,
    pub version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subaccount {
    pub since_time: String,
    pub requests: Vec<Request>,
    pub region_host: String,
    pub subaccount: String,
    #[serde(rename = "locationID")]
    pub location_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub start_time: String,
    pub id: i64,
    pub protocol: String,
    pub virtual_backend: String,
    pub internal_backend: String,
    pub resource: String,
    pub sent_bytes: i64,
    pub received_bytes: i64,
    pub user: Option<String>,
    pub total_time: i64,
    pub external_time: i64,
    pub gen_sso_time: i64,
    pub open_remote_time: i64,
    pub validate_sso_time: i64,
    pub latency_time: i64,
}
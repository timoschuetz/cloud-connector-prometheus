//
// /api/monitoring/performance/backends
//

pub struct MBackendPerformance {
    pub url: String,
    pub payload: Option<BackendPerformanceRoot>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackendPerformanceRoot {
    pub subaccounts: Vec<Subaccount>,
    pub version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subaccount {
    pub backend_performance: Vec<BackendPerformance>,
    pub since_time: String,
    pub region_host: String,
    pub subaccount: String,
    #[serde(rename = "locationID")]
    pub location_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackendPerformance {
    pub virtual_host: String,
    pub virtual_port: String,
    pub protocol: String,
    pub buckets: Vec<Bucket>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bucket {
    pub number_of_calls: i64,
    pub minimum_call_duration_ms: i64,
}
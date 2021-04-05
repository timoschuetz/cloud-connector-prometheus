//
// /api/monitoring/memory
//

pub struct Memory {
    pub memory_info: Option<Root>,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "physicalKB")]
    pub physical_kb: PhysicalKb,
    #[serde(rename = "virtualKB")]
    pub virtual_kb: VirtualKb,
    #[serde(rename = "cloudConnectorHeapKB")]
    pub cloud_connector_heap_kb: CloudConnectorHeapKb,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalKb {
    pub total: i64,
    #[serde(rename = "CloudConnector")]
    pub cloud_connector: i64,
    pub others: i64,
    pub free: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualKb {
    pub total: i64,
    #[serde(rename = "CloudConnector")]
    pub cloud_connector: i64,
    pub others: i64,
    pub free: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudConnectorHeapKb {
    pub total: i64,
    pub used: i64,
    pub free: i64,
}
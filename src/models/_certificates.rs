//
// /api/monitoring/certificates
//

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub expired: Vec<::serde_json::Value>,
    pub expiring: Vec<::serde_json::Value>,
    pub ok: Vec<Ok>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ok {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "subjectDN")]
    pub subject_dn: Option<String>,
    pub valid_to: i64,
    pub subaccount_name: Option<String>,
    pub subaccount_region: Option<String>,
}

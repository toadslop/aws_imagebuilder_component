use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct ExecuteDocument {
    document: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    document_s3_bucket_owner: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phases: Option<String>,
    parameters: Vec<Parameter>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct Parameter {
    name: String,
    value: String,
}

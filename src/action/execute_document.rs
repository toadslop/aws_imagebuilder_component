use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct ExecuteDocument {
    document: String,
    document_s3_bucket_owner: Option<u64>,
    phases: Option<String>,
    parameters: Vec<Parameter>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct Parameter {
    name: String,
    value: String,
}

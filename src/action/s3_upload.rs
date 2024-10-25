use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct S3Upload {
    source: String,
    destination: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expected_bucket_owner: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recurse: Option<bool>,
}

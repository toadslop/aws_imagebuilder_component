use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct S3Download {
    source: String,
    destination: String,
    expected_bucket_owner: Option<String>,
    overwrite: Option<bool>,
}

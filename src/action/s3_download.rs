use super::file_system::MoveOperation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct S3Download {
    #[serde(flatten)]
    move_operation: MoveOperation,
    #[serde(skip_serializing_if = "Option::is_none")]
    expected_bucket_owner: Option<u64>,
}

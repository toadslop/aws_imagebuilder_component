use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct MoveOperation {
    source: String,
    destination: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    overwrite: Option<bool>,
}
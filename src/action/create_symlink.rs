use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct CreateSymlink {
    path: String,
    target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    force: Option<bool>,
}

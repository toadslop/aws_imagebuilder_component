use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct DeleteFolder {
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    force: Option<bool>,
}

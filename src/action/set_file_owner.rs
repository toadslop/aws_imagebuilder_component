use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct SetFileOwner {
    path: String,
    owner: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<String>,
}

use super::file_system::Permissions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct CreateFolder {
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    overwrite: Option<bool>,
    #[serde(flatten)]
    permissions: Permissions,
}

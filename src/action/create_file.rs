use super::{file_system::Permissions, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct CreateFile {
    #[serde(flatten)]
    file: File,
    #[serde(flatten)]
    permissions: Permissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    overwrite: Option<bool>,
}

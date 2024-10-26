use crate::misc::StringOrNumber;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetFolderPermissions {
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<StringOrNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recursive: Option<bool>,
}

use serde::{Deserialize, Serialize};

use crate::misc::Permissions;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetFolderPermissions {
    path: String,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    recursive: Option<bool>,
}

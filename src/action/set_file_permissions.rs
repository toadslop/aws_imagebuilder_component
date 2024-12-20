use crate::misc::Permissions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetFilePermissions {
    path: String,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    permissions: Option<Permissions>,
}

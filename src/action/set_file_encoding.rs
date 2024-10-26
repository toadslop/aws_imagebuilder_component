use super::file_system::Encoding;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetFileEncoding {
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<Encoding>,
}

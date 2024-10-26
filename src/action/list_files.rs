use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct ListFiles {
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_name_pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recursive: Option<bool>,
}

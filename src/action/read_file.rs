use super::file_system::Encoding;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct ReadFile {
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<Encoding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    print_file_content: Option<bool>,
}

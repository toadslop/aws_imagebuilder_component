use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct ExecuteBinary {
    path: String,
    arguments: Vec<String>,
}

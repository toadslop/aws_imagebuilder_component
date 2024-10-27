use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SetRegistry {
    path: String,
    name: String,
    r#type: RegistryType,
    value: RegistryValue,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RegistryType {
    Binary,
    Dword,
    Qword,
    Sz,
    ExpandSz,
    MultiSz,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum RegistryValue {
    Float(f64),
    String(String),
    Strings(Vec<String>),
    Floats(Vec<f64>),
}

use either::Either;
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
pub enum RegistryValue {
    Number(f64),
    String(String),
    Array(Vec<Either<String, f64>>),
}

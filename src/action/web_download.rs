use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebDownload {
    source: String,
    destination: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    overwrite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    checksum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm: Option<Algorithm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_certificate_errors: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Algorithm {
    MD5,
    SHA1,
    SHA256,
    SHA512,
}
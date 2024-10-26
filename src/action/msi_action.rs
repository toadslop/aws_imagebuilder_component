use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MsiAction {
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    reboot: Option<Reboot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_options: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_authenticode_signature_errors: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_unsigned_installer: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub enum Reboot {
    #[default]
    Allow,
    Force,
    Skip,
}

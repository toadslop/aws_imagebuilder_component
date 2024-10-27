use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Reboot {
    #[serde(skip_serializing_if = "Option::is_none")]
    delay_seconds: Option<u32>,
}

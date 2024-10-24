use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Loop {
    name: String,
    #[serde(flatten)]
    loop_type: LoopType,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum LoopType {
    #[serde(rename_all = "camelCase")]
    For {
        start: u32,
        end: u32,
        update_by: u32,
    },
    ForEach(ForEach),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum ForEach {
    List(Vec<String>),
    Delimiter { list: String, delimiter: String },
}

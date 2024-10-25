use conditional::If;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod action;
pub mod conditional;
pub mod r#loop;

pub use action::Action;
pub use conditional::Expression;
pub use r#loop::{Loop, LoopType};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub schema_version: SchemaVersion,
    pub phases: Vec<Phase>,
    #[serde(default)]
    pub parameters: Vec<HashMap<String, Parameter>>,
    #[serde(default)]
    pub constants: Vec<HashMap<String, Constant>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct Parameter {
    r#type: ParamType,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct Constant {
    r#type: ParamType,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum ParamType {
    #[default]
    String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub enum SchemaVersion {
    #[serde(rename = "1.0")]
    #[default]
    OneDotZero,

    Other(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Phase {
    name: PhaseName,
    steps: Vec<Step>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum PhaseName {
    Build,
    Validate,
    Test,
    Other(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Step {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_seconds: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_failure: Option<FailurePolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_attempts: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#if: Option<If>,
    #[serde(flatten)]
    action: Action,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#loop: Option<Loop>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub enum FailurePolicy {
    #[default]
    Abort,
    Continue,
    Ignore,
}

#[cfg(test)]
mod test {
    use crate::Component;
    use std::fs::{read_dir, read_to_string};

    #[test]
    fn test() {
        for entry in read_dir("./test").expect("Failed to read the tests folder") {
            let entry = entry.expect("failed to load the entry");
            let yaml = read_to_string(entry.path())
                .unwrap_or_else(|e| panic!("Failed to read {:?}: {e}", entry.file_name()));
            let component: Component = serde_yaml::from_str(&yaml)
                .unwrap_or_else(|e| panic!("Failed to deserialize {:?}: {e}", entry.file_name()));

            let serialized = serde_yaml::to_string(&component)
                .unwrap_or_else(|e| panic!("Failed to serialize {:?}: {e}", entry.file_name()));

            println!("\n{serialized}\n");

            if serialized.contains("null") {
                panic!("Should not serialize 'None'")
            }
        }
    }
}

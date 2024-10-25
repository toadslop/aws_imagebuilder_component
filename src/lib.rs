use conditional::If;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

mod action;
mod conditional;
mod r#loop;

pub use action::Action;
pub use conditional::Conditional;
pub use r#loop::{Loop, LoopType};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub name: Option<String>,
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
    default: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct Constant {
    r#type: ParamType,
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
    timeout_seconds: Option<u32>,
    on_failure: Option<FailurePolicy>,
    max_attempts: Option<u32>,
    r#if: Option<If>,
    #[serde(flatten)]
    action: Action,
    r#loop: Option<Loop>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub enum FailurePolicy {
    #[default]
    Abort,
    Continue,
    Ignore,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct MaxAttempts(u32);

impl MaxAttempts {
    pub fn new(inner: u32) -> Self {
        Self(inner)
    }
}

impl Default for MaxAttempts {
    fn default() -> Self {
        Self(1)
    }
}

impl Deref for MaxAttempts {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MaxAttempts {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<u32> for MaxAttempts {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

impl AsMut<u32> for MaxAttempts {
    fn as_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
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
            let _: Component = serde_yaml::from_str(&yaml)
                .unwrap_or_else(|e| panic!("Failed to deserialize {:?}: {e}", entry.file_name()));
        }
    }
}

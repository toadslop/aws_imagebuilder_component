use action::Action;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

mod action;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub name: Option<String>,
    pub description: Option<String>,
    pub schema_version: SchemaVersion,
    pub phases: Vec<Phase>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub enum SchemaVersion {
    #[serde(rename = "1.0")]
    #[default]
    OneDotZero,

    Other(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Step {
    name: String,
    timeout_seconds: Option<u32>,
    on_failure: Option<FailurePolicy>,
    max_attempts: Option<u32>,
    #[serde(flatten)]
    action: Action,
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
    use std::fs::read_to_string;

    use crate::Component;

    #[test]
    fn test() {
        let test_yaml = read_to_string("./test/test.yaml").expect("Failed to load test file.");
        let it: Component =
            serde_yaml::from_str(&test_yaml).expect("Failed to deserialize test file");

        println!("{it:?}");
    }
}

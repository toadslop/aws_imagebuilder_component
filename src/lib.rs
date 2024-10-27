use conditional::Conditional;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod action;
pub mod conditional;
pub mod r#loop;
pub mod misc;

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
    r#if: Option<Conditional>,
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

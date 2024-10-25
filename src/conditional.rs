use serde::{Deserialize, Serialize};

mod comparison;
mod logical;

pub use comparison::Comparison;
pub use logical::Logical;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct If {
    #[serde(flatten)]
    expression: Expression,
    #[serde(skip_serializing_if = "Option::is_none")]
    then: Option<StepAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#else: Option<StepAction>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum Expression {
    Logical(Logical),
    Comparison(Comparison),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum StepAction {
    Abort,
    Execute,
    Skip,
}

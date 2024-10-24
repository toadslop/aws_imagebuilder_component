use serde::{Deserialize, Serialize};

mod comparison;
mod logical;

pub use comparison::Comparison;
pub use logical::Logical;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct If {
    #[serde(flatten)]
    expression: Conditional,
    then: Option<StepAction>,
    r#else: Option<StepAction>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum Conditional {
    Logical(Logical),
    Comparison(Comparison),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum StepAction {
    Abort,
    Execute,
    Skip,
}

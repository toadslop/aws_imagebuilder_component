use serde::{Deserialize, Serialize};

use super::{comparison::Comparison, Conditional};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Logical {
    And(Vec<Conditional>),
    Or(Vec<Conditional>),
    Not(Comparison),
}

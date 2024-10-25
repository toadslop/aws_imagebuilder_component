use serde::{Deserialize, Serialize};

use super::{comparison::Comparison, Expression};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Logical {
    And(Vec<Expression>),
    Or(Vec<Expression>),
    Not(Comparison),
}

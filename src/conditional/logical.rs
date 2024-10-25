use super::{comparison::Comparison, Expression};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Logical {
    And(Vec<Expression>),
    Or(Vec<Expression>),
    Not(Comparison),
}

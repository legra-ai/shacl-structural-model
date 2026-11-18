//! Human-readable expression labels, not RDF serializations.

use super::{FunctionName, NodeExpression};
use crate::{local_name, vocabulary};

impl<Value: std::fmt::Debug, Named: AsRef<str>> NodeExpression<Value, Named> {
    /// A constant's term, or otherwise the source node or function local name.
    #[must_use]
    pub fn display(&self) -> String {
        if let Some(value) = self.as_constant().or(self.source()) {
            return format!("{value:?}");
        }
        match self.function_name() {
            FunctionName::BuiltIn(name) => local_name(name),
            FunctionName::ListParameter(function) => local_name(function),
            FunctionName::Constant | FunctionName::NamedParameter => {
                local_name(vocabulary::NAMED_PARAMETER_EXPRESSION)
            }
        }
    }
}

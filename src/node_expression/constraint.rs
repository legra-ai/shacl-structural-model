//! The `sh:expression` constraint (Node Expressions section 7.1).

use iri_rfc3987::Iri;

use super::NodeExpression;

/// A `sh:expression` constraint with expression-specific messages.
///
/// Each value node is valid when the expression yields exactly `( true )`
/// with the value node bound to the scope variable `value`. Evaluation and
/// report construction are consumer-owned.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Value: serde::Serialize, Named: serde::Serialize",
        deserialize = "Value: serde::Deserialize<'de>, Named: serde::Deserialize<'de>"
    )
)]
pub struct ExpressionConstraint<Value = crate::DefaultTerm, Named = Iri<String>> {
    /// The expression; its retained source supports `sh:sourceConstraint`.
    pub expression: NodeExpression<Value, Named>,
    /// RDF `sh:message` literals attached to the expression.
    pub messages: Vec<Value>,
}

impl<Value, Named> ExpressionConstraint<Value, Named> {
    /// Create a constraint without messages.
    #[must_use]
    pub fn new(expression: NodeExpression<Value, Named>) -> Self {
        Self {
            expression,
            messages: Vec::new(),
        }
    }
}

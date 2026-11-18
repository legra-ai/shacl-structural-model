//! Deactivation of individual constraints through reifiers (Core section 3.1.6).

use iri_rfc3987::Iri;

use crate::NodeExpression;

/// A `sh:deactivated` reifier on one constraint parameter triple.
///
/// SHACL 1.2 Core, Working Draft 16 November 2026, section 3.1.6: a triple
/// with a shape as subject and a parameter (such as `sh:minCount`) as
/// predicate can have one reifier with a `sh:deactivated` node expression.
/// The record names that triple by its three positions, so a consumer can
/// stream it beside the shape. The consumer evaluates the expression and
/// decides which constraints use the triple.
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
pub struct ConstraintDeactivation<Value = crate::DefaultTerm, Named = Iri<String>> {
    /// The shape that is the subject of the reified triple.
    pub shape: Value,
    /// The parameter predicate of the reified triple.
    pub parameter: Named,
    /// The parameter value that is the object of the reified triple.
    pub value: Value,
    /// The reifier's `sh:deactivated` node expression.
    pub deactivated: NodeExpression<Value, Named>,
}

impl<Value, Named> ConstraintDeactivation<Value, Named> {
    /// Deactivate the constraint triple `shape parameter value`.
    #[must_use]
    pub const fn new(
        shape: Value,
        parameter: Named,
        value: Value,
        deactivated: NodeExpression<Value, Named>,
    ) -> Self {
        Self {
            shape,
            parameter,
            value,
            deactivated,
        }
    }
}

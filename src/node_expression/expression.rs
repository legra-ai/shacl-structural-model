//! The checked node-expression tree.

use iri_rfc3987::Iri;

use super::{
    ExpressionError, ExpressionForm, Footprint, FunctionName, OutputContract, VariableName,
};

/// A SHACL 1.2 node expression (Node Expressions, Working Draft 16 November 2026).
///
/// Every nested expression was checked when it was built, and the tree's
/// depth and node count are bounded (see [`NodeExpression::MAX_DEPTH`] and
/// [`NodeExpression::MAX_NODES`]). Deserialization applies the same checks
/// under [`crate::DecodeLimits`]. Nothing is evaluated here.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize)]
#[serde(bound(serialize = "Value: serde::Serialize, Named: serde::Serialize"))]
pub struct NodeExpression<Value = crate::DefaultTerm, Named = Iri<String>> {
    source: Option<Value>,
    form: ExpressionForm<Value, Named>,
    #[serde(skip_serializing)]
    footprint: Footprint,
}

impl<Value, Named> NodeExpression<Value, Named> {
    /// The maximum nesting depth of expressions and property paths.
    pub const MAX_DEPTH: usize = Footprint::MAX_DEPTH;
    /// The maximum number of expression, path and list-member nodes.
    pub const MAX_NODES: usize = Footprint::MAX_NODES;

    /// Check one expression over already checked nested expressions.
    ///
    /// # Errors
    /// Rejects sequence or alternative paths with fewer than two members, a
    /// custom named parameter expression without arguments, and trees above
    /// [`Self::MAX_DEPTH`] or [`Self::MAX_NODES`].
    pub fn try_new(form: ExpressionForm<Value, Named>) -> Result<Self, ExpressionError> {
        let footprint = Footprint::of(&form)?;
        Ok(Self {
            source: None,
            form,
            footprint,
        })
    }

    /// A constant IRI, literal or triple-term expression.
    #[must_use]
    pub fn constant(value: Value) -> Self {
        Self {
            source: None,
            form: ExpressionForm::Constant(value),
            footprint: Footprint::LEAF,
        }
    }

    /// The `[ shnex:var "focusNode" ]` expression.
    #[must_use]
    pub fn focus_node() -> Self {
        Self {
            source: None,
            form: ExpressionForm::Var(VariableName::focus_node()),
            footprint: Footprint::LEAF,
        }
    }

    /// Retain the blank node that represents this expression in its source graph.
    ///
    /// Constants are their own node and normally have no separate source.
    #[must_use]
    pub fn with_source(mut self, source: Value) -> Self {
        self.source = Some(source);
        self
    }

    /// The retained source node, for result reporting (`sh:sourceConstraint`).
    #[must_use]
    pub const fn source(&self) -> Option<&Value> {
        self.source.as_ref()
    }

    /// The expression's function and operands.
    #[must_use]
    pub const fn form(&self) -> &ExpressionForm<Value, Named> {
        &self.form
    }

    /// Take the source node and form apart.
    #[must_use]
    pub fn into_parts(self) -> (Option<Value>, ExpressionForm<Value, Named>) {
        (self.source, self.form)
    }

    /// The constant term, when this is a constant expression.
    #[must_use]
    pub const fn as_constant(&self) -> Option<&Value> {
        match &self.form {
            ExpressionForm::Constant(value) => Some(value),
            _ => None,
        }
    }

    /// The function name of this expression.
    #[must_use]
    pub fn function_name(&self) -> FunctionName<'_, Named> {
        self.form.function_name()
    }

    /// The output-sequence contract of this expression's function.
    #[must_use]
    pub const fn output_contract(&self) -> OutputContract {
        self.form.output_contract()
    }

    /// The nesting depth, counting this expression and nested paths.
    #[must_use]
    pub const fn depth(&self) -> usize {
        self.footprint.depth()
    }

    /// The number of expression, path and list-member nodes in the tree.
    #[must_use]
    pub const fn node_count(&self) -> usize {
        self.footprint.nodes()
    }

    pub(super) const fn footprint(&self) -> Footprint {
        self.footprint
    }
}

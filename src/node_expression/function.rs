//! Custom node-expression function declarations (Node Expressions section 6).

use iri_rfc3987::Iri;

use super::NodeExpression;
use crate::{Parameter, vocabulary};

/// How a custom function receives its arguments.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum ParameterStyle {
    /// `sh:NamedParameterExpressionFunction` (6.1): keyed by parameter paths,
    /// at least one of which has `sh:keyParameter true`.
    Named,
    /// `sh:ListParameterExpressionFunction` (6.2): positional arguments, read
    /// with `[ shnex:arg n ]`; the function IRI is its list parameter property.
    List,
}

impl ParameterStyle {
    /// The function class IRI.
    #[must_use]
    pub const fn class_iri(self) -> &'static str {
        match self {
            Self::Named => vocabulary::NAMED_PARAMETER_EXPRESSION_FUNCTION,
            Self::List => vocabulary::LIST_PARAMETER_EXPRESSION_FUNCTION,
        }
    }

    /// The class of expressions that call a function of this style.
    #[must_use]
    pub const fn expression_class_iri(self) -> &'static str {
        match self {
            Self::Named => vocabulary::NAMED_PARAMETER_EXPRESSION,
            Self::List => vocabulary::LIST_PARAMETER_EXPRESSION,
        }
    }
}

/// A custom node-expression function: an IRI with a `sh:bodyExpression`.
///
/// Built-in functions are declared without a body and are not custom
/// functions. Key-parameter disjointness, scope construction and evaluation
/// are consumer-owned.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Value: serde::Serialize, Count: serde::Serialize, Flags: serde::Serialize, Pattern: serde::Serialize, Named: serde::Serialize",
        deserialize = "Value: serde::Deserialize<'de>, Count: serde::Deserialize<'de>, Flags: serde::Deserialize<'de>, Pattern: serde::Deserialize<'de>, Named: serde::Deserialize<'de>"
    )
)]
pub struct ExpressionFunction<
    Value = crate::DefaultTerm,
    Count = Value,
    Flags = Value,
    Pattern = Value,
    Named = Iri<String>,
> {
    /// The function IRI.
    pub id: Named,
    /// Named or list parameter style.
    pub style: ParameterStyle,
    /// `sh:parameter` declarations in caller order.
    pub parameters: Vec<Parameter<Value, Count, Flags, Pattern, Named>>,
    /// The single `sh:bodyExpression`.
    pub body: NodeExpression<Value, Named>,
}

impl<Value, Count, Flags, Pattern, Named> ExpressionFunction<Value, Count, Flags, Pattern, Named> {
    /// Create a function declaration without parameters.
    #[must_use]
    pub fn new(id: Named, style: ParameterStyle, body: NodeExpression<Value, Named>) -> Self {
        Self {
            id,
            style,
            parameters: Vec::new(),
            body,
        }
    }

    /// Add a parameter declaration.
    #[must_use]
    pub fn with_parameter(
        mut self,
        parameter: Parameter<Value, Count, Flags, Pattern, Named>,
    ) -> Self {
        self.parameters.push(parameter);
        self
    }

    /// Iterate over the parameters declared with `sh:keyParameter true`.
    pub fn key_parameters(
        &self,
    ) -> impl Iterator<Item = &Parameter<Value, Count, Flags, Pattern, Named>> + '_ {
        self.parameters
            .iter()
            .filter(|parameter| parameter.key_parameter)
    }
}

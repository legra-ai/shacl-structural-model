//! Typed operands shared by the SHACL 1.2 node-expression forms.

use super::ExpressionError;

/// A checked `shnex:var` name (Node Expressions section 4.1.2).
///
/// The draft constrains it to an `xsd:string` with `sh:minLength 1`; the
/// lexical form is retained exactly.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(try_from = "String", into = "String")]
pub struct VariableName(String);

impl VariableName {
    /// The reserved name that selects the evaluation's focus node.
    pub const FOCUS_NODE: &str = "focusNode";

    /// Check a variable name.
    ///
    /// # Errors
    /// Returns [`ExpressionError::EmptyVariableName`] for the empty string.
    pub fn try_new(name: impl Into<String>) -> Result<Self, ExpressionError> {
        let name = name.into();
        if name.is_empty() {
            return Err(ExpressionError::EmptyVariableName);
        }
        Ok(Self(name))
    }

    /// The `"focusNode"` variable, which replaces the AF `sh:this` expression.
    #[must_use]
    pub fn focus_node() -> Self {
        Self(Self::FOCUS_NODE.to_owned())
    }

    /// Whether this name selects the focus node rather than a scope entry.
    #[must_use]
    pub fn is_focus_node(&self) -> bool {
        self.0 == Self::FOCUS_NODE
    }

    /// The exact lexical name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for VariableName {
    type Error = ExpressionError;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        Self::try_new(name)
    }
}

impl From<VariableName> for String {
    fn from(name: VariableName) -> Self {
        name.0
    }
}

/// The constant members of a list expression (Node Expressions section 4.1.3).
///
/// The `rdf:first`/`rdf:rest` shape makes the draft's "at least one member"
/// rule structural. Members are constants (IRIs, literals or triple terms),
/// which the consumer checks in its own term type.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct ConstantList<Value> {
    /// The `rdf:first` member.
    pub first: Value,
    /// The members of `rdf:rest`, in list order.
    pub rest: Vec<Value>,
}

impl<Value> ConstantList<Value> {
    /// Create a list from its first member and the remaining members.
    #[must_use]
    pub fn new(first: Value, rest: Vec<Value>) -> Self {
        Self { first, rest }
    }

    /// The number of members, always at least one.
    #[must_use]
    pub fn len(&self) -> usize {
        self.rest.len() + 1
    }

    /// Always `false`; provided for API symmetry with [`ConstantList::len`].
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }

    /// Iterate over the members in list order.
    pub fn iter(&self) -> impl Iterator<Item = &Value> + '_ {
        std::iter::once(&self.first).chain(self.rest.iter())
    }
}

/// The branches of an if expression (Node Expressions section 4.1.6).
///
/// At least one of `shnex:then` and `shnex:else` is required, so the
/// branch-free combination is unrepresentable.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub enum IfBranches<Child> {
    /// Only `shnex:then`; a false condition yields the empty list.
    Then(Child),
    /// Only `shnex:else`; a true condition yields the empty list.
    Else(Child),
    /// Both branches.
    Both {
        /// The `shnex:then` branch.
        then: Child,
        /// The `shnex:else` branch.
        otherwise: Child,
    },
}

impl<Child> IfBranches<Child> {
    /// The `shnex:then` branch, when declared.
    #[must_use]
    pub const fn then(&self) -> Option<&Child> {
        match self {
            Self::Then(then) | Self::Both { then, .. } => Some(then),
            Self::Else(_) => None,
        }
    }

    /// The `shnex:else` branch, when declared.
    #[must_use]
    pub const fn otherwise(&self) -> Option<&Child> {
        match self {
            Self::Else(otherwise) | Self::Both { otherwise, .. } => Some(otherwise),
            Self::Then(_) => None,
        }
    }

    pub(super) fn try_map<Mapped, Error>(
        self,
        mut map: impl FnMut(Child) -> Result<Mapped, Error>,
    ) -> Result<IfBranches<Mapped>, Error> {
        Ok(match self {
            Self::Then(then) => IfBranches::Then(map(then)?),
            Self::Else(otherwise) => IfBranches::Else(map(otherwise)?),
            Self::Both { then, otherwise } => IfBranches::Both {
                then: map(then)?,
                otherwise: map(otherwise)?,
            },
        })
    }
}

/// One keyed argument of a custom named parameter expression (section 6.1).
///
/// The argument is an unevaluated node expression, as the draft passes it on.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct NamedArgument<Named, Child> {
    /// The parameter predicate (the parameter's `sh:path`).
    pub parameter: Named,
    /// The argument node expression.
    pub value: Child,
}

/// The `shnex:arg` key of an arg expression (Node Expressions section 6.3).
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub enum ArgumentKey<Value, Named> {
    /// A named parameter's `sh:path` IRI, for custom named parameter functions.
    Parameter(Named),
    /// The caller's `xsd:integer` literal, a zero-based list-argument position.
    Position(Value),
}

/// The aggregation expressions of Node Expressions section 4.4.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum Aggregate {
    /// `shnex:count` (4.4.1): exactly one `xsd:integer`.
    Count,
    /// `shnex:min` (4.4.2): SPARQL `MIN`, or no output for empty input.
    Min,
    /// `shnex:max` (4.4.3): SPARQL `MAX`, or no output for empty input.
    Max,
    /// `shnex:sum` (4.4.4): SPARQL `SUM`, `0` for empty input.
    Sum,
}

/// The query literal of a SHACL-SPARQL select or SPARQL expr expression.
///
/// SHACL 1.2 SPARQL Extensions, Working Draft 22 October 2026, sections 6.1
/// and 6.2 allow at most one `sh:prefixes` value. The literal is not parsed.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct SparqlExpressionText<Value> {
    /// The `sh:select` or `sh:sparqlExpr` `xsd:string` literal.
    pub text: Value,
    /// The optional IRI or blank-node `sh:prefixes` reference.
    pub prefixes: Option<Value>,
}

impl<Value> SparqlExpressionText<Value> {
    /// Create query text without a prefixes reference.
    #[must_use]
    pub fn new(text: Value) -> Self {
        Self {
            text,
            prefixes: None,
        }
    }

    /// Set the `sh:prefixes` reference without resolving it.
    #[must_use]
    pub fn with_prefixes(mut self, prefixes: Value) -> Self {
        self.prefixes = Some(prefixes);
        self
    }
}

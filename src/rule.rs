//! SHACL Advanced Features rule declarations.

use iri_rfc3987::Iri;

use crate::{NodeExpression, SparqlQuery};

/// A SHACL-AF rule with its identity, execution metadata, and declared bodies.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShaclRule<Value = crate::DefaultTerm, Named = Iri<String>> {
    /// The rule's IRI or blank node.
    pub id: Value,
    /// The original numeric `sh:order` literal.
    pub order: Option<Value>,
    /// Whether the rule is deactivated.
    pub deactivated: bool,
    /// IRI or blank-node references to `sh:condition` shapes.
    pub conditions: Vec<Value>,
    /// Supported rule bodies. A resource may declare multiple rule types.
    pub kinds: Vec<RuleKind<Value, Named>>,
}

impl<Value, Named> ShaclRule<Value, Named> {
    /// Create an active rule with one body and no order or conditions.
    #[must_use]
    pub fn new(id: Value, kind: RuleKind<Value, Named>) -> Self {
        Self {
            id,
            order: None,
            deactivated: false,
            conditions: Vec::new(),
            kinds: vec![kind],
        }
    }
}

/// A declarative rule body; no query or node expression is executed here.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum RuleKind<Value = crate::DefaultTerm, Named = Iri<String>> {
    /// A `sh:SPARQLRule` with its CONSTRUCT query declaration.
    Sparql(SparqlQuery<Value>),
    /// A `sh:TripleRule` with three node expressions.
    Triple(Box<TripleRule<Value, Named>>),
}

/// The three expressions of a `sh:TripleRule`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TripleRule<Value = crate::DefaultTerm, Named = Iri<String>> {
    /// The `sh:subject` expression.
    pub subject: NodeExpression<Value, Named>,
    /// The `sh:predicate` expression.
    pub predicate: NodeExpression<Value, Named>,
    /// The `sh:object` expression.
    pub object: NodeExpression<Value, Named>,
}

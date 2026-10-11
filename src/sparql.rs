//! SPARQL declarations. Queries and prefix imports are never executed here.

use iri_rfc3987::Iri;

/// A query literal and its `sh:prefixes` references.
///
/// Consumers resolve prefix resources and parse the original literal in their
/// own engine. Its lexical form and datatype are not inspected here.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SparqlQuery<Value = crate::DefaultTerm> {
    /// The original `sh:select`, `sh:ask`, or `sh:construct` literal.
    pub text: Value,
    /// IRI or blank-node references to prefix declaration resources.
    pub prefixes: Vec<Value>,
}

impl<Value> SparqlQuery<Value> {
    /// Create a query with no prefix-resource references.
    #[must_use]
    pub fn new(text: Value) -> Self {
        Self {
            text,
            prefixes: Vec::new(),
        }
    }

    /// Add a `sh:prefixes` resource reference without resolving it.
    #[must_use]
    pub fn with_prefixes(mut self, resource: Value) -> Self {
        self.prefixes.push(resource);
        self
    }
}

/// The query forms supported by SPARQL validators and functions.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum SparqlSelectOrAsk<Value = crate::DefaultTerm> {
    /// A `sh:select` declaration.
    Select(SparqlQuery<Value>),
    /// A `sh:ask` declaration.
    Ask(SparqlQuery<Value>),
}

/// One `sh:declare` resource, retaining the original RDF literals.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrefixDeclaration<Value = crate::DefaultTerm> {
    /// The declaration's IRI or blank node.
    pub id: Value,
    /// The `sh:prefix` literal.
    pub prefix: Value,
    /// The `sh:namespace` literal, normally an `xsd:anyURI`.
    pub namespace: Value,
}

/// The prefix declarations and import edges attached to one resource.
///
/// Retaining edges lets callers detect conflicts and cycles without this model
/// loading or traversing a graph. This is not a resolved prefix map.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrefixDeclarations<Value = crate::DefaultTerm> {
    /// The resource containing `sh:declare` and/or `owl:imports` triples.
    pub id: Value,
    /// Direct `sh:declare` values.
    pub declarations: Vec<PrefixDeclaration<Value>>,
    /// Direct `owl:imports` references.
    pub imports: Vec<Value>,
}

/// An AF `sh:resultAnnotation` declaration on a constraint or validator.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResultAnnotation<Value = crate::DefaultTerm> {
    /// The annotation resource's IRI or blank node.
    pub id: Value,
    /// The `sh:annotationProperty` IRI.
    pub property: Iri<String>,
    /// The optional `sh:annotationVarName` literal.
    pub variable: Option<Value>,
    /// Constant `sh:annotationValue` terms.
    pub values: Vec<Value>,
}

/// A `sh:sparql` constraint, including its source identity and messages.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SparqlConstraint<Value = crate::DefaultTerm> {
    /// The constraint's IRI or blank node (the result's source constraint).
    pub id: Value,
    /// The SELECT query declaration.
    pub select: SparqlQuery<Value>,
    /// Whether this constraint is deactivated.
    pub deactivated: bool,
    /// RDF `sh:message` literals, including any language tags.
    pub messages: Vec<Value>,
    /// Result annotation declarations.
    pub annotations: Vec<ResultAnnotation<Value>>,
}

impl<Value> SparqlConstraint<Value> {
    /// Create an active constraint with no messages or annotations.
    #[must_use]
    pub fn new(id: Value, select: SparqlQuery<Value>) -> Self {
        Self {
            id,
            select,
            deactivated: false,
            messages: Vec::new(),
            annotations: Vec::new(),
        }
    }
}

/// A SPARQL validator attached to a custom constraint component.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SparqlValidator<Value = crate::DefaultTerm> {
    /// The validator's IRI or blank node.
    pub id: Value,
    /// The ASK or SELECT query declaration.
    pub query: SparqlSelectOrAsk<Value>,
    /// RDF `sh:message` literals.
    pub messages: Vec<Value>,
    /// Result annotation declarations.
    pub annotations: Vec<ResultAnnotation<Value>>,
}

impl<Value> SparqlValidator<Value> {
    /// Create a validator with no messages or annotations.
    #[must_use]
    pub fn new(id: Value, query: SparqlSelectOrAsk<Value>) -> Self {
        Self {
            id,
            query,
            messages: Vec::new(),
            annotations: Vec::new(),
        }
    }
}

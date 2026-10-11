//! Parameters shared by custom components, target types, and functions.

use iri_rfc3987::Iri;

use crate::Component;

/// One `sh:parameter` declaration, retaining constraints and RDF annotations.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Value: serde::Serialize, Count: serde::Serialize",
        deserialize = "Value: serde::Deserialize<'de>, Count: serde::Deserialize<'de>"
    )
)]
pub struct Parameter<Value = crate::DefaultTerm, Count = Value> {
    /// The parameter's IRI or blank node.
    pub id: Value,
    /// The parameter predicate (`sh:path`), from which its variable is derived.
    pub path: Iri<String>,
    /// Whether the parameter is optional.
    pub optional: bool,
    /// The original numeric `sh:order` literal, without conversion to a float.
    pub order: Option<Value>,
    /// RDF `sh:name` literals.
    pub names: Vec<Value>,
    /// RDF `sh:description` literals.
    pub descriptions: Vec<Value>,
    /// Constraints on supplied parameter values.
    pub components: Vec<Component<Value, Count>>,
}

impl<Value, Count> Parameter<Value, Count> {
    /// Create a required parameter with no constraints or annotations.
    #[must_use]
    pub fn new(id: Value, path: Iri<String>) -> Self {
        Self {
            id,
            path,
            optional: false,
            order: None,
            names: Vec::new(),
            descriptions: Vec::new(),
            components: Vec::new(),
        }
    }
}

/// All values supplied for one parameter predicate at a component or target use.
///
/// Values are not reduced to a single binding. The consumer checks cardinality
/// and, for custom constraints, forms the required binding combinations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParameterValues<Value = crate::DefaultTerm> {
    /// The parameter predicate.
    pub path: Iri<String>,
    /// Supplied RDF terms in caller order.
    pub values: Vec<Value>,
}

//! Parameters shared by custom components, target types, and functions.

use iri_rfc3987::Iri;

use crate::Component;

/// One `sh:parameter` declaration, retaining constraints and RDF annotations.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Value: serde::Serialize, Count: serde::Serialize, Flags: serde::Serialize, Pattern: serde::Serialize, Named: serde::Serialize",
        deserialize = "Value: serde::Deserialize<'de>, Count: serde::Deserialize<'de>, Flags: serde::Deserialize<'de>, Pattern: serde::Deserialize<'de>, Named: serde::Deserialize<'de>"
    )
)]
pub struct Parameter<
    Value = crate::DefaultTerm,
    Count = Value,
    Flags = Value,
    Pattern = Value,
    Named = Iri<String>,
> {
    /// The parameter's IRI or blank node.
    pub id: Value,
    /// The parameter predicate (`sh:path`), from which its variable is derived.
    pub path: Named,
    /// Whether the parameter is optional.
    pub optional: bool,
    /// The original numeric `sh:order` literal, without conversion to a float.
    pub order: Option<Value>,
    /// RDF `sh:name` literals.
    pub names: Vec<Value>,
    /// RDF `sh:description` literals.
    pub descriptions: Vec<Value>,
    /// Constraints on supplied parameter values.
    pub components: Vec<Component<Value, Count, Flags, Pattern, Named>>,
}

impl<Value, Count, Flags, Pattern, Named> Parameter<Value, Count, Flags, Pattern, Named> {
    /// Create a required parameter with no constraints or annotations.
    #[must_use]
    pub fn new(id: Value, path: Named) -> Self {
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
pub struct ParameterValues<Value = crate::DefaultTerm, Named = Iri<String>> {
    /// The parameter predicate.
    pub path: Named,
    /// Supplied RDF terms in caller order.
    pub values: Vec<Value>,
}

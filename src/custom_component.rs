//! SPARQL-based custom component definitions and uses.

use iri_rfc3987::Iri;

use crate::{Parameter, ParameterValues, SparqlValidator};

/// A custom `sh:ConstraintComponent` declaration.
///
/// Validator roles are kept separately so consumers can select the applicable
/// node/property validator without losing the generic ASK validators.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Value: serde::Serialize, Count: serde::Serialize",
        deserialize = "Value: serde::Deserialize<'de>, Count: serde::Deserialize<'de>"
    )
)]
pub struct CustomConstraintComponent<Value = crate::DefaultTerm, Count = Value> {
    /// The component IRI.
    pub id: Iri<String>,
    /// Parameter declarations in caller order; no ordering is evaluated here.
    pub parameters: Vec<Parameter<Value, Count>>,
    /// Generic `sh:validator` declarations.
    pub validators: Vec<SparqlValidator<Value>>,
    /// `sh:nodeValidator` declarations.
    pub node_validators: Vec<SparqlValidator<Value>>,
    /// `sh:propertyValidator` declarations.
    pub property_validators: Vec<SparqlValidator<Value>>,
    /// RDF `sh:labelTemplate` literals.
    pub label_templates: Vec<Value>,
}

impl<Value, Count> CustomConstraintComponent<Value, Count> {
    /// Create a component whose declarations can be populated by the caller.
    #[must_use]
    pub fn new(id: Iri<String>) -> Self {
        Self {
            id,
            parameters: Vec::new(),
            validators: Vec::new(),
            node_validators: Vec::new(),
            property_validators: Vec::new(),
            label_templates: Vec::new(),
        }
    }
}

/// An application of a custom component to a shape.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CustomConstraint<Value = crate::DefaultTerm> {
    /// The IRI of a separately supplied component declaration.
    pub component: Iri<String>,
    /// All supplied parameter values, before constructing execution bindings.
    pub parameters: Vec<ParameterValues<Value>>,
}

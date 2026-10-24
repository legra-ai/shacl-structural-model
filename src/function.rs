//! SPARQL-based SHACL function declarations.

use iri_rfc3987::Iri;

use crate::{Parameter, SparqlSelectOrAsk};

/// A `sh:SPARQLFunction` declaration without registration or execution.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Value: serde::Serialize, Count: serde::Serialize, Flags: serde::Serialize, Pattern: serde::Serialize, Named: serde::Serialize",
        deserialize = "Value: serde::Deserialize<'de>, Count: serde::Deserialize<'de>, Flags: serde::Deserialize<'de>, Pattern: serde::Deserialize<'de>, Named: serde::Deserialize<'de>"
    )
)]
pub struct ShaclFunction<
    Value = crate::DefaultTerm,
    Count = Value,
    Flags = Value,
    Pattern = Value,
    Named = Iri<String>,
> {
    /// The function IRI.
    pub id: Named,
    /// Parameters in caller order; consumers apply SHACL's ordering rules.
    pub parameters: Vec<Parameter<Value, Count, Flags, Pattern, Named>>,
    /// Optional `sh:returnType` IRI.
    pub return_type: Option<Named>,
    /// The function's ASK or SELECT query declaration.
    pub query: SparqlSelectOrAsk<Value>,
}

impl<Value, Count, Flags, Pattern, Named> ShaclFunction<Value, Count, Flags, Pattern, Named> {
    /// Create a function with no parameters or declared return type.
    #[must_use]
    pub fn new(id: Named, query: SparqlSelectOrAsk<Value>) -> Self {
        Self {
            id,
            parameters: Vec::new(),
            return_type: None,
            query,
        }
    }
}

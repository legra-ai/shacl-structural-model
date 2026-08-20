//! SHACL node kind enumeration.

use crate::StructuralModelError;

/// The type of RDF node a value must be.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum NodeKind {
    /// The value must be an IRI (`sh:IRI`).
    Iri,
    /// The value must be a blank node (`sh:BlankNode`).
    BlankNode,
    /// The value must be a literal (`sh:Literal`).
    Literal,
    /// The value must be a blank node or IRI (`sh:BlankNodeOrIRI`).
    BlankNodeOrIri,
    /// The value must be a blank node or literal (`sh:BlankNodeOrLiteral`).
    BlankNodeOrLiteral,
    /// The value must be an IRI or literal (`sh:IRIOrLiteral`).
    IriOrLiteral,
}

impl NodeKind {
    /// Parse a node kind from its canonical SHACL IRI.
    ///
    /// # Errors
    ///
    /// Returns an error for an unknown IRI.
    pub fn from_iri(iri: &str) -> Result<Self, StructuralModelError> {
        match iri {
            "http://www.w3.org/ns/shacl#IRI" => Ok(Self::Iri),
            "http://www.w3.org/ns/shacl#BlankNode" => Ok(Self::BlankNode),
            "http://www.w3.org/ns/shacl#Literal" => Ok(Self::Literal),
            "http://www.w3.org/ns/shacl#BlankNodeOrIRI" => Ok(Self::BlankNodeOrIri),
            "http://www.w3.org/ns/shacl#BlankNodeOrLiteral" => Ok(Self::BlankNodeOrLiteral),
            "http://www.w3.org/ns/shacl#IRIOrLiteral" => Ok(Self::IriOrLiteral),
            _ => Err(StructuralModelError::UnknownNodeKind {
                value: iri.to_owned(),
            }),
        }
    }

    /// Return the canonical SHACL IRI for this node kind.
    #[must_use]
    pub const fn as_iri(self) -> &'static str {
        match self {
            Self::Iri => "http://www.w3.org/ns/shacl#IRI",
            Self::BlankNode => "http://www.w3.org/ns/shacl#BlankNode",
            Self::Literal => "http://www.w3.org/ns/shacl#Literal",
            Self::BlankNodeOrIri => "http://www.w3.org/ns/shacl#BlankNodeOrIRI",
            Self::BlankNodeOrLiteral => "http://www.w3.org/ns/shacl#BlankNodeOrLiteral",
            Self::IriOrLiteral => "http://www.w3.org/ns/shacl#IRIOrLiteral",
        }
    }
}

//! SHACL node kind enumeration.

use crate::{LexicalRole, StructuralModelError, vocabulary};

/// The type of RDF node a value must be.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
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
    /// Returns [`StructuralModelError::UnknownTerm`] with
    /// [`LexicalRole::NodeKindIri`] for any other IRI.
    pub fn from_iri(iri: &str) -> Result<Self, StructuralModelError> {
        match iri {
            vocabulary::IRI => Ok(Self::Iri),
            vocabulary::BLANK_NODE => Ok(Self::BlankNode),
            vocabulary::LITERAL => Ok(Self::Literal),
            vocabulary::BLANK_NODE_OR_IRI => Ok(Self::BlankNodeOrIri),
            vocabulary::BLANK_NODE_OR_LITERAL => Ok(Self::BlankNodeOrLiteral),
            vocabulary::IRI_OR_LITERAL => Ok(Self::IriOrLiteral),
            _ => Err(StructuralModelError::UnknownTerm {
                lexical: iri.to_owned(),
                expected: LexicalRole::NodeKindIri,
            }),
        }
    }

    /// Return the canonical SHACL IRI for this node kind.
    #[must_use]
    pub const fn as_iri(self) -> &'static str {
        match self {
            Self::Iri => vocabulary::IRI,
            Self::BlankNode => vocabulary::BLANK_NODE,
            Self::Literal => vocabulary::LITERAL,
            Self::BlankNodeOrIri => vocabulary::BLANK_NODE_OR_IRI,
            Self::BlankNodeOrLiteral => vocabulary::BLANK_NODE_OR_LITERAL,
            Self::IriOrLiteral => vocabulary::IRI_OR_LITERAL,
        }
    }
}

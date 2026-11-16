//! SHACL severity levels.

use iri_rfc3987::Iri;

use crate::{LexicalRole, StructuralModelError, vocabulary};

/// The severity of a SHACL validation result.
///
/// The order runs from most to least severe: `Violation < Warning < Info`,
/// so an ascending sort lists the most severe results first. Application-defined
/// severities carry no standard rank; they sort after `Info`, by IRI.
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum Severity {
    /// A violation.
    #[default]
    Violation,
    /// A warning.
    Warning,
    /// An informational note.
    Info,
    /// An application-defined severity IRI.
    Custom(Iri<String>),
}

impl Severity {
    /// Parse a standard or application-defined severity IRI.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralModelError::InvalidSyntax`] with
    /// [`LexicalRole::SeverityIri`] if the value is not a valid absolute IRI.
    pub fn from_iri(iri: &str) -> Result<Self, StructuralModelError> {
        match iri {
            vocabulary::VIOLATION => Ok(Self::Violation),
            vocabulary::WARNING => Ok(Self::Warning),
            vocabulary::INFO => Ok(Self::Info),
            _ => iri
                .parse()
                .map(Self::Custom)
                .map_err(|_| StructuralModelError::InvalidSyntax {
                    lexical: iri.to_owned(),
                    expected: LexicalRole::SeverityIri,
                }),
        }
    }

    /// Return the human-readable severity label.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Violation => "Violation",
            Self::Warning => "Warning",
            Self::Info => "Info",
            Self::Custom(iri) => iri.as_str(),
        }
    }

    /// Return the full severity IRI.
    #[must_use]
    pub fn as_iri(&self) -> &str {
        match self {
            Self::Violation => vocabulary::VIOLATION,
            Self::Warning => vocabulary::WARNING,
            Self::Info => vocabulary::INFO,
            Self::Custom(iri) => iri.as_str(),
        }
    }
}

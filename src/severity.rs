//! SHACL severity levels.

use crate::StructuralModelError;
use iri_rfc3987::Iri;

/// The severity of a SHACL validation result.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
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
    /// Returns an error if the value is not a valid absolute IRI.
    pub fn from_iri(iri: &str) -> Result<Self, StructuralModelError> {
        match iri {
            "http://www.w3.org/ns/shacl#Violation" => Ok(Self::Violation),
            "http://www.w3.org/ns/shacl#Warning" => Ok(Self::Warning),
            "http://www.w3.org/ns/shacl#Info" => Ok(Self::Info),
            _ => iri.parse().map(Self::Custom).map_err(|_| {
                StructuralModelError::InvalidSeverityIri {
                    value: iri.to_owned(),
                }
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
            Self::Violation => "http://www.w3.org/ns/shacl#Violation",
            Self::Warning => "http://www.w3.org/ns/shacl#Warning",
            Self::Info => "http://www.w3.org/ns/shacl#Info",
            Self::Custom(iri) => iri.as_str(),
        }
    }
}

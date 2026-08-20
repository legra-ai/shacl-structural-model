//! SHACL severity levels.

use crate::StructuralModelError;

/// The severity of a SHACL validation result.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum Severity {
    /// A violation.
    #[default]
    Violation,
    /// A warning.
    Warning,
    /// An informational note.
    Info,
}

impl Severity {
    /// Parse a severity from its canonical SHACL IRI.
    ///
    /// # Errors
    ///
    /// Returns an error for an unknown IRI.
    pub fn from_iri(iri: &str) -> Result<Self, StructuralModelError> {
        match iri {
            "http://www.w3.org/ns/shacl#Violation" => Ok(Self::Violation),
            "http://www.w3.org/ns/shacl#Warning" => Ok(Self::Warning),
            "http://www.w3.org/ns/shacl#Info" => Ok(Self::Info),
            _ => Err(StructuralModelError::UnknownSeverity {
                value: iri.to_owned(),
            }),
        }
    }

    /// Return the human-readable severity label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Violation => "Violation",
            Self::Warning => "Warning",
            Self::Info => "Info",
        }
    }

    /// Return the canonical SHACL IRI for this severity.
    #[must_use]
    pub const fn as_iri(self) -> &'static str {
        match self {
            Self::Violation => "http://www.w3.org/ns/shacl#Violation",
            Self::Warning => "http://www.w3.org/ns/shacl#Warning",
            Self::Info => "http://www.w3.org/ns/shacl#Info",
        }
    }
}

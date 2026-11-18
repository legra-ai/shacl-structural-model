//! SHACL severity levels.

use iri_rfc3987::Iri;

use crate::{LexicalRole, StructuralModelError, vocabulary};

/// The severity of a SHACL validation result.
///
/// The order runs from most to least severe:
/// `Violation < Warning < Info < Debug < Trace`, so an ascending sort lists the
/// most severe results first. Application-defined severities carry no standard
/// rank; they sort after `Trace`, by IRI.
///
/// `Debug` and `Trace` follow SHACL 1.2 Core, Working Draft 16 November 2026,
/// section 3.1.4: they are messages that are not constraint violations.
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
    /// A SHACL 1.2 debug message that is not a constraint violation.
    Debug,
    /// A SHACL 1.2 trace message that is not a constraint violation.
    Trace,
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
            vocabulary::DEBUG => Ok(Self::Debug),
            vocabulary::TRACE => Ok(Self::Trace),
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
            Self::Debug => "Debug",
            Self::Trace => "Trace",
            Self::Custom(iri) => iri.as_str(),
        }
    }

    /// Whether this standard severity reports a constraint violation.
    ///
    /// `Debug` and `Trace` do not. An application-defined severity has no
    /// standard meaning and returns `None`.
    #[must_use]
    pub const fn is_violation(&self) -> Option<bool> {
        match self {
            Self::Violation | Self::Warning | Self::Info => Some(true),
            Self::Debug | Self::Trace => Some(false),
            Self::Custom(_) => None,
        }
    }

    /// Return the full severity IRI.
    #[must_use]
    pub fn as_iri(&self) -> &str {
        match self {
            Self::Violation => vocabulary::VIOLATION,
            Self::Warning => vocabulary::WARNING,
            Self::Info => vocabulary::INFO,
            Self::Debug => vocabulary::DEBUG,
            Self::Trace => vocabulary::TRACE,
            Self::Custom(iri) => iri.as_str(),
        }
    }
}

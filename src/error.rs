//! Errors produced while decoding standard SHACL enum values.

/// Errors raised by structural constructors and standard-value decoding.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum StructuralModelError {
    /// An IRI did not identify a known SHACL node kind.
    #[error("unknown SHACL node kind IRI: {value}")]
    UnknownNodeKind {
        /// The unrecognised IRI.
        value: String,
    },
    /// A severity value was not a valid absolute IRI.
    #[error("invalid SHACL severity IRI: {value}")]
    InvalidSeverityIri {
        /// The invalid IRI text.
        value: String,
    },
}

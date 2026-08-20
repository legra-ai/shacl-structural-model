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
    /// An IRI did not identify a known SHACL severity.
    #[error("unknown SHACL severity IRI: {value}")]
    UnknownSeverity {
        /// The unrecognised IRI.
        value: String,
    },
}

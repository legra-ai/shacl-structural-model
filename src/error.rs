//! Errors produced by checked constructors and standard-value decoding.

use std::fmt;

/// The role a rejected lexical value was supplied for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LexicalRole {
    /// A `sh:nodeKind` value, one of the seven SHACL node-kind IRIs.
    NodeKindIri,
    /// A `sh:severity` value, a standard or application-defined absolute IRI.
    SeverityIri,
    /// A `sh:languageIn` member, a well-formed BCP 47 language tag.
    LanguageTag,
    /// A `sh:codeIdentifier` value, matching `^[a-zA-Z_][a-zA-Z0-9_]*$`.
    CodeIdentifier,
}

impl fmt::Display for LexicalRole {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::NodeKindIri => "SHACL node kind IRI",
            Self::SeverityIri => "SHACL severity IRI",
            Self::LanguageTag => "BCP 47 language tag",
            Self::CodeIdentifier => "SHACL code identifier",
        })
    }
}

/// Errors raised by structural constructors and standard-value decoding.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum StructuralModelError {
    /// A well-formed value is not a member of the role's closed vocabulary.
    #[error("unknown {expected}: {lexical}")]
    UnknownTerm {
        /// The offending lexical value.
        lexical: String,
        /// The role whose vocabulary was expected.
        expected: LexicalRole,
    },
    /// A value does not have the lexical syntax its role requires.
    #[error("invalid {expected}: {lexical}")]
    InvalidSyntax {
        /// The offending lexical value.
        lexical: String,
        /// The role whose syntax was expected.
        expected: LexicalRole,
    },
}

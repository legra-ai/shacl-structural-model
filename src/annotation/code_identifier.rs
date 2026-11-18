//! Checked `sh:codeIdentifier` values (Core section 6.11.4).

use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{LexicalRole, StructuralModelError};

/// A `sh:codeIdentifier` name matching `^[a-zA-Z_][a-zA-Z0-9_]*$`.
///
/// SHACL 1.2 Core, Working Draft 16 November 2026, section 6.11.4. Serde
/// decoding applies the same check as [`CodeIdentifier::try_new`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CodeIdentifier(String);

impl CodeIdentifier {
    /// Check the identifier syntax.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralModelError::InvalidSyntax`] with
    /// [`LexicalRole::CodeIdentifier`] when the text does not match the pattern.
    pub fn try_new(lexical: impl Into<String>) -> Result<Self, StructuralModelError> {
        let lexical = lexical.into();
        if Self::is_valid(&lexical) {
            Ok(Self(lexical))
        } else {
            Err(StructuralModelError::InvalidSyntax {
                lexical,
                expected: LexicalRole::CodeIdentifier,
            })
        }
    }

    fn is_valid(text: &str) -> bool {
        let mut bytes = text.bytes();
        bytes
            .next()
            .is_some_and(|first| first.is_ascii_alphabetic() || first == b'_')
            && bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    }

    /// The identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CodeIdentifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Serialize for CodeIdentifier {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for CodeIdentifier {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_new(String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

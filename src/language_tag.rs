//! Validated `sh:languageIn` members.

use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{LexicalRole, StructuralModelError};

/// A well-formed BCP 47 language tag, as listed by `sh:languageIn`.
///
/// The check is syntactic (RFC 5646 well-formedness, including unique variants
/// and extension singletons); the IANA registry is not consulted. Spelling is
/// preserved, while equality, hashing and ordering are ASCII case-insensitive.
/// Serde decoding applies the same check as [`LanguageTag::try_new`].
#[derive(Debug, Clone)]
pub struct LanguageTag(String);

impl LanguageTag {
    /// Validate a language tag.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralModelError::InvalidSyntax`] with
    /// [`LexicalRole::LanguageTag`] for a malformed tag.
    pub fn try_new(lexical: impl Into<String>) -> Result<Self, StructuralModelError> {
        let lexical = lexical.into();
        if rdf_structural_model::LanguageTag::try_new(lexical.as_str()).is_err() {
            return Err(StructuralModelError::InvalidSyntax {
                lexical,
                expected: LexicalRole::LanguageTag,
            });
        }
        Ok(Self(lexical))
    }

    /// The original spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn folded(&self) -> impl Iterator<Item = u8> + '_ {
        self.as_str().bytes().map(|byte| byte.to_ascii_lowercase())
    }
}

impl PartialEq for LanguageTag {
    fn eq(&self, other: &Self) -> bool {
        self.as_str().eq_ignore_ascii_case(other.as_str())
    }
}

impl Eq for LanguageTag {}

impl Hash for LanguageTag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for byte in self.folded() {
            state.write_u8(byte);
        }
        state.write_u8(0xff);
    }
}

impl PartialOrd for LanguageTag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LanguageTag {
    fn cmp(&self, other: &Self) -> Ordering {
        self.folded().cmp(other.folded())
    }
}

impl fmt::Display for LanguageTag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl Serialize for LanguageTag {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for LanguageTag {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_new(String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tags_compare_case_insensitively_and_keep_spelling() {
        let upper = LanguageTag::try_new("EN-gb").unwrap();
        let lower = LanguageTag::try_new("en-GB").unwrap();
        assert_eq!(upper, lower);
        assert_eq!(upper.cmp(&lower), Ordering::Equal);
        assert_eq!(upper.as_str(), "EN-gb");
        assert!(LanguageTag::try_new("de").unwrap() < upper);
    }

    #[test]
    fn malformed_tags_report_the_lexical_value_and_role() {
        assert_eq!(
            LanguageTag::try_new("en--gb"),
            Err(StructuralModelError::InvalidSyntax {
                lexical: "en--gb".to_owned(),
                expected: LexicalRole::LanguageTag,
            })
        );
        assert!(LanguageTag::try_new("de-1996-1996").is_err());
    }
}

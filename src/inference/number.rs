//! Exact layer/order declarations, not a general datatype evaluator.

use std::cmp::Ordering;

use borrow_or_share::Bos;
use iri_rfc3987::Iri;
use rdf_structural_model::Literal;
use serde::{Deserialize, Deserializer};

use super::InferenceModelError;

const INTEGER: &str = "http://www.w3.org/2001/XMLSchema#integer";
const DECIMAL: &str = "http://www.w3.org/2001/XMLSchema#decimal";

/// An exact numeric declaration retaining its original RDF lexical identity.
///
/// Equality/hash remain lexical RDF identity. Use [`Self::cmp_value`] for numeric
/// ordering; no `Ord` is provided because it would conflict with lexical `Eq`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
#[serde(transparent, bound(serialize = "Text: serde::Serialize"))]
pub struct RuleNumber<Text: Bos<str> + AsRef<str> = String>(Literal<Text>);

impl<Text: Bos<str> + AsRef<str>> RuleNumber<Text> {
    /// Check the datatype and lexical space without normalizing identity.
    ///
    /// # Errors
    /// Returns [`InferenceModelError::InvalidRuleNumber`] for an unsupported
    /// datatype or malformed numeric lexical form.
    pub fn try_from_literal(literal: Literal<Text>) -> Result<Self, InferenceModelError> {
        let datatype = literal.metadata().datatype().as_str();
        let text = literal.lexical();
        // Exactly one leading sign is permitted, and whitespace is not normalized.
        let text = if literal.lexical().starts_with(['+', '-']) {
            &literal.lexical()[1..]
        } else {
            text
        };
        let valid = match datatype {
            INTEGER => !text.is_empty() && text.bytes().all(|c| c.is_ascii_digit()),
            DECIMAL => {
                let mut dots = 0;
                let mut digits = 0;
                for c in text.bytes() {
                    if c.is_ascii_digit() {
                        digits += 1;
                    } else if c == b'.' {
                        dots += 1;
                    } else {
                        return Err(InferenceModelError::InvalidRuleNumber);
                    }
                }
                dots <= 1 && digits > 0
            }
            _ => false,
        };
        if !valid {
            return Err(InferenceModelError::InvalidRuleNumber);
        }
        Ok(Self(literal))
    }

    /// The unchanged lexical literal.
    #[must_use]
    pub const fn literal(&self) -> &Literal<Text> {
        &self.0
    }

    /// Compare exact values using borrowed digit windows and constant state.
    /// No float, bounded-width integer conversion or normalized digit buffer is used.
    #[must_use]
    pub fn cmp_value<Other: Bos<str> + AsRef<str>>(&self, other: &RuleNumber<Other>) -> Ordering {
        let a = Digits::new(self.0.lexical());
        let b = Digits::new(other.0.lexical());
        match (a.negative, b.negative) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            _ => {
                let magnitude = a
                    .integer
                    .len()
                    .cmp(&b.integer.len())
                    .then_with(|| a.integer.cmp(b.integer))
                    .then_with(|| a.fraction.cmp(b.fraction));
                if a.negative {
                    magnitude.reverse()
                } else {
                    magnitude
                }
            }
        }
    }
}

impl<'de, Text> Deserialize<'de> for RuleNumber<Text>
where
    Text: Bos<str> + AsRef<str> + Deserialize<'de>,
    Iri<Text>: Deserialize<'de>,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from_literal(Literal::deserialize(deserializer)?)
            .map_err(serde::de::Error::custom)
    }
}

struct Digits<'a> {
    negative: bool,
    integer: &'a str,
    fraction: &'a str,
}
impl<'a> Digits<'a> {
    fn new(text: &'a str) -> Self {
        let unsigned = text.strip_prefix(['+', '-']).unwrap_or(text);
        let (integer, fraction) = unsigned.split_once('.').unwrap_or((unsigned, ""));
        let integer = integer.trim_start_matches('0');
        let fraction = fraction.trim_end_matches('0');
        Self {
            negative: text.starts_with('-') && !(integer.is_empty() && fraction.is_empty()),
            integer,
            fraction,
        }
    }
}

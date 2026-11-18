//! `sh:ruleProcessor` values (Inference Rules section 10).

use borrow_or_share::Bos;
use iri_rfc3987::Iri;

/// A custom rule processor requested by a rule or rule set.
///
/// The values of `sh:ruleProcessor` are IRIs or `xsd:string` literals. A rules
/// engine that cannot handle the processor reports a failure; that decision
/// is consumer-owned.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(bound(
    serialize = "Text: serde::Serialize",
    deserialize = "Text: serde::Deserialize<'de>, Iri<Text>: serde::Deserialize<'de>"
))]
pub enum RuleProcessor<Text: Bos<str> + AsRef<str> = String> {
    /// A processor identified by an IRI.
    Iri(Iri<Text>),
    /// The lexical form of an `xsd:string` literal, such as `"SRL 1.2"`.
    Literal(Text),
}

impl<Text: Bos<str> + AsRef<str>> RuleProcessor<Text> {
    /// The IRI or literal text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Iri(iri) => iri.as_str(),
            Self::Literal(text) => text.as_ref(),
        }
    }
}

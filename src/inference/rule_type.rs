//! Rule-type declarations (Inference Rules section 3.1).

use borrow_or_share::Bos;
use iri_rfc3987::Iri;
use rdf_structural_model::Subject;

use crate::vocabulary;

/// A rule type: `sh:SPARQLRule`, `sh:TripleRule`, or another rule-type IRI
/// such as a SPARQL rule template (section 12) or an engine-specific type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(bound(
    serialize = "Text: serde::Serialize",
    deserialize = "Text: serde::Deserialize<'de>, Iri<Text>: serde::Deserialize<'de>"
))]
pub enum RuleType<Text: Bos<str> + AsRef<str> = String> {
    /// `sh:SPARQLRule` (section 11.1).
    Sparql,
    /// `sh:TripleRule` (section 11.2).
    Triple,
    /// Any other rule-type IRI; the consumer resolves templates and custom types.
    Other(Iri<Text>),
}

impl<Text: Bos<str> + AsRef<str>> RuleType<Text> {
    /// Classify a rule-type IRI.
    #[must_use]
    pub fn from_iri(iri: Iri<Text>) -> Self {
        match iri.as_str() {
            vocabulary::SPARQL_RULE => Self::Sparql,
            vocabulary::TRIPLE_RULE => Self::Triple,
            _ => Self::Other(iri),
        }
    }

    /// The rule-type IRI.
    #[must_use]
    pub fn iri(&self) -> &str {
        match self {
            Self::Sparql => vocabulary::SPARQL_RULE,
            Self::Triple => vocabulary::TRIPLE_RULE,
            Self::Other(iri) => iri.as_str(),
        }
    }

    /// Whether this is one of the two built-in rule types (section 11).
    #[must_use]
    pub const fn is_built_in(&self) -> bool {
        matches!(self, Self::Sparql | Self::Triple)
    }
}

/// One `rdf:type` triple of a rule. A rule may have several rule types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Text: serde::Serialize",
        deserialize = "Text: serde::Deserialize<'de>, Iri<Text>: serde::Deserialize<'de>"
    )
)]
pub struct RuleTyping<Text: Bos<str> + AsRef<str> = String> {
    rule: Subject<Text>,
    rule_type: RuleType<Text>,
}

impl<Text: Bos<str> + AsRef<str>> RuleTyping<Text> {
    /// The triple `rule rdf:type rule_type`.
    #[must_use]
    pub const fn new(rule: Subject<Text>, rule_type: RuleType<Text>) -> Self {
        Self { rule, rule_type }
    }
    /// The typed rule.
    #[must_use]
    pub const fn rule(&self) -> &Subject<Text> {
        &self.rule
    }
    /// The declared rule type.
    #[must_use]
    pub const fn rule_type(&self) -> &RuleType<Text> {
        &self.rule_type
    }
}

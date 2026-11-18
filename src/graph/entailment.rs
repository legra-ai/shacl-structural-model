//! `sh:entailment` declarations, including `sh:RulesEntailment`.

use iri_rfc3987::Iri;

use crate::vocabulary;

/// A value of `sh:entailment`.
///
/// Core section 1.4 makes the values IRIs. Inference Rules section 6.1
/// defines `sh:RulesEntailment`; section 6.2 allows a rule-set IRI, which is
/// indistinguishable from another regime IRI without the shapes graph, so
/// both are [`Entailment::Regime`] and the consumer resolves them.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum Entailment<Named = Iri<String>> {
    /// `sh:RulesEntailment`: execute the default rule set of the shapes graph.
    Rules,
    /// Any other regime IRI, such as a SPARQL entailment regime or a rule set.
    Regime(Named),
}

impl<Named: AsRef<str>> Entailment<Named> {
    /// Classify an entailment IRI.
    #[must_use]
    pub fn from_iri(iri: Named) -> Self {
        if iri.as_ref() == vocabulary::RULES_ENTAILMENT {
            Self::Rules
        } else {
            Self::Regime(iri)
        }
    }

    /// The entailment IRI.
    #[must_use]
    pub fn iri(&self) -> &str {
        match self {
            Self::Rules => vocabulary::RULES_ENTAILMENT,
            Self::Regime(iri) => iri.as_ref(),
        }
    }
}

/// One `sh:entailment` triple of a shapes graph.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct EntailmentDeclaration<Named = Iri<String>> {
    /// The shapes graph IRI.
    pub shapes_graph: Named,
    /// The required entailment.
    pub entailment: Entailment<Named>,
}

impl<Named> EntailmentDeclaration<Named> {
    /// The triple `shapes_graph sh:entailment entailment`.
    #[must_use]
    pub const fn new(shapes_graph: Named, entailment: Entailment<Named>) -> Self {
        Self {
            shapes_graph,
            entailment,
        }
    }
}

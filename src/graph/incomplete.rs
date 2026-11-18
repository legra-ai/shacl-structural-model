//! `sh:incomplete` and `sh:incompleteURIStart` data-graph declarations
//! (Core section 6.9.3.1).

use iri_rfc3987::Iri;

/// One `sh:incomplete` triple of the data graph.
///
/// The incompleteness values of a node also include `sh:incomplete` value
/// nodes derived from property shapes; the consumer combines both.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct IncompleteNode<Value = crate::DefaultTerm> {
    /// The subject node.
    pub node: Value,
    /// Whether the node is incomplete (`true`) or complete (`false`).
    pub incomplete: bool,
}

impl<Value> IncompleteNode<Value> {
    /// The triple `node sh:incomplete incomplete`.
    #[must_use]
    pub const fn new(node: Value, incomplete: bool) -> Self {
        Self { node, incomplete }
    }
}

/// One `sh:incompleteURIStart` triple of the data graph; its subject is an IRI.
///
/// With `true` as the subject's only value, every other IRI starting with the
/// subject IRI is incomplete unless declared `sh:incomplete false`.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct IncompleteUriStart<Named = Iri<String>> {
    /// The subject IRI.
    pub prefix: Named,
    /// The declared boolean.
    pub incomplete_uri_start: bool,
}

impl<Named: AsRef<str>> IncompleteUriStart<Named> {
    /// The triple `prefix sh:incompleteURIStart incomplete_uri_start`.
    #[must_use]
    pub const fn new(prefix: Named, incomplete_uri_start: bool) -> Self {
        Self {
            prefix,
            incomplete_uri_start,
        }
    }

    /// Whether this declaration is `true` and `iri` starts with, but is not
    /// equal to, the prefix. The consumer still checks that `true` is the
    /// prefix's only value and applies `sh:incomplete false` exceptions.
    #[must_use]
    pub fn covers(&self, iri: &str) -> bool {
        let prefix = self.prefix.as_ref();
        self.incomplete_uri_start && iri.len() > prefix.len() && iri.starts_with(prefix)
    }
}

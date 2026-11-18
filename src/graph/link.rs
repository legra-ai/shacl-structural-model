//! `sh:shapesGraph`, `sh:dataGraph` and `sh:suggestedShapesGraph` links.

use iri_rfc3987::Iri;

use crate::vocabulary;

/// The predicate of a graph link.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum GraphLinkKind {
    /// `sh:shapesGraph`: a data graph suggests a shapes graph (Core section 6.5).
    ShapesGraph,
    /// `sh:dataGraph`: a shapes graph names a data graph it validates (Core section 6.6).
    DataGraph,
    /// `sh:suggestedShapesGraph`: an ontology suggests a shapes graph used in
    /// the absence of `sh:shapesGraph`.
    SuggestedShapesGraph,
}

impl GraphLinkKind {
    /// The predicate IRI.
    #[must_use]
    pub const fn predicate_iri(self) -> &'static str {
        match self {
            Self::ShapesGraph => vocabulary::SHAPES_GRAPH,
            Self::DataGraph => vocabulary::DATA_GRAPH,
            Self::SuggestedShapesGraph => vocabulary::SUGGESTED_SHAPES_GRAPH,
        }
    }

    /// Whether the processor follows `owl:imports` in the linked graph; Core
    /// section 6.6 excludes data graphs.
    #[must_use]
    pub const fn follows_imports(self) -> bool {
        !matches!(self, Self::DataGraph)
    }
}

/// One graph-link triple `from kind to`, without resolution or import closure.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct GraphLink<Named = Iri<String>> {
    /// The subject graph or ontology IRI.
    pub from: Named,
    /// The link predicate.
    pub kind: GraphLinkKind,
    /// The linked graph IRI, possibly an `owl:versionIRI`.
    pub to: Named,
}

impl<Named> GraphLink<Named> {
    /// The triple `from kind to`.
    #[must_use]
    pub const fn new(from: Named, kind: GraphLinkKind, to: Named) -> Self {
        Self { from, kind, to }
    }
}

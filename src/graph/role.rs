//! `sh:Graph`, `sh:DataGraph`, `sh:ShapesGraph` and `sh:RulesGraph` typing.

use iri_rfc3987::Iri;

use crate::vocabulary;

/// A graph type class. The roles are not mutually exclusive.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum GraphRole {
    /// `sh:Graph`, the class of graphs.
    Graph,
    /// `sh:DataGraph` (Core section 6.2).
    DataGraph,
    /// `sh:ShapesGraph` (Core section 6.1).
    ShapesGraph,
    /// `sh:RulesGraph`, a shapes graph that contains rules (Inference Rules section 5).
    RulesGraph,
}

impl GraphRole {
    /// The class IRI.
    #[must_use]
    pub const fn class_iri(self) -> &'static str {
        match self {
            Self::Graph => vocabulary::GRAPH,
            Self::DataGraph => vocabulary::DATA_GRAPH_CLASS,
            Self::ShapesGraph => vocabulary::SHAPES_GRAPH_CLASS,
            Self::RulesGraph => vocabulary::RULES_GRAPH_CLASS,
        }
    }

    /// The role for a class IRI, if it is one of the four graph classes.
    #[must_use]
    pub fn from_class_iri(iri: &str) -> Option<Self> {
        [
            Self::Graph,
            Self::DataGraph,
            Self::ShapesGraph,
            Self::RulesGraph,
        ]
        .into_iter()
        .find(|role| role.class_iri() == iri)
    }

    /// Whether the role is a shapes graph; `sh:RulesGraph` is a subclass of
    /// `sh:ShapesGraph`.
    #[must_use]
    pub const fn is_shapes_graph(self) -> bool {
        matches!(self, Self::ShapesGraph | Self::RulesGraph)
    }
}

/// One `rdf:type` triple typing a graph IRI with a [`GraphRole`].
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct GraphTypeDeclaration<Named = Iri<String>> {
    /// The graph IRI.
    pub graph: Named,
    /// The declared role.
    pub role: GraphRole,
}

impl<Named> GraphTypeDeclaration<Named> {
    /// Type `graph` with `role`.
    #[must_use]
    pub const fn new(graph: Named, role: GraphRole) -> Self {
        Self { graph, role }
    }
}

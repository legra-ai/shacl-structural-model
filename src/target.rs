//! SHACL target declarations.

use iri_rfc3987::Iri;

use crate::local_name;

/// A target declaration on a node shape.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Target {
    /// `sh:targetNode` — the given node is a focus node.
    Node(Iri<String>),
    /// `sh:targetClass` — instances of the class are focus nodes.
    Class(Iri<String>),
    /// `sh:targetSubjectsOf` — subjects of the predicate are focus nodes.
    SubjectsOf(Iri<String>),
    /// `sh:targetObjectsOf` — objects of the predicate are focus nodes.
    ObjectsOf(Iri<String>),
    /// An implicit class target.
    ImplicitClass(Iri<String>),
}

impl Target {
    /// Return the SHACL target kind name.
    #[must_use]
    pub const fn kind_name(&self) -> &'static str {
        match self {
            Self::Node(_) => "targetNode",
            Self::Class(_) => "targetClass",
            Self::SubjectsOf(_) => "targetSubjectsOf",
            Self::ObjectsOf(_) => "targetObjectsOf",
            Self::ImplicitClass(_) => "implicitClass",
        }
    }

    /// Return the IRI carried by this target.
    #[must_use]
    pub fn iri(&self) -> &Iri<String> {
        match self {
            Self::Node(iri)
            | Self::Class(iri)
            | Self::SubjectsOf(iri)
            | Self::ObjectsOf(iri)
            | Self::ImplicitClass(iri) => iri,
        }
    }

    /// Return the local-name display value of this target.
    #[must_use]
    pub fn display(&self) -> String {
        local_name(self.iri())
    }
}

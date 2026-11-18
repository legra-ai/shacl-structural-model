//! The value of `sh:closed`.

use crate::vocabulary;

/// The value of `sh:closed`, following SHACL 1.2 Core, Working Draft
/// 16 November 2026, section 6.10.9.1.
///
/// The draft admits the booleans `true` and `false` and the IRI `sh:ByTypes`.
/// Explicit `false` is kept as [`Closedness::Open`] rather than dropped.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum Closedness {
    /// `sh:closed false` — the shape is explicitly not closed.
    Open,
    /// `sh:closed true` — only properties reachable via `sh:property/sh:path`
    /// of this shape, plus ignored properties, are permitted.
    Closed,
    /// `sh:closed sh:ByTypes` — only `rdf:type` and the properties declared by
    /// the classes and shapes that apply to the value node by its types, plus
    /// ignored properties, are permitted.
    ByTypes,
}

impl Closedness {
    /// Map an `xsd:boolean` value of `sh:closed`.
    #[must_use]
    pub const fn from_boolean(closed: bool) -> Self {
        if closed { Self::Closed } else { Self::Open }
    }

    /// Return the `sh:ByTypes` value when `iri` is its full IRI.
    #[must_use]
    pub fn from_iri(iri: &str) -> Option<Self> {
        (iri == vocabulary::BY_TYPES).then_some(Self::ByTypes)
    }

    /// Whether the closure check applies.
    #[must_use]
    pub const fn is_closed(self) -> bool {
        !matches!(self, Self::Open)
    }

    /// Return the RDF lexical form: `true`, `false` or the `sh:ByTypes` IRI.
    #[must_use]
    pub const fn as_rdf_lexical(self) -> &'static str {
        match self {
            Self::Open => "false",
            Self::Closed => "true",
            Self::ByTypes => vocabulary::BY_TYPES,
        }
    }

    /// Return a short human-readable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "false",
            Self::Closed => "true",
            Self::ByTypes => "ByTypes",
        }
    }
}

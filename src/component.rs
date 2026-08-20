//! SHACL Core constraint components.

use std::fmt;

use iri_rfc3987::Iri;

use crate::{NodeKind, ShaclPath};

/// The default value type used by [`Component`].
pub type DefaultValue = rdf_syntax::Term;

/// A SHACL Core constraint component.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(bound(
    serialize = "Value: serde::Serialize",
    deserialize = "Value: serde::Deserialize<'de>"
))]
pub enum Component<Value = DefaultValue> {
    /// `sh:class` — the value must be an instance of the class.
    Class(Iri<String>),
    /// `sh:datatype` — the value must have the given datatype.
    Datatype(Iri<String>),
    /// `sh:nodeKind` — the value must have the given RDF node kind.
    NodeKind(NodeKind),
    /// `sh:minCount`.
    MinCount(u64),
    /// `sh:maxCount`.
    MaxCount(u64),
    /// `sh:minExclusive`.
    MinExclusive(Value),
    /// `sh:minInclusive`.
    MinInclusive(Value),
    /// `sh:maxExclusive`.
    MaxExclusive(Value),
    /// `sh:maxInclusive`.
    MaxInclusive(Value),
    /// `sh:minLength`.
    MinLength(u64),
    /// `sh:maxLength`.
    MaxLength(u64),
    /// `sh:pattern` with optional `sh:flags`.
    Pattern {
        /// The regular expression pattern.
        pattern: String,
        /// Optional regular expression flags.
        flags: Option<String>,
    },
    /// `sh:languageIn`.
    LanguageIn(Vec<String>),
    /// `sh:uniqueLang`.
    UniqueLang(bool),
    /// `sh:equals`.
    Equals(ShaclPath),
    /// `sh:disjoint`.
    Disjoint(ShaclPath),
    /// `sh:lessThan`.
    LessThan(ShaclPath),
    /// `sh:lessThanOrEquals`.
    LessThanOrEquals(ShaclPath),
    /// `sh:and`.
    And(Vec<Iri<String>>),
    /// `sh:or`.
    Or(Vec<Iri<String>>),
    /// `sh:not`.
    Not(Iri<String>),
    /// `sh:xone`.
    Xone(Vec<Iri<String>>),
    /// `sh:node`.
    Node(Iri<String>),
    /// `sh:qualifiedValueShape` with optional min/max counts.
    QualifiedValueShape {
        /// The shape to count conforming values against.
        shape: Iri<String>,
        /// Minimum number of conforming values.
        min_count: Option<u64>,
        /// Maximum number of conforming values.
        max_count: Option<u64>,
        /// Whether sibling qualified shapes use disjoint value sets.
        disjoint: bool,
    },
    /// `sh:hasValue`.
    HasValue(Value),
    /// `sh:in`.
    In(Vec<Value>),
    /// `sh:closed` with `sh:ignoredProperties`.
    Closed {
        /// Whether the shape is closed.
        is_closed: bool,
        /// Properties exempt from the closure check.
        ignored_properties: Vec<Iri<String>>,
    },
}

impl<Value> Component<Value> {
    /// Return the SHACL constraint kind name.
    #[must_use]
    pub const fn kind_name(&self) -> &'static str {
        match self {
            Self::Class(_) => "class",
            Self::Datatype(_) => "datatype",
            Self::NodeKind(_) => "nodeKind",
            Self::MinCount(_) => "minCount",
            Self::MaxCount(_) => "maxCount",
            Self::MinExclusive(_) => "minExclusive",
            Self::MinInclusive(_) => "minInclusive",
            Self::MaxExclusive(_) => "maxExclusive",
            Self::MaxInclusive(_) => "maxInclusive",
            Self::MinLength(_) => "minLength",
            Self::MaxLength(_) => "maxLength",
            Self::Pattern { .. } => "pattern",
            Self::LanguageIn(_) => "languageIn",
            Self::UniqueLang(_) => "uniqueLang",
            Self::Equals(_) => "equals",
            Self::Disjoint(_) => "disjoint",
            Self::LessThan(_) => "lessThan",
            Self::LessThanOrEquals(_) => "lessThanOrEquals",
            Self::And(_) => "and",
            Self::Or(_) => "or",
            Self::Not(_) => "not",
            Self::Xone(_) => "xone",
            Self::Node(_) => "node",
            Self::QualifiedValueShape { .. } => "qualifiedValueShape",
            Self::HasValue(_) => "hasValue",
            Self::In(_) => "in",
            Self::Closed { .. } => "closed",
        }
    }
}

impl<Value: fmt::Debug> Component<Value> {
    /// Return a human-readable display value for this component.
    #[must_use]
    pub fn display_value(&self) -> String {
        match self {
            Self::Class(iri) | Self::Datatype(iri) | Self::Not(iri) | Self::Node(iri) => {
                crate::local_name(iri)
            }
            Self::NodeKind(kind) => format!("{kind:?}"),
            Self::MinCount(count)
            | Self::MaxCount(count)
            | Self::MinLength(count)
            | Self::MaxLength(count) => count.to_string(),
            Self::MinExclusive(value)
            | Self::MinInclusive(value)
            | Self::MaxExclusive(value)
            | Self::MaxInclusive(value)
            | Self::HasValue(value) => format!("{value:?}"),
            Self::Pattern { pattern, flags } => match flags {
                Some(flags) => format!("/{pattern}/{flags}"),
                None => format!("/{pattern}/"),
            },
            Self::LanguageIn(languages) => languages.join(", "),
            Self::UniqueLang(value) => value.to_string(),
            Self::Equals(path)
            | Self::Disjoint(path)
            | Self::LessThan(path)
            | Self::LessThanOrEquals(path) => path.to_string(),
            Self::And(iris) | Self::Or(iris) | Self::Xone(iris) => iris
                .iter()
                .map(crate::local_name)
                .collect::<Vec<_>>()
                .join(", "),
            Self::QualifiedValueShape {
                shape,
                min_count,
                max_count,
                ..
            } => {
                let mut parts = vec![crate::local_name(shape)];
                if let Some(min) = min_count {
                    parts.push(format!("min={min}"));
                }
                if let Some(max) = max_count {
                    parts.push(format!("max={max}"));
                }
                parts.join(", ")
            }
            Self::In(values) => values
                .iter()
                .map(|value| format!("{value:?}"))
                .collect::<Vec<_>>()
                .join(", "),
            Self::Closed {
                is_closed,
                ignored_properties,
            } => {
                if *is_closed {
                    if ignored_properties.is_empty() {
                        "true".to_owned()
                    } else {
                        let ignored = ignored_properties
                            .iter()
                            .map(crate::local_name)
                            .collect::<Vec<_>>();
                        format!("true (ignore: {})", ignored.join(", "))
                    }
                } else {
                    "false".to_owned()
                }
            }
        }
    }
}

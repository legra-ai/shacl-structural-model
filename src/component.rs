//! SHACL Core constraint components.

use std::fmt;

use iri_rfc3987::Iri;

use crate::{CustomConstraint, ExpressionConstraint, NodeKind, ShaclPath, SparqlConstraint};

/// The default value type used by [`Component`].
pub type DefaultValue = crate::DefaultTerm;

/// A Core, SPARQL, or AF constraint declaration applied to a shape.
///
/// Integer count and length parameters use `Count`, which defaults to the
/// caller's RDF value type. Consumers may instead supply a compiled comparison
/// type without imposing a machine-integer limit on the structural model.
/// Regex flags use an independent `Flags` type, also defaulting to `Value`.
/// Consumers can retain an RDF reference beside a constant-size flag summary.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Value: serde::Serialize, Count: serde::Serialize, Flags: serde::Serialize",
        deserialize = "Value: serde::Deserialize<'de>, Count: serde::Deserialize<'de>, Flags: serde::Deserialize<'de>"
    )
)]
pub enum Component<Value = DefaultValue, Count = Value, Flags = Value> {
    /// `sh:class` — the value must be an instance of the class.
    Class(Iri<String>),
    /// `sh:datatype` — the value must have the given datatype.
    Datatype(Iri<String>),
    /// `sh:nodeKind` — the value must have the given RDF node kind.
    NodeKind(NodeKind),
    /// `sh:minCount`.
    MinCount(Count),
    /// `sh:maxCount`.
    MaxCount(Count),
    /// `sh:minExclusive`.
    MinExclusive(Value),
    /// `sh:minInclusive`.
    MinInclusive(Value),
    /// `sh:maxExclusive`.
    MaxExclusive(Value),
    /// `sh:maxInclusive`.
    MaxInclusive(Value),
    /// `sh:minLength`.
    MinLength(Count),
    /// `sh:maxLength`.
    MaxLength(Count),
    /// `sh:pattern` with optional `sh:flags`.
    Pattern {
        /// The regular expression pattern.
        pattern: String,
        /// Original RDF flags value, or a caller-defined compiled representation.
        flags: Option<Flags>,
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
    And(Vec<Value>),
    /// `sh:or`.
    Or(Vec<Value>),
    /// `sh:not`.
    Not(Value),
    /// `sh:xone`.
    Xone(Vec<Value>),
    /// `sh:node`.
    Node(Value),
    /// `sh:qualifiedValueShape` with optional min/max counts.
    QualifiedValueShape {
        /// The shape to count conforming values against.
        shape: Value,
        /// Minimum number of conforming values.
        min_count: Option<Count>,
        /// Maximum number of conforming values.
        max_count: Option<Count>,
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
    /// A `sh:sparql` constraint declaration.
    Sparql(Box<SparqlConstraint<Value>>),
    /// A `sh:expression` constraint declaration.
    Expression(Box<ExpressionConstraint<Value>>),
    /// A use of a separately declared custom component.
    Custom(CustomConstraint<Value>),
}

impl<Value, Count, Flags> Component<Value, Count, Flags> {
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
            Self::Sparql(_) => "sparql",
            Self::Expression(_) => "expression",
            Self::Custom(_) => "custom",
        }
    }
}

impl<Value: fmt::Debug, Count: fmt::Debug, Flags: fmt::Debug> Component<Value, Count, Flags> {
    /// Return a human-readable display value for this component.
    #[must_use]
    pub fn display_value(&self) -> String {
        match self {
            Self::Class(iri) | Self::Datatype(iri) => crate::local_name(iri),
            Self::NodeKind(kind) => format!("{kind:?}"),
            Self::MinCount(count)
            | Self::MaxCount(count)
            | Self::MinLength(count)
            | Self::MaxLength(count) => format!("{count:?}"),
            Self::MinExclusive(value)
            | Self::MinInclusive(value)
            | Self::MaxExclusive(value)
            | Self::MaxInclusive(value)
            | Self::HasValue(value)
            | Self::Not(value)
            | Self::Node(value) => format!("{value:?}"),
            Self::Pattern { pattern, flags } => match flags {
                Some(flags) => format!("/{pattern}/{flags:?}"),
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
                .map(|value| format!("{value:?}"))
                .collect::<Vec<_>>()
                .join(", "),
            Self::QualifiedValueShape {
                shape,
                min_count,
                max_count,
                ..
            } => {
                let mut parts = vec![format!("{shape:?}")];
                if let Some(min) = min_count {
                    parts.push(format!("min={min:?}"));
                }
                if let Some(max) = max_count {
                    parts.push(format!("max={max:?}"));
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
            Self::Sparql(constraint) => format!("{:?}", constraint.select.text),
            Self::Expression(constraint) => format!("{:?}", constraint.expression.node),
            Self::Custom(constraint) => crate::local_name(&constraint.component),
        }
    }
}

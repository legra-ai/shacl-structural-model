//! The constraint component declaration enum.

use iri_rfc3987::Iri;

use super::{Closedness, NamedOrList};
use crate::{
    CustomConstraint, ExpressionConstraint, LanguageTag, NodeExpression, NodeKind, ShaclPath,
    SparqlConstraint,
};

/// The default value type used by [`Component`].
pub type DefaultValue = crate::DefaultTerm;

/// A Core, SPARQL, or AF constraint declaration applied to a shape.
///
/// Integer count and length parameters use `Count`, which defaults to the
/// caller's RDF value type. Consumers may instead supply a compiled comparison
/// type without imposing a machine-integer limit on the structural model.
/// Regex flags and patterns use independent `Flags` and `Pattern` types,
/// also defaulting to `Value`. Consumers may retain an RDF reference beside
/// a compiled representation without requiring an owned lexical string.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Value: serde::Serialize, Count: serde::Serialize, Flags: serde::Serialize, Pattern: serde::Serialize, Named: serde::Serialize",
        deserialize = "Value: serde::Deserialize<'de>, Count: serde::Deserialize<'de>, Flags: serde::Deserialize<'de>, Pattern: serde::Deserialize<'de>, Named: serde::Deserialize<'de>"
    )
)]
pub enum Component<
    Value = DefaultValue,
    Count = Value,
    Flags = Value,
    Pattern = Value,
    Named = Iri<String>,
> {
    /// `sh:class` — the value must be an instance of the class.
    Class(Named),
    /// `sh:datatype` — the value must have the given datatype.
    Datatype(Named),
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
        /// Original RDF pattern value, or a caller-defined compiled representation.
        pattern: Pattern,
        /// Original RDF flags value, or a caller-defined compiled representation.
        flags: Option<Flags>,
    },
    /// `sh:languageIn`, with each member checked as a BCP 47 language tag.
    LanguageIn(Vec<LanguageTag>),
    /// `sh:uniqueLang`.
    UniqueLang(bool),
    /// `sh:equals`.
    Equals(ShaclPath<Named>),
    /// `sh:disjoint`.
    Disjoint(ShaclPath<Named>),
    /// `sh:lessThan`.
    LessThan(ShaclPath<Named>),
    /// `sh:lessThanOrEquals`.
    LessThanOrEquals(ShaclPath<Named>),
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
    ///
    /// SHACL 1.2 Core, Working Draft 16 November 2026, section 6.10.9.1, admits
    /// `true`, `false` or `sh:ByTypes`; see [`Closedness`].
    Closed {
        /// The `sh:closed` value.
        closedness: Closedness,
        /// Properties exempt from the closure check.
        ignored_properties: Vec<Named>,
    },
    /// A `sh:sparql` constraint declaration.
    Sparql(Box<SparqlConstraint<Value>>),
    /// A `sh:expression` constraint (SHACL 1.2 Node Expressions section 7.1).
    Expression(Box<ExpressionConstraint<Value, Named>>),
    /// A use of a separately declared custom component.
    Custom(CustomConstraint<Value, Named>),
    /// `sh:singleLine` from SHACL 1.2 Core, Working Draft 16 November 2026, section 6.10.4.4.
    ///
    /// When true, literal values must not contain U+000A through U+000D.
    /// False explicitly permits multiline values. This declaration is at risk
    /// in the draft; representing it does not implement constraint evaluation.
    SingleLine(bool),
    /// `sh:memberShape` from SHACL 1.2 Core, Working Draft 16 November 2026, section 6.10.5.1.
    ///
    /// Each value must be a well-formed SHACL list whose members conform to
    /// this caller-owned shape reference. The consumer checks and evaluates it.
    MemberShape(Value),
    /// Draft `sh:minListLength` (section 6.10.5.2), using the independent caller-owned count type.
    ///
    /// The consumer checks the non-negative integer parameter and list structure.
    MinListLength(Count),
    /// Draft `sh:maxListLength` (section 6.10.5.3), using the independent caller-owned count type.
    ///
    /// The consumer checks the non-negative integer parameter and list structure.
    MaxListLength(Count),
    /// Draft `sh:uniqueMembers` (section 6.10.5.4) — require distinct members when true.
    ///
    /// Both true and false require a well-formed SHACL list. False relaxes only
    /// the uniqueness check, not list well-formedness. Evaluation is consumer-owned.
    UniqueMembers(bool),
    /// Draft `sh:subsetOf` (SHACL 1.2 Core, Working Draft 16 November 2026,
    /// section 6.10.6.3) — every value node must also be reachable from the
    /// focus node via this path.
    SubsetOf(ShaclPath<Named>),
    /// Draft `sh:someValue` (section 6.10.8.3) — at least one value node must
    /// conform to this caller-owned shape reference.
    SomeValue(Value),
    /// Draft `sh:reifierShape` with optional `sh:reificationRequired`
    /// (section 6.10.8.5).
    ///
    /// Every reifier of each value triple must conform to the node shape. An
    /// absent `sh:reificationRequired` stays `None`, distinct from explicit
    /// `false`. Reifier lookup and evaluation are consumer-owned.
    ReifierShape {
        /// The node shape that reifiers must conform to.
        shape: Value,
        /// The optional `sh:reificationRequired` boolean.
        reification_required: Option<bool>,
    },
    /// Draft `sh:rootClass` (section 6.10.9.4) — each value node must be one of
    /// these classes or a transitive subclass. The IRI or SHACL-list form is kept.
    RootClass(NamedOrList<Named>),
    /// Draft `sh:uniqueValuesFor` (section 6.10.9.5) — no two target nodes may
    /// share the same combination of values for these properties. The IRI or
    /// SHACL-list form is kept.
    UniqueValuesFor(NamedOrList<Named>),
    /// `sh:nodeByExpression` (SHACL 1.2 Node Expressions, Working Draft
    /// 16 November 2026, section 7.2): each value node must conform to every
    /// shape IRI the expression yields with that value node as focus node.
    NodeByExpression(Box<NodeExpression<Value, Named>>),
}

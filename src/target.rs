//! SHACL target declarations.

use iri_rfc3987::Iri;

use crate::{NodeExpression, Parameter, ParameterValues, SparqlQuery, local_name};

/// A target declaration on a node or property shape.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Value: serde::Serialize, Named: serde::Serialize",
        deserialize = "Value: serde::Deserialize<'de>, Named: serde::Deserialize<'de>"
    )
)]
pub enum Target<Value = crate::DefaultTerm, Named = Iri<String>> {
    /// `sh:targetNode` — the output nodes of this node expression are focus nodes.
    ///
    /// SHACL 1.2 Core, Working Draft 16 November 2026, section 3.1.3.1. A
    /// Core node target is a constant expression ([`NodeExpression::constant`]).
    Node(NodeExpression<Value, Named>),
    /// `sh:targetClass` — instances of the class are focus nodes.
    Class(Named),
    /// `sh:targetSubjectsOf` — subjects of the predicate are focus nodes.
    SubjectsOf(Named),
    /// `sh:targetObjectsOf` — objects of the predicate are focus nodes.
    ObjectsOf(Named),
    /// An implicit class target (Core section 3.1.3.3): the shape is also a class.
    ImplicitClass {
        /// The shape's own class term.
        class: Value,
        /// Whether `rdfs:Class` typing or `sh:ShapeClass` declared it.
        declaration: ClassDeclaration,
    },
    /// `sh:targetWhere` (Core section 3.1.3.6): nodes conforming to this shape.
    Where(Value),
    /// A direct SHACL-AF SPARQL target.
    Sparql(Box<SparqlTarget<Value>>),
    /// An application of a separately declared target type.
    Custom(CustomTarget<Value, Named>),
}

impl<Value, Named> Target<Value, Named> {
    /// Return the SHACL target kind name.
    #[must_use]
    pub const fn kind_name(&self) -> &'static str {
        match self {
            Self::Node(_) => "targetNode",
            Self::Class(_) => "targetClass",
            Self::SubjectsOf(_) => "targetSubjectsOf",
            Self::ObjectsOf(_) => "targetObjectsOf",
            Self::ImplicitClass { .. } => "implicitClass",
            Self::Where(_) => "targetWhere",
            Self::Sparql(_) => "SPARQLTarget",
            Self::Custom(_) => "target",
        }
    }

    /// Return an IRI-valued Core class or predicate target, when applicable.
    ///
    /// Node and implicit-class targets use the caller's term representation,
    /// even for IRI nodes; this helper does not inspect those opaque values.
    #[must_use]
    pub fn iri(&self) -> Option<&Named> {
        match self {
            Self::Class(iri) | Self::SubjectsOf(iri) | Self::ObjectsOf(iri) => Some(iri),
            Self::Node(_)
            | Self::ImplicitClass { .. }
            | Self::Where(_)
            | Self::Sparql(_)
            | Self::Custom(_) => None,
        }
    }
}

impl<Value: std::fmt::Debug, Named: AsRef<str>> Target<Value, Named> {
    /// Return a human-readable label, not an RDF serialization.
    #[must_use]
    pub fn display(&self) -> String {
        match self {
            Self::Class(iri) | Self::SubjectsOf(iri) | Self::ObjectsOf(iri) => local_name(iri),
            Self::Node(expression) => expression.display(),
            Self::ImplicitClass { class: value, .. } | Self::Where(value) => format!("{value:?}"),
            Self::Sparql(target) => format!("{:?}", target.id),
            Self::Custom(target) => format!("{:?}", target.target_type),
        }
    }
}

/// The syntax that made a shape an implicit class target (Core section 3.1.3.3).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum ClassDeclaration {
    /// The shape is a SHACL instance of `rdfs:Class` and of a shape class.
    RdfsClass,
    /// The shape is a SHACL instance of `sh:ShapeClass`.
    ShapeClass,
}

/// One `sh:shape` triple of a data graph (Core section 3.1.3.7).
///
/// An explicit shape target is declared in the data graph, not on the shape,
/// so it is an independent record that a consumer streams beside shapes.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct ExplicitShapeTarget<Value = crate::DefaultTerm, Named = Iri<String>> {
    /// The data-graph subject that is a target of the shape.
    pub node: Value,
    /// The shape IRI.
    pub shape: Named,
}

/// A direct `sh:SPARQLTarget` declaration.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct SparqlTarget<Value = crate::DefaultTerm> {
    /// The target's IRI or blank node.
    pub id: Value,
    /// The SELECT query identifying focus nodes.
    pub select: SparqlQuery<Value>,
    /// Optional ASK query testing target membership.
    pub ask: Option<SparqlQuery<Value>>,
}

impl<Value> SparqlTarget<Value> {
    /// Create a target without an explicit ASK query.
    #[must_use]
    pub fn new(id: Value, select: SparqlQuery<Value>) -> Self {
        Self {
            id,
            select,
            ask: None,
        }
    }
}

/// A reusable, parameterized `sh:SPARQLTargetType` declaration.
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
pub struct SparqlTargetType<
    Value = crate::DefaultTerm,
    Count = Value,
    Flags = Value,
    Pattern = Value,
    Named = Iri<String>,
> {
    /// The target type's IRI or blank-node term.
    pub id: Value,
    /// Parameter declarations in caller order.
    pub parameters: Vec<Parameter<Value, Count, Flags, Pattern, Named>>,
    /// The SELECT query identifying focus nodes.
    pub select: SparqlQuery<Value>,
    /// Optional ASK query testing target membership.
    pub ask: Option<SparqlQuery<Value>>,
    /// RDF `sh:labelTemplate` literals.
    pub label_templates: Vec<Value>,
}

impl<Value, Count, Flags, Pattern, Named> SparqlTargetType<Value, Count, Flags, Pattern, Named> {
    /// Create a target type with no parameters, ASK query, or label templates.
    #[must_use]
    pub fn new(id: Value, select: SparqlQuery<Value>) -> Self {
        Self {
            id,
            select,
            ask: None,
            parameters: Vec::new(),
            label_templates: Vec::new(),
        }
    }
}

/// One use of a custom target type, with its supplied RDF parameter values.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct CustomTarget<Value = crate::DefaultTerm, Named = Iri<String>> {
    /// The target instance's IRI or blank node.
    pub id: Value,
    /// An IRI or blank-node reference to a separately supplied target type.
    pub target_type: Value,
    /// Supplied values; consumers check target parameter cardinality.
    pub parameters: Vec<ParameterValues<Value, Named>>,
}

//! SHACL target declarations.

use iri_rfc3987::Iri;

use crate::{Parameter, ParameterValues, SparqlQuery, local_name};

/// A target declaration on a node or property shape.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Target<Value = crate::DefaultTerm> {
    /// `sh:targetNode` — the given node is a focus node.
    Node(Value),
    /// `sh:targetClass` — instances of the class are focus nodes.
    Class(Iri<String>),
    /// `sh:targetSubjectsOf` — subjects of the predicate are focus nodes.
    SubjectsOf(Iri<String>),
    /// `sh:targetObjectsOf` — objects of the predicate are focus nodes.
    ObjectsOf(Iri<String>),
    /// An implicit class target.
    ImplicitClass(Iri<String>),
    /// A direct SHACL-AF SPARQL target.
    Sparql(Box<SparqlTarget<Value>>),
    /// An application of a separately declared target type.
    Custom(CustomTarget<Value>),
}

impl<Value> Target<Value> {
    /// Return the SHACL target kind name.
    #[must_use]
    pub const fn kind_name(&self) -> &'static str {
        match self {
            Self::Node(_) => "targetNode",
            Self::Class(_) => "targetClass",
            Self::SubjectsOf(_) => "targetSubjectsOf",
            Self::ObjectsOf(_) => "targetObjectsOf",
            Self::ImplicitClass(_) => "implicitClass",
            Self::Sparql(_) => "SPARQLTarget",
            Self::Custom(_) => "target",
        }
    }

    /// Return an IRI-valued Core class or predicate target, when applicable.
    ///
    /// Node targets use the caller's term representation, even for IRI nodes.
    #[must_use]
    pub fn iri(&self) -> Option<&Iri<String>> {
        match self {
            Self::Class(iri)
            | Self::SubjectsOf(iri)
            | Self::ObjectsOf(iri)
            | Self::ImplicitClass(iri) => Some(iri),
            Self::Node(_) | Self::Sparql(_) | Self::Custom(_) => None,
        }
    }
}

impl<Value: std::fmt::Debug> Target<Value> {
    /// Return a human-readable label, not an RDF serialization.
    #[must_use]
    pub fn display(&self) -> String {
        match self {
            Self::Class(iri)
            | Self::SubjectsOf(iri)
            | Self::ObjectsOf(iri)
            | Self::ImplicitClass(iri) => local_name(iri),
            Self::Node(value) => format!("{value:?}"),
            Self::Sparql(target) => format!("{:?}", target.id),
            Self::Custom(target) => local_name(&target.target_type),
        }
    }
}

/// A direct `sh:SPARQLTarget` declaration.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Value: serde::Serialize, Count: serde::Serialize",
        deserialize = "Value: serde::Deserialize<'de>, Count: serde::Deserialize<'de>"
    )
)]
pub struct SparqlTargetType<Value = crate::DefaultTerm, Count = Value> {
    /// The target type IRI.
    pub id: Iri<String>,
    /// Parameter declarations in caller order.
    pub parameters: Vec<Parameter<Value, Count>>,
    /// The SELECT query identifying focus nodes.
    pub select: SparqlQuery<Value>,
    /// Optional ASK query testing target membership.
    pub ask: Option<SparqlQuery<Value>>,
    /// RDF `sh:labelTemplate` literals.
    pub label_templates: Vec<Value>,
}

impl<Value, Count> SparqlTargetType<Value, Count> {
    /// Create a target type with no parameters, ASK query, or label templates.
    #[must_use]
    pub fn new(id: Iri<String>, select: SparqlQuery<Value>) -> Self {
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CustomTarget<Value = crate::DefaultTerm> {
    /// The target instance's IRI or blank node.
    pub id: Value,
    /// The IRI of a separately supplied target type declaration.
    pub target_type: Iri<String>,
    /// Supplied values; consumers check target parameter cardinality.
    pub parameters: Vec<ParameterValues<Value>>,
}

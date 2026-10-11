//! SHACL node shapes.

use iri_rfc3987::Iri;

use crate::{Component, Severity, ShaclRule, Target};

/// A SHACL node shape.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(bound(
    serialize = "Value: serde::Serialize, Count: serde::Serialize",
    deserialize = "Value: serde::Deserialize<'de>, Count: serde::Deserialize<'de>"
))]
pub struct NodeShape<Value = crate::DefaultTerm, Count = Value> {
    /// The IRI identifying this shape.
    pub id: Iri<String>,
    /// Target declarations.
    pub targets: Vec<Target>,
    /// Constraint components applied to the focus node.
    pub components: Vec<Component<Value, Count>>,
    /// IRIs of property shapes attached to this node shape.
    pub property_shapes: Vec<Iri<String>>,
    /// Whether this shape is deactivated.
    pub deactivated: bool,
    /// The severity for results from this shape.
    pub severity: Severity,
    /// Custom validation messages.
    pub messages: Vec<String>,
    /// SHACL-AF rules attached to this shape.
    pub rules: Vec<ShaclRule>,
}

impl<Value, Count> NodeShape<Value, Count> {
    /// Create an empty node shape with the given IRI.
    #[must_use]
    pub fn new(id: Iri<String>) -> Self {
        Self {
            id,
            targets: Vec::new(),
            components: Vec::new(),
            property_shapes: Vec::new(),
            deactivated: false,
            severity: Severity::default(),
            messages: Vec::new(),
            rules: Vec::new(),
        }
    }

    /// Add a target declaration.
    #[must_use]
    pub fn with_target(mut self, target: Target) -> Self {
        self.targets.push(target);
        self
    }

    /// Add a constraint component.
    #[must_use]
    pub fn with_component(mut self, component: Component<Value, Count>) -> Self {
        self.components.push(component);
        self
    }

    /// Add a property-shape reference.
    #[must_use]
    pub fn with_property_shape(mut self, iri: Iri<String>) -> Self {
        self.property_shapes.push(iri);
        self
    }

    /// Set whether this shape is deactivated.
    #[must_use]
    pub fn with_deactivated(mut self, deactivated: bool) -> Self {
        self.deactivated = deactivated;
        self
    }

    /// Set the severity level.
    #[must_use]
    pub fn with_severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    /// Add a custom validation message.
    #[must_use]
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.messages.push(message.into());
        self
    }

    /// Return the local name of this shape's IRI.
    #[must_use]
    pub fn local_name(&self) -> String {
        crate::local_name(&self.id)
    }

    /// Iterate over class and implicit-class target IRIs.
    pub fn target_class_iris(&self) -> impl Iterator<Item = &Iri<String>> + '_ {
        self.targets.iter().filter_map(|target| match target {
            Target::Class(iri) | Target::ImplicitClass(iri) => Some(iri),
            _ => None,
        })
    }

    /// Return the number of attached property-shape references.
    #[must_use]
    pub fn property_count(&self) -> usize {
        self.property_shapes.len()
    }
}

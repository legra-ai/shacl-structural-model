//! SHACL property shapes.

use iri_rfc3987::Iri;

use crate::{Component, Severity, ShaclPath, ShaclRule};

/// A SHACL property shape.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(bound(
    serialize = "Value: serde::Serialize, Count: serde::Serialize",
    deserialize = "Value: serde::Deserialize<'de>, Count: serde::Deserialize<'de>"
))]
pub struct PropertyShape<Value = crate::DefaultTerm, Count = Value> {
    /// The IRI identifying this shape.
    pub id: Iri<String>,
    /// The property path.
    pub path: ShaclPath,
    /// Constraint components applied to value nodes.
    pub components: Vec<Component<Value, Count>>,
    /// IRIs of nested property shapes.
    pub property_shapes: Vec<Iri<String>>,
    /// Whether this shape is deactivated.
    pub deactivated: bool,
    /// The severity for results from this shape.
    pub severity: Severity,
    /// Human-readable name.
    pub name: Option<String>,
    /// Human-readable description.
    pub description: Option<String>,
    /// Display ordering hint.
    pub order: Option<f64>,
    /// Default RDF value.
    pub default_value: Option<Value>,
    /// Custom validation messages.
    pub messages: Vec<String>,
    /// SHACL-AF rules attached to this shape.
    pub rules: Vec<ShaclRule>,
}

impl<Value, Count> PropertyShape<Value, Count> {
    /// Create an empty property shape with the given IRI and path.
    #[must_use]
    pub fn new(id: Iri<String>, path: ShaclPath) -> Self {
        Self {
            id,
            path,
            components: Vec::new(),
            property_shapes: Vec::new(),
            deactivated: false,
            severity: Severity::default(),
            name: None,
            description: None,
            order: None,
            default_value: None,
            messages: Vec::new(),
            rules: Vec::new(),
        }
    }

    /// Add a constraint component.
    #[must_use]
    pub fn with_component(mut self, component: Component<Value, Count>) -> Self {
        self.components.push(component);
        self
    }

    /// Add a nested property-shape reference.
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

    /// Set the human-readable name.
    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the human-readable description.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the display order.
    #[must_use]
    pub fn with_order(mut self, order: f64) -> Self {
        self.order = Some(order);
        self
    }

    /// Set the default RDF value.
    #[must_use]
    pub fn with_default_value(mut self, value: Value) -> Self {
        self.default_value = Some(value);
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

    /// Return a display string for the property path.
    #[must_use]
    pub fn path_display(&self) -> String {
        self.path.to_string()
    }
}

//! SHACL property shapes.

use crate::{Component, Severity, ShaclPath, ShaclRule, Target};

/// A SHACL property shape.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Value: serde::Serialize, Count: serde::Serialize, Flags: serde::Serialize, Pattern: serde::Serialize",
        deserialize = "Value: serde::Deserialize<'de>, Count: serde::Deserialize<'de>, Flags: serde::Deserialize<'de>, Pattern: serde::Deserialize<'de>"
    )
)]
pub struct PropertyShape<Value = crate::DefaultTerm, Count = Value, Flags = Value, Pattern = Value>
{
    /// The IRI or blank node identifying this shape.
    pub id: Value,
    /// The property path.
    pub path: ShaclPath,
    /// The original path node, when retained for source/result identity.
    pub path_node: Option<Value>,
    /// Target declarations, which may also occur on property shapes.
    pub targets: Vec<Target<Value>>,
    /// Constraint components applied to value nodes.
    pub components: Vec<Component<Value, Count, Flags, Pattern>>,
    /// IRI or blank-node references to nested property shapes.
    pub property_shapes: Vec<Value>,
    /// Whether this shape is deactivated.
    pub deactivated: bool,
    /// The severity for results from this shape.
    pub severity: Severity,
    /// RDF `sh:name` literals, including any language tags.
    pub names: Vec<Value>,
    /// RDF `sh:description` literals, including any language tags.
    pub descriptions: Vec<Value>,
    /// The original numeric `sh:order` literal for display and rule ordering.
    pub order: Option<Value>,
    /// Default RDF value.
    pub default_value: Option<Value>,
    /// RDF `sh:message` literals, including any language tags.
    pub messages: Vec<Value>,
    /// SHACL-AF rules attached to this shape.
    pub rules: Vec<ShaclRule<Value>>,
}

impl<Value, Count, Flags, Pattern> PropertyShape<Value, Count, Flags, Pattern> {
    /// Create an empty property shape with an IRI or blank-node term and path.
    #[must_use]
    pub fn new(id: Value, path: ShaclPath) -> Self {
        Self {
            id,
            path,
            path_node: None,
            targets: Vec::new(),
            components: Vec::new(),
            property_shapes: Vec::new(),
            deactivated: false,
            severity: Severity::default(),
            names: Vec::new(),
            descriptions: Vec::new(),
            order: None,
            default_value: None,
            messages: Vec::new(),
            rules: Vec::new(),
        }
    }

    /// Add a constraint component.
    #[must_use]
    pub fn with_component(mut self, component: Component<Value, Count, Flags, Pattern>) -> Self {
        self.components.push(component);
        self
    }

    /// Add a nested property-shape reference.
    #[must_use]
    pub fn with_property_shape(mut self, shape: Value) -> Self {
        self.property_shapes.push(shape);
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

    /// Add an RDF name literal.
    #[must_use]
    pub fn with_name(mut self, name: Value) -> Self {
        self.names.push(name);
        self
    }

    /// Add an RDF description literal.
    #[must_use]
    pub fn with_description(mut self, description: Value) -> Self {
        self.descriptions.push(description);
        self
    }

    /// Set the original numeric ordering literal.
    #[must_use]
    pub fn with_order(mut self, order: Value) -> Self {
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
    pub fn with_message(mut self, message: Value) -> Self {
        self.messages.push(message);
        self
    }

    /// Add a target declaration.
    #[must_use]
    pub fn with_target(mut self, target: Target<Value>) -> Self {
        self.targets.push(target);
        self
    }

    /// Attach a rule declaration.
    #[must_use]
    pub fn with_rule(mut self, rule: ShaclRule<Value>) -> Self {
        self.rules.push(rule);
        self
    }

    /// Return a display string for the property path.
    #[must_use]
    pub fn path_display(&self) -> String {
        self.path.to_string()
    }
}

//! SHACL node shapes.

use iri_rfc3987::Iri;

use crate::{Component, Severity, ShaclRule, Target};

/// A SHACL node shape.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Value: serde::Serialize, Count: serde::Serialize, Flags: serde::Serialize, Pattern: serde::Serialize, Named: serde::Serialize",
        deserialize = "Value: serde::Deserialize<'de>, Count: serde::Deserialize<'de>, Flags: serde::Deserialize<'de>, Pattern: serde::Deserialize<'de>, Named: serde::Deserialize<'de>"
    )
)]
pub struct NodeShape<
    Value = crate::DefaultTerm,
    Count = Value,
    Flags = Value,
    Pattern = Value,
    Named = Iri<String>,
> {
    /// The IRI or blank node identifying this shape.
    pub id: Value,
    /// Target declarations.
    pub targets: Vec<Target<Value, Named>>,
    /// Constraint components applied to the focus node.
    pub components: Vec<Component<Value, Count, Flags, Pattern, Named>>,
    /// IRI or blank-node references to attached property shapes.
    pub property_shapes: Vec<Value>,
    /// Whether this shape is deactivated.
    pub deactivated: bool,
    /// The severity for results from this shape.
    pub severity: Severity,
    /// RDF `sh:message` literals, including any language tags.
    pub messages: Vec<Value>,
    /// SHACL-AF rules attached to this shape.
    pub rules: Vec<ShaclRule<Value, Named>>,
    /// The original numeric `sh:order` literal for AF rule execution.
    pub order: Option<Value>,
}

impl<Value, Count, Flags, Pattern, Named> NodeShape<Value, Count, Flags, Pattern, Named> {
    /// Create an empty node shape with the given IRI or blank-node term.
    #[must_use]
    pub fn new(id: Value) -> Self {
        Self {
            id,
            targets: Vec::new(),
            components: Vec::new(),
            property_shapes: Vec::new(),
            deactivated: false,
            severity: Severity::default(),
            messages: Vec::new(),
            rules: Vec::new(),
            order: None,
        }
    }

    /// Add a target declaration.
    #[must_use]
    pub fn with_target(mut self, target: Target<Value, Named>) -> Self {
        self.targets.push(target);
        self
    }

    /// Add a constraint component.
    #[must_use]
    pub fn with_component(
        mut self,
        component: Component<Value, Count, Flags, Pattern, Named>,
    ) -> Self {
        self.components.push(component);
        self
    }

    /// Add a property-shape reference.
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

    /// Add a custom validation message.
    #[must_use]
    pub fn with_message(mut self, message: Value) -> Self {
        self.messages.push(message);
        self
    }

    /// Set the original numeric execution-order literal.
    #[must_use]
    pub fn with_order(mut self, order: Value) -> Self {
        self.order = Some(order);
        self
    }

    /// Attach a rule declaration.
    #[must_use]
    pub fn with_rule(mut self, rule: ShaclRule<Value, Named>) -> Self {
        self.rules.push(rule);
        self
    }

    /// Iterate over explicit class target IRIs.
    ///
    /// Implicit class targets use opaque `Value` terms and are not inspected.
    pub fn target_class_iris(&self) -> impl Iterator<Item = &Named> + '_ {
        self.targets.iter().filter_map(|target| match target {
            Target::Class(iri) => Some(iri),
            _ => None,
        })
    }

    /// Return the number of attached property-shape references.
    #[must_use]
    pub fn property_count(&self) -> usize {
        self.property_shapes.len()
    }
}

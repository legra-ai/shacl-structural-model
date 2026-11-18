//! `sh:PropertyGroup` declarations (Core section 6.11.8).

/// A `sh:PropertyGroup` resource, streamed beside the shapes that link to it.
///
/// SHACL 1.2 Core, Working Draft 16 November 2026, section 6.11.8. Property
/// shapes reference a group through `PropertyShape::group`; the group's own
/// `sh:order` literal orders groups within a form. Other triples about the
/// group, such as `rdfs:label`, are application data the caller retains.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct PropertyGroup<Value = crate::DefaultTerm> {
    /// The group's IRI or blank node.
    pub id: Value,
    /// The original numeric `sh:order` literal of the group.
    pub order: Option<Value>,
}

impl<Value> PropertyGroup<Value> {
    /// A group without an order.
    #[must_use]
    pub const fn new(id: Value) -> Self {
        Self { id, order: None }
    }

    /// Set the original `sh:order` literal.
    #[must_use]
    pub fn with_order(mut self, order: Value) -> Self {
        self.order = Some(order);
        self
    }
}

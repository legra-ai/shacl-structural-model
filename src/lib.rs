#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

//! Typed structural values for SHACL shapes without an RDF-store dependency.

mod component;
mod error;
mod local_name;
mod node_kind;
mod node_shape;
mod path;
mod property_shape;
mod rule;
mod severity;
mod target;

pub use component::Component;
pub use error::StructuralModelError;
pub use local_name::local_name;
pub use node_kind::NodeKind;
pub use node_shape::NodeShape;
pub use path::ShaclPath;
pub use property_shape::PropertyShape;
pub use rdf_syntax::Term as DefaultTerm;
pub use rule::ShaclRule;
pub use severity::Severity;
pub use target::Target;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_term_component_is_available() {
        let component = Component::HasValue(DefaultTerm::literal(rdf_syntax::Literal::new(
            "value",
            rdf_syntax::XSD_STRING,
        )));
        assert_eq!(component.kind_name(), "hasValue");
    }

    #[test]
    fn caller_value_type_can_be_used() {
        let component: Component<String> = Component::HasValue("value".to_owned());
        assert_eq!(component.kind_name(), "hasValue");
    }

    /// ```
    /// use shacl_structural_model::{Component, DefaultTerm, NodeShape, ShaclPath, Target};
    ///
    /// let person = "https://example.org/Person".parse().expect("valid IRI");
    /// let shape = NodeShape::<DefaultTerm>::new(
    ///     "https://example.org/PersonShape".parse().expect("valid IRI"),
    /// )
    /// .with_target(Target::Class(person))
    /// .with_component(Component::MinCount(1));
    ///
    /// assert_eq!(shape.components[0].kind_name(), "minCount");
    /// assert_eq!(ShaclPath::Predicate("https://example.org/name".parse().unwrap()).to_string(), "name");
    /// ```
    #[test]
    fn doctest_anchor() {}
}

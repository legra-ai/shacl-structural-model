#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

//! Typed structural values for SHACL shapes without an RDF-store dependency.

mod component;
mod custom_component;
mod error;
mod expression;
mod function;
mod local_name;
mod node_kind;
mod node_shape;
mod parameter;
mod path;
mod property_shape;
mod rule;
mod severity;
mod sparql;
mod target;

pub use component::Component;
pub use custom_component::{CustomConstraint, CustomConstraintComponent};
pub use error::StructuralModelError;
pub use expression::{ExpressionConstraint, ExpressionKind, NodeExpression};
pub use function::ShaclFunction;
pub use local_name::local_name;
pub use node_kind::NodeKind;
pub use node_shape::NodeShape;
pub use parameter::{Parameter, ParameterValues};
pub use path::ShaclPath;
pub use property_shape::PropertyShape;
pub use rdf_syntax::Term as DefaultTerm;
pub use rule::{RuleKind, ShaclRule, TripleRule};
pub use severity::Severity;
pub use sparql::{
    PrefixDeclaration, PrefixDeclarations, ResultAnnotation, SparqlConstraint, SparqlQuery,
    SparqlSelectOrAsk, SparqlValidator,
};
pub use target::{CustomTarget, SparqlTarget, SparqlTargetType, Target};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_term_component_is_available() {
        let component: Component = Component::HasValue(DefaultTerm::literal(
            rdf_syntax::Literal::new("value", rdf_syntax::XSD_STRING),
        ));
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
    ///     DefaultTerm::iri("https://example.org/PersonShape".parse().expect("valid IRI")),
    /// )
    /// .with_target(Target::Class(person.clone()))
    /// .with_component(Component::Class(person));
    ///
    /// assert_eq!(shape.components[0].kind_name(), "class");
    /// assert_eq!(ShaclPath::Predicate("https://example.org/name".parse().unwrap()).to_string(), "name");
    /// ```
    #[test]
    fn doctest_anchor() {}
}

//! Compiled patterns remain independent of terms, counts, and flags everywhere.

use serde::{Deserialize, Serialize};
use shacl_structural_model::{
    Component, CustomConstraintComponent, NodeShape, Parameter, PropertyShape, ShaclFunction,
    ShaclPath, SparqlQuery, SparqlSelectOrAsk, SparqlTargetType,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct CompiledPattern {
    original: String,
    // bounded: a four-byte opaque fixture, not an engine-specific automaton.
    program: [u8; 4],
}

type Constraint = Component<String, u64, u8, CompiledPattern>;
type Node = NodeShape<String, u64, u8, CompiledPattern>;
type Property = PropertyShape<String, u64, u8, CompiledPattern>;
type Declaration = Parameter<String, u64, u8, CompiledPattern>;
type Custom = CustomConstraintComponent<String, u64, u8, CompiledPattern>;
type Function = ShaclFunction<String, u64, u8, CompiledPattern>;
type TargetType = SparqlTargetType<String, u64, u8, CompiledPattern>;

fn pattern() -> Constraint {
    Constraint::Pattern {
        pattern: CompiledPattern {
            original: "urn:pattern".to_owned(),
            program: [1, 2, 3, 4],
        },
        flags: Some(1),
    }
}

fn roundtrip<T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug>(value: &T) {
    let encoded = serde_json::to_string(value).unwrap();
    assert_eq!(&serde_json::from_str::<T>(&encoded).unwrap(), value);
}

#[test]
fn shapes_retain_compiled_patterns_and_source_identity() {
    let node = Node::new("urn:node".to_owned()).with_component(pattern());
    let property = Property::new(
        "urn:property".to_owned(),
        ShaclPath::Predicate("urn:path".parse().unwrap()),
    )
    .with_component(pattern());
    roundtrip(&(node, property));
}

#[test]
fn all_parameter_owners_retain_compiled_patterns() {
    let mut parameter = Declaration::new("urn:parameter".to_owned(), "urn:path".parse().unwrap());
    parameter.components.push(pattern());
    let mut custom = Custom::new("urn:component".parse().unwrap());
    custom.parameters.push(parameter.clone());
    let query = SparqlQuery::new("urn:query".to_owned());
    let mut function = Function::new(
        "urn:function".parse().unwrap(),
        SparqlSelectOrAsk::Select(query.clone()),
    );
    function.parameters.push(parameter.clone());
    let mut target = TargetType::new("urn:target".to_owned(), query);
    target.parameters.push(parameter.clone());
    roundtrip(&(parameter, custom, function, target));
}

#[test]
fn pattern_construction_does_not_require_serialization_or_cloning() {
    struct Opaque;
    let constraint = Component::<String, u64, u8, &Opaque>::Pattern {
        pattern: &Opaque,
        flags: None,
    };
    assert_eq!(constraint.kind_name(), "pattern");
}

#[test]
fn pattern_display_uses_the_callers_debug_representation() {
    let constraint = pattern();
    assert!(constraint.display_value().contains("urn:pattern"));
    assert_eq!(constraint.kind_name(), "pattern");
}

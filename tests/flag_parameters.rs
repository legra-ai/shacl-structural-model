//! Regex flags retain caller-defined RDF values instead of requiring strings.

use shacl_structural_model::{Component, NodeShape, Parameter, PropertyShape, ShaclPath};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
enum Value {
    Resource(String),
    LiteralReference { identity: String },
}

#[test]
fn flags_preserve_opaque_rdf_values() {
    let encoded =
        r#"{"Pattern":{"pattern":"^a","flags":{"LiteralReference":{"identity":"urn:flags"}}}}"#;
    let component: Component<Value> =
        serde_json::from_str(encoded).expect("regex flags must retain the caller's RDF value");
    assert_eq!(serde_json::to_string(&component).unwrap(), encoded);
}

#[test]
fn parameter_constraints_preserve_flag_values() {
    let encoded = r#"{"id":{"Resource":"urn:parameter"},"path":"urn:value","optional":false,"order":null,"names":[],"descriptions":[],"components":[{"Pattern":{"pattern":"^a","flags":{"LiteralReference":{"identity":"urn:flags"}}}}]}"#;
    let parameter: Parameter<Value> =
        serde_json::from_str(encoded).expect("parameter constraints must retain RDF flag values");
    assert_eq!(serde_json::to_string(&parameter).unwrap(), encoded);
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct CompiledFlags {
    original: Value,
    case_insensitive: bool,
}

#[test]
fn shapes_support_independently_compiled_flags() {
    let component = Component::<Value, u64, CompiledFlags>::Pattern {
        pattern: "^a".to_owned(),
        flags: Some(CompiledFlags {
            original: Value::LiteralReference {
                identity: "urn:flags".to_owned(),
            },
            case_insensitive: true,
        }),
    };
    let node =
        NodeShape::new(Value::Resource("urn:node".to_owned())).with_component(component.clone());
    let property = PropertyShape::new(
        Value::Resource("urn:property".to_owned()),
        ShaclPath::Predicate("urn:path".parse().unwrap()),
    )
    .with_component(component);
    let encoded = serde_json::to_string(&(node.clone(), property.clone())).unwrap();
    let decoded: (
        NodeShape<Value, u64, CompiledFlags>,
        PropertyShape<Value, u64, CompiledFlags>,
    ) = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded, (node, property));
}

//! Pattern bodies retain RDF identity independently of their compiled form.

use shacl_structural_model::{Component, Parameter};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
enum Value {
    Resource(String),
    LiteralReference { identity: String },
}

#[test]
fn patterns_preserve_opaque_rdf_values() {
    let encoded =
        r#"{"Pattern":{"pattern":{"LiteralReference":{"identity":"urn:pattern"}},"flags":null}}"#;
    let component: Component<Value> =
        serde_json::from_str(encoded).expect("patterns must retain the caller's RDF value");
    assert_eq!(serde_json::to_string(&component).unwrap(), encoded);
}

#[test]
fn parameter_constraints_preserve_pattern_values() {
    let encoded = r#"{"id":{"Resource":"urn:parameter"},"path":"urn:value","optional":false,"order":null,"names":[],"descriptions":[],"components":[{"Pattern":{"pattern":{"LiteralReference":{"identity":"urn:pattern"}},"flags":null}}]}"#;
    let parameter: Parameter<Value> = serde_json::from_str(encoded)
        .expect("parameter constraints must retain RDF pattern values");
    assert_eq!(serde_json::to_string(&parameter).unwrap(), encoded);
}

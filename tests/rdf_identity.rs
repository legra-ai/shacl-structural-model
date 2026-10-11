//! Regressions for RDF identity at the structural/engine boundary.

use serde_json::{Value, json};
use shacl_structural_model::{Component, NodeShape, PropertyShape, Severity};

fn round_trip<T: serde::Serialize + serde::de::DeserializeOwned>(expected: Value) {
    let encoded = serde_json::to_string(&expected).unwrap();
    let decoded: T = serde_json::from_value(expected).unwrap();
    let actual = serde_json::to_value(decoded).unwrap();
    assert_eq!(serde_json::to_string(&actual).unwrap(), encoded);
}

fn blank(label: &str) -> Value {
    json!({"BlankNode": label})
}

fn literal(lexical: &str, language: Option<&str>) -> Value {
    json!({"Literal": {"lexical": lexical, "language": language}})
}

fn node() -> Value {
    json!({
        "id": "urn:shape", "targets": [], "components": [],
        "property_shapes": [], "deactivated": false, "severity": "Violation",
        "messages": [], "rules": [], "order": null
    })
}

#[test]
fn blank_node_shape_identifiers_and_references_survive() {
    let mut shape = node();
    shape["id"] = blank("shape");
    shape["property_shapes"] = json!([blank("property")]);
    round_trip::<NodeShape<Value>>(shape);
}

#[test]
fn logical_and_qualified_constraints_retain_blank_node_references() {
    for kind in ["And", "Or", "Xone"] {
        round_trip::<Component<Value>>(json!({kind: [blank("condition")]}));
    }
    for kind in ["Not", "Node"] {
        round_trip::<Component<Value>>(json!({kind: blank("condition")}));
    }
    round_trip::<Component<Value>>(json!({"QualifiedValueShape": {
        "shape": blank("condition"), "min_count": "-1",
        "max_count": "18446744073709551616", "disjoint": true
    }}));
}

#[test]
fn target_nodes_can_be_literals_or_blank_nodes() {
    let mut shape = node();
    shape["targets"] = json!([
        {"Node": blank("focus")}, {"Node": literal("bonjour", Some("fr"))}
    ]);
    round_trip::<NodeShape<Value>>(shape);
}

#[test]
fn messages_preserve_language_and_literal_identity() {
    let mut shape = node();
    shape["messages"] = json!([
        literal("Required", Some("en")), literal("Verplicht", Some("nl")),
        {"Literal": {"lexical": "Required", "datatype": "xsd:string", "explicit": true}},
        {"Literal": {"lexical": "Required", "datatype": "xsd:string", "explicit": false}}
    ]);
    round_trip::<NodeShape<Value>>(shape);
}

#[test]
fn node_shape_order_preserves_decimal_precision() {
    let mut shape = node();
    shape["order"] = json!({"Decimal": "+0009007199254740993.000000000000000001"});
    round_trip::<NodeShape<Value>>(shape);
}

#[test]
fn property_shapes_have_targets_and_multilingual_annotations() {
    let mut shape = node();
    shape["id"] = blank("property");
    shape["path"] = json!({"Inverse": {"Predicate": "urn:parent"}});
    shape["path_node"] = blank("inversePath");
    shape["targets"] = json!([{"Node": literal("focus", None)}]);
    shape["names"] = json!([literal("Name", Some("en")), literal("Naam", Some("nl"))]);
    shape["descriptions"] = json!([literal("Description", Some("en"))]);
    shape["order"] = json!({"Decimal": "1.0000000000000000000000001"});
    shape["default_value"] = literal("default", Some("en"));
    round_trip::<PropertyShape<Value>>(shape);
}

#[test]
fn application_defined_severities_are_valid_and_keep_their_iri() {
    let severity = Severity::from_iri("https://example.org/severity/Blocking").unwrap();
    assert_eq!(severity.as_iri(), "https://example.org/severity/Blocking");
    assert_eq!(
        serde_json::from_str::<Severity>(&serde_json::to_string(&severity).unwrap()).unwrap(),
        severity
    );
    assert!(Severity::from_iri("not an IRI").is_err());
}

#[test]
fn unrecognized_shape_fields_fail_instead_of_disappearing() {
    let mut shape = node();
    shape["unsupported_declaration"] = json!({"must": "survive"});
    assert!(serde_json::from_value::<NodeShape<Value>>(shape).is_err());
}

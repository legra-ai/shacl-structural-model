//! Regressions for RDF identity at the structural/engine boundary.
//!
//! Each value is built from typed terms and compared with checked-in golden
//! JSON and Postcard vectors in `tests/fixtures/golden`. The vectors pin the
//! serialized format: an encoding change fails here instead of silently
//! changing stored data. Regenerate deliberately with `UPDATE_GOLDEN=1`.

use std::fmt::Debug;
use std::path::PathBuf;

use iri_rfc3987::Iri;
use rdf_structural_model::{BlankNodeId, LanguageTag, Literal};
use serde::Serialize;
use serde::de::DeserializeOwned;
use shacl_structural_model::{
    Closedness, Component, DefaultTerm, NamedOrList, NodeShape, PropertyShape, Severity, ShaclPath,
    Target,
};

const XSD_INTEGER: &str = "http://www.w3.org/2001/XMLSchema#integer";
const XSD_DECIMAL: &str = "http://www.w3.org/2001/XMLSchema#decimal";
const XSD_STRING: &str = "http://www.w3.org/2001/XMLSchema#string";

fn iri(text: &str) -> Iri<String> {
    text.parse().unwrap()
}

fn blank(label: &str) -> DefaultTerm {
    DefaultTerm::BlankNode(
        BlankNodeId::try_new(iri("urn:shapes:source"), label.to_owned()).unwrap(),
    )
}

fn text(lexical: &str, language: &str) -> DefaultTerm {
    let language = LanguageTag::try_new(language.to_owned()).unwrap();
    DefaultTerm::literal(Literal::language_tagged(lexical, language))
}

fn typed(lexical: &str, datatype: &str) -> DefaultTerm {
    DefaultTerm::literal(Literal::typed(lexical, iri(datatype)).unwrap())
}

fn golden_path(name: &str, extension: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/golden")
        .join(format!("{name}.{extension}"))
}

/// Assert that `value` encodes to the golden vectors and decodes back from them.
fn assert_golden<T>(name: &str, value: &T)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    let json = serde_json::to_string_pretty(value).unwrap() + "\n";
    let binary = postcard::to_allocvec(value).unwrap();
    let (json_path, binary_path) = (golden_path(name, "json"), golden_path(name, "postcard"));
    if std::env::var_os("UPDATE_GOLDEN").is_some() {
        std::fs::write(&json_path, &json).unwrap();
        std::fs::write(&binary_path, &binary).unwrap();
    }
    let expected_json = std::fs::read_to_string(&json_path).unwrap();
    let expected_binary = std::fs::read(&binary_path).unwrap();
    assert_eq!(json, expected_json, "JSON vector {name} changed");
    assert_eq!(binary, expected_binary, "Postcard vector {name} changed");
    assert_eq!(&serde_json::from_str::<T>(&expected_json).unwrap(), value);
    assert_eq!(&postcard::from_bytes::<T>(&expected_binary).unwrap(), value);
}

#[test]
fn blank_node_shape_identifiers_and_references_survive() {
    let shape =
        NodeShape::<DefaultTerm>::new(blank("shape")).with_property_shape(blank("property"));
    assert_golden("blank_node_shape", &shape);
}

#[test]
fn logical_and_qualified_constraints_retain_blank_node_references() {
    let components: Vec<Component> = vec![
        Component::And(vec![blank("condition")]),
        Component::Or(vec![blank("condition")]),
        Component::Xone(vec![blank("condition")]),
        Component::Not(blank("condition")),
        Component::Node(blank("condition")),
        Component::QualifiedValueShape {
            shape: blank("condition"),
            min_count: Some(typed("-1", XSD_INTEGER)),
            max_count: Some(typed("18446744073709551616", XSD_INTEGER)),
            disjoint: true,
        },
    ];
    assert_golden("logical_components", &components);
}

#[test]
fn target_nodes_can_be_literals_or_blank_nodes() {
    let shape = NodeShape::<DefaultTerm>::new(DefaultTerm::iri(iri("urn:shape")))
        .with_target(Target::Node(blank("focus")))
        .with_target(Target::Node(text("bonjour", "fr")));
    assert_golden("literal_targets", &shape);
}

#[test]
fn messages_preserve_language_and_literal_identity() {
    let shape = NodeShape::<DefaultTerm>::new(DefaultTerm::iri(iri("urn:shape")))
        .with_message(text("Required", "en"))
        .with_message(text("Verplicht", "nl"))
        .with_message(typed("Required", XSD_STRING))
        .with_message(DefaultTerm::literal(Literal::string("Required")));
    assert_golden("messages", &shape);
}

#[test]
fn node_shape_order_preserves_decimal_precision() {
    let shape = NodeShape::<DefaultTerm>::new(DefaultTerm::iri(iri("urn:shape"))).with_order(
        typed("+0009007199254740993.000000000000000001", XSD_DECIMAL),
    );
    assert_golden("decimal_order", &shape);
}

#[test]
fn property_shapes_have_targets_and_multilingual_annotations() {
    let path = ShaclPath::Inverse(Box::new(ShaclPath::Predicate(iri("urn:parent"))));
    let mut shape = PropertyShape::<DefaultTerm>::new(blank("property"), path)
        .with_target(Target::Node(DefaultTerm::literal(Literal::string("focus"))))
        .with_name(text("Name", "en"))
        .with_name(text("Naam", "nl"))
        .with_description(text("Description", "en"))
        .with_order(typed("1.0000000000000000000000001", XSD_DECIMAL))
        .with_default_value(text("default", "en"));
    shape.path_node = Some(blank("inversePath"));
    assert_golden("property_shape", &shape);
}

#[test]
fn application_defined_severities_are_valid_and_keep_their_iri() {
    let severity = Severity::from_iri("https://example.org/severity/Blocking").unwrap();
    assert_eq!(severity.as_iri(), "https://example.org/severity/Blocking");
    assert_golden("custom_severity", &severity);
    assert!(Severity::from_iri("not an IRI").is_err());
}

#[test]
fn unrecognized_shape_fields_fail_instead_of_disappearing() {
    let shape = NodeShape::<DefaultTerm>::new(DefaultTerm::iri(iri("urn:shape")));
    let serde_json::Value::Object(mut fields) = serde_json::to_value(&shape).unwrap() else {
        panic!("a node shape serializes as a JSON object");
    };
    fields.insert(
        "unsupported_declaration".to_owned(),
        serde_json::Value::String("must survive".to_owned()),
    );
    let encoded = serde_json::to_string(&fields).unwrap();
    assert!(serde_json::from_str::<NodeShape<DefaultTerm>>(&encoded).is_err());
}

#[test]
fn shacl12_core_components_and_severities_keep_their_encoding() {
    let components: Vec<Component> = vec![
        Component::Closed {
            closedness: Closedness::ByTypes,
            ignored_properties: vec![iri("https://example.org/note")],
        },
        Component::Closed {
            closedness: Closedness::Open,
            ignored_properties: vec![],
        },
        Component::SubsetOf(ShaclPath::Predicate(iri("https://example.org/child"))),
        Component::SomeValue(blank("duck")),
        Component::ReifierShape {
            shape: blank("provenance"),
            reification_required: Some(false),
        },
        Component::RootClass(NamedOrList::One(iri("https://example.org/Animal"))),
        Component::UniqueValuesFor(NamedOrList::List(vec![
            iri("https://example.org/aCode"),
            iri("https://example.org/bCode"),
        ])),
    ];
    assert_golden("core12_components", &components);
    assert_golden("core12_severities", &[Severity::Debug, Severity::Trace]);
}

//! Core constraint components and severities from SHACL 1.2 Core,
//! Working Draft 16 November 2026 (sections 3.1.4, 6.10.6.3, 6.10.8.3,
//! 6.10.8.5, 6.10.9.1, 6.10.9.4 and 6.10.9.5).

use iri_rfc3987::Iri;
use rdf_structural_model::BlankNodeId;
use serde::{Deserialize, Serialize};
use shacl_structural_model::{
    Closedness, Component, DefaultTerm, NamedOrList, PropertyShape, Severity, ShaclPath, vocabulary,
};

fn roundtrip<T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug>(value: &T) {
    let json = serde_json::to_vec(value).unwrap();
    assert_eq!(&serde_json::from_slice::<T>(&json).unwrap(), value);
    let binary = postcard::to_allocvec(value).unwrap();
    assert_eq!(&postcard::from_bytes::<T>(&binary).unwrap(), value);
}

fn iri(local: &str) -> Iri<String> {
    format!("https://example.org/{local}").parse().unwrap()
}

fn blank(label: &str) -> DefaultTerm {
    DefaultTerm::BlankNode(
        BlankNodeId::try_new("urn:shapes".parse().unwrap(), label.to_owned()).unwrap(),
    )
}

#[test]
fn debug_and_trace_are_standard_non_violation_severities() {
    for (severity, iri, label) in [
        (Severity::Debug, vocabulary::DEBUG, "Debug"),
        (Severity::Trace, vocabulary::TRACE, "Trace"),
    ] {
        assert_eq!(Severity::from_iri(iri).unwrap(), severity);
        assert_eq!(severity.as_iri(), iri);
        assert_eq!(severity.as_str(), label);
        assert_eq!(severity.is_violation(), Some(false));
        roundtrip(&severity);
    }
    assert_eq!(Severity::Info.is_violation(), Some(true));
    assert_eq!(Severity::Custom(iri("Blocking")).is_violation(), None);
    assert!(Severity::Info < Severity::Debug && Severity::Debug < Severity::Trace);
    assert!(Severity::Trace < Severity::Custom(iri("Blocking")));
}

#[test]
fn closedness_keeps_true_false_and_by_types() {
    assert_eq!(Closedness::from_boolean(true), Closedness::Closed);
    assert_eq!(Closedness::from_boolean(false), Closedness::Open);
    assert_eq!(
        Closedness::from_iri(vocabulary::BY_TYPES),
        Some(Closedness::ByTypes)
    );
    assert_eq!(Closedness::from_iri(vocabulary::TRACE), None);
    assert!(!Closedness::Open.is_closed());
    assert!(Closedness::ByTypes.is_closed());
    assert_eq!(Closedness::ByTypes.as_rdf_lexical(), vocabulary::BY_TYPES);
    for closedness in [Closedness::Open, Closedness::Closed, Closedness::ByTypes] {
        let component: Component = Component::Closed {
            closedness,
            ignored_properties: vec![iri("note")],
        };
        assert_eq!(component.kind_name(), "closed");
        roundtrip(&component);
    }
    let by_types: Component = Component::Closed {
        closedness: Closedness::ByTypes,
        ignored_properties: vec![iri("note")],
    };
    assert_eq!(by_types.display_value(), "ByTypes (ignore: note)");
}

#[test]
fn new_components_report_kind_names_and_display_values() {
    let cases: [(Component, &str, &str); 7] = [
        (
            Component::SubsetOf(ShaclPath::Predicate(iri("child"))),
            "subsetOf",
            "child",
        ),
        (Component::SomeValue(blank("duck")), "someValue", ""),
        (
            Component::ReifierShape {
                shape: blank("provenance"),
                reification_required: Some(true),
            },
            "reifierShape",
            "required=true",
        ),
        (
            Component::ReifierShape {
                shape: blank("provenance"),
                reification_required: None,
            },
            "reifierShape",
            "",
        ),
        (
            Component::RootClass(NamedOrList::One(iri("Animal"))),
            "rootClass",
            "Animal",
        ),
        (
            Component::RootClass(NamedOrList::List(vec![iri("Animal"), iri("Plant")])),
            "rootClass",
            "Animal, Plant",
        ),
        (
            Component::UniqueValuesFor(NamedOrList::List(vec![iri("aCode"), iri("bCode")])),
            "uniqueValuesFor",
            "aCode, bCode",
        ),
    ];
    for (component, kind, display) in cases {
        assert_eq!(component.kind_name(), kind);
        assert!(component.display_value().contains(display));
        roundtrip(&component);
    }
}

#[test]
fn single_iri_and_one_member_list_stay_distinct() {
    let one = NamedOrList::One(iri("id"));
    let list = NamedOrList::List(vec![iri("id")]);
    assert_ne!(one, list);
    assert!(!one.is_list() && list.is_list());
    assert_eq!(one.len(), list.len());
    assert!(one.iter().eq(list.iter()));
    assert!(NamedOrList::<Iri<String>>::List(vec![]).is_empty());
    roundtrip(&(one, list));
}

#[test]
fn reification_required_absent_differs_from_explicit_false() {
    let shapes = [None, Some(false), Some(true)].map(|reification_required| {
        PropertyShape::<DefaultTerm>::new(blank("age"), ShaclPath::Predicate(iri("age")))
            .with_component(Component::ReifierShape {
                shape: blank("provenance"),
                reification_required,
            })
    });
    assert_ne!(shapes[0], shapes[1]);
    roundtrip(&shapes);
}

#[test]
fn new_declarations_reject_unknown_fields_and_wrong_types() {
    for invalid in [
        r#"{"ReifierShape":{"shape":"s","reification_required":null,"extra":1}}"#,
        r#"{"ReifierShape":{"shape":"s","reification_required":1}}"#,
        r#"{"Closed":{"closedness":"Closed","ignored_properties":[],"is_closed":true}}"#,
        r#"{"Closed":{"closedness":true,"ignored_properties":[]}}"#,
        r#"{"RootClass":"urn:a"}"#,
        r#"{"UniqueValuesFor":{"Many":["urn:a"]}}"#,
    ] {
        assert!(
            serde_json::from_str::<Component<String, String, String, String, String>>(invalid)
                .is_err(),
            "{invalid}"
        );
    }
    assert!(serde_json::from_str::<Severity>(r#""Fatal""#).is_err());
}

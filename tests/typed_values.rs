//! Typed parameters, error context, ordering and vocabulary constants.

use shacl_structural_model::{
    Component, LanguageTag, LexicalRole, NodeKind, Severity, StructuralModelError, vocabulary,
};

#[test]
fn language_in_members_are_checked_on_construction_and_decoding() {
    let tags = ["en", "nl-BE", "zh-Hant-TW", "x-private"]
        .map(|tag| LanguageTag::try_new(tag).unwrap())
        .to_vec();
    let component: Component<String> = Component::LanguageIn(tags);
    assert_eq!(
        component.display_value(),
        "en, nl-BE, zh-Hant-TW, x-private"
    );
    let json = serde_json::to_string(&component).unwrap();
    assert_eq!(
        json,
        r#"{"LanguageIn":["en","nl-BE","zh-Hant-TW","x-private"]}"#
    );
    assert_eq!(
        serde_json::from_str::<Component<String>>(&json).unwrap(),
        component
    );
    let binary = postcard::to_allocvec(&component).unwrap();
    assert_eq!(
        postcard::from_bytes::<Component<String>>(&binary).unwrap(),
        component
    );

    let malformed: Component<String> = Component::LanguageIn(Vec::new());
    let json = serde_json::to_string(&malformed)
        .unwrap()
        .replace("[]", r#"["en_US"]"#);
    assert!(serde_json::from_str::<Component<String>>(&json).is_err());
    let binary = postcard::to_allocvec(&["en_US".to_owned()].to_vec()).unwrap();
    assert!(postcard::from_bytes::<Vec<LanguageTag>>(&binary).is_err());
    assert_eq!(
        LanguageTag::try_new("en_US"),
        Err(StructuralModelError::InvalidSyntax {
            lexical: "en_US".to_owned(),
            expected: LexicalRole::LanguageTag,
        })
    );
}

#[test]
fn decoding_errors_carry_the_lexical_value_and_expected_role() {
    assert_eq!(
        NodeKind::from_iri("http://www.w3.org/ns/shacl#Nothing"),
        Err(StructuralModelError::UnknownTerm {
            lexical: "http://www.w3.org/ns/shacl#Nothing".to_owned(),
            expected: LexicalRole::NodeKindIri,
        })
    );
    let error = Severity::from_iri("not an IRI").unwrap_err();
    assert_eq!(
        error,
        StructuralModelError::InvalidSyntax {
            lexical: "not an IRI".to_owned(),
            expected: LexicalRole::SeverityIri,
        }
    );
    assert_eq!(error.to_string(), "invalid SHACL severity IRI: not an IRI");
}

#[test]
fn severities_sort_from_most_to_least_severe() {
    let custom = Severity::from_iri("https://example.org/severity/Blocking").unwrap();
    let mut severities = vec![
        custom.clone(),
        Severity::Info,
        Severity::Violation,
        Severity::Warning,
    ];
    severities.sort();
    assert_eq!(
        severities,
        [
            Severity::Violation,
            Severity::Warning,
            Severity::Info,
            custom
        ]
    );
}

#[test]
fn value_types_sort_canonically() {
    let mut components: Vec<Component<String>> = vec![
        Component::MinCount("2".to_owned()),
        Component::Class("urn:b".parse().unwrap()),
        Component::MinCount("1".to_owned()),
        Component::Class("urn:a".parse().unwrap()),
    ];
    components.sort();
    assert_eq!(
        components,
        [
            Component::Class("urn:a".parse().unwrap()),
            Component::Class("urn:b".parse().unwrap()),
            Component::MinCount("1".to_owned()),
            Component::MinCount("2".to_owned()),
        ]
    );
}

#[test]
fn standard_iris_come_from_the_vocabulary_module() {
    for kind in [
        NodeKind::Iri,
        NodeKind::BlankNode,
        NodeKind::Literal,
        NodeKind::BlankNodeOrIri,
        NodeKind::BlankNodeOrLiteral,
        NodeKind::IriOrLiteral,
    ] {
        assert!(kind.as_iri().starts_with(vocabulary::NAMESPACE));
        assert_eq!(NodeKind::from_iri(kind.as_iri()), Ok(kind));
    }
    for severity in [Severity::Violation, Severity::Warning, Severity::Info] {
        assert_eq!(Severity::from_iri(severity.as_iri()).unwrap(), severity);
    }
    assert_eq!(
        vocabulary::VIOLATION,
        "http://www.w3.org/ns/shacl#Violation"
    );
}

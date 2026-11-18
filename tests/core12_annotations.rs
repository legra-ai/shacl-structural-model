//! Non-validating shape characteristics, the triple-term node kind and
//! expression-valued deactivation from SHACL 1.2 Core, Working Draft 16
//! November 2026 (sections 3.1.6, 6.10.1.3 and 6.11), and SHACL 1.2
//! SPARQL-Related Features, Working Draft 22 October 2026 (section 3.2).

#[path = "support/golden.rs"]
mod golden;

use iri_rfc3987::Iri;
use rdf_structural_model::{LanguageTag as RdfLanguageTag, Literal};
use shacl_structural_model::{
    CORE_DRAFT, CodeIdentifier, Component, ConstraintDeactivation, DefaultTerm, ExpressionForm,
    Intent, LexicalRole, NodeExpression, NodeKind, NodeShape, PropertyGroup, PropertyShape,
    ShaclPath, ShapeAnnotations, SparqlConstraint, SparqlQuery, StructuralModelError, vocabulary,
};

fn iri(local: &str) -> Iri<String> {
    format!("https://example.org/{local}").parse().unwrap()
}

fn english(text: &str) -> DefaultTerm {
    DefaultTerm::literal(Literal::language_tagged(
        text,
        RdfLanguageTag::try_new("en".to_owned()).unwrap(),
    ))
}

fn boolean(value: bool) -> NodeExpression {
    NodeExpression::constant(DefaultTerm::literal(
        Literal::typed(
            value.to_string(),
            "http://www.w3.org/2001/XMLSchema#boolean".parse().unwrap(),
        )
        .unwrap(),
    ))
}

/// Core example 77: a birth-date property shape with intents, one formalized.
fn birth_date() -> PropertyShape {
    let annotations = ShapeAnnotations::new()
        .with_intent(
            Intent::new(english("Each person has at most one birth date.")).with_formalized(true),
        )
        .with_intent(Intent::new(english(
            "The birth date must not be in the future.",
        )))
        .with_agent_instruction(english("Look up the legal date of birth."))
        .with_code_identifier(CodeIdentifier::try_new("birth_date").unwrap())
        .with_code_type("http://www.w3.org/2001/XMLSchema#string".parse().unwrap())
        .with_unit(DefaultTerm::literal(Literal::string("d")));
    PropertyShape::new(
        DefaultTerm::iri(iri("Person-birthDate")),
        ShaclPath::Predicate(iri("birthDate")),
    )
    .with_annotations(annotations)
    .with_group(DefaultTerm::iri(iri("NameGroup")))
    .with_order(DefaultTerm::literal(Literal::string("0")))
}

#[test]
fn shapes_retain_intents_instructions_code_hints_units_and_groups() {
    let shape = birth_date();
    let annotations = &shape.annotations;
    assert_eq!(annotations.intents.len(), 2);
    assert!(annotations.intents[0].is_formalized());
    assert_eq!(annotations.intents[1].formalized, None);
    assert_ne!(
        Intent::new(english("x")),
        Intent::new(english("x")).with_formalized(false)
    );
    assert_eq!(
        annotations
            .code_identifier
            .as_ref()
            .map(CodeIdentifier::as_str),
        Some("birth_date")
    );
    assert!(!annotations.is_empty());
    assert!(ShapeAnnotations::<DefaultTerm>::new().is_empty());
    assert_eq!(shape.group, Some(DefaultTerm::iri(iri("NameGroup"))));
    let node = NodeShape::<DefaultTerm>::new(DefaultTerm::iri(iri("FootballPlayer")))
        .with_annotations(
            ShapeAnnotations::new()
                .with_agent_instruction(english("This is about soccer."))
                .with_unit(DefaultTerm::iri(iri("CentiM"))),
        );
    golden::assert_golden("core12_annotations", &(shape, node));
    let group = PropertyGroup::<DefaultTerm>::new(DefaultTerm::iri(iri("NameGroup")))
        .with_order(DefaultTerm::literal(Literal::string("0")));
    golden::round_trip(&group);
    assert!(CORE_DRAFT.ends_with("WD-shacl12-core-20261116/"));
}

#[test]
fn code_identifiers_are_checked_on_construction_and_decoding() {
    for valid in ["a", "_", "Person_name2", "_x9"] {
        let identifier = CodeIdentifier::try_new(valid).unwrap();
        golden::round_trip(&identifier);
        assert_eq!(identifier.to_string(), valid);
    }
    for invalid in ["", "9lives", "first-name", "naïve", "a b", "a.b"] {
        assert_eq!(
            CodeIdentifier::try_new(invalid),
            Err(StructuralModelError::InvalidSyntax {
                lexical: invalid.to_owned(),
                expected: LexicalRole::CodeIdentifier,
            })
        );
        let json = serde_json::to_string(invalid).unwrap();
        assert!(serde_json::from_str::<CodeIdentifier>(&json).is_err());
        let bytes = postcard::to_allocvec(invalid).unwrap();
        assert!(postcard::from_bytes::<CodeIdentifier>(&bytes).is_err());
    }
}

#[test]
fn triple_term_is_the_seventh_node_kind_and_keeps_earlier_indexes() {
    assert_eq!(
        NodeKind::from_iri(vocabulary::TRIPLE_TERM),
        Ok(NodeKind::TripleTerm)
    );
    assert_eq!(NodeKind::TripleTerm.as_iri(), vocabulary::TRIPLE_TERM);
    assert_eq!(postcard::to_allocvec(&NodeKind::Iri).unwrap(), [0]);
    assert_eq!(postcard::to_allocvec(&NodeKind::IriOrLiteral).unwrap(), [5]);
    assert_eq!(postcard::to_allocvec(&NodeKind::TripleTerm).unwrap(), [6]);
    golden::round_trip(&Component::<DefaultTerm>::NodeKind(NodeKind::TripleTerm));
}

#[test]
fn deactivation_is_a_node_expression_on_shapes_constraints_and_reifiers() {
    let absent = NodeShape::<DefaultTerm>::new(DefaultTerm::iri(iri("Open")));
    assert_eq!(absent.deactivated, None);
    let shape = NodeShape::<DefaultTerm>::new(DefaultTerm::iri(iri("PersonShape")))
        .with_deactivated(boolean(true));
    let computed = PropertyShape::<DefaultTerm>::new(
        DefaultTerm::iri(iri("PersonShape-name")),
        ShaclPath::Predicate(iri("name")),
    )
    .with_deactivated(
        NodeExpression::try_new(ExpressionForm::Exists(Box::new(
            NodeExpression::try_new(ExpressionForm::PathValues {
                path: ShaclPath::Predicate(iri("retired")),
                path_node: None,
                focus_node: None,
            })
            .unwrap(),
        )))
        .unwrap(),
    );
    let constraint: SparqlConstraint = SparqlConstraint::new(
        DefaultTerm::iri(iri("constraint")),
        SparqlQuery::new(DefaultTerm::literal(Literal::string("SELECT $this {}"))),
    )
    .with_deactivated(boolean(false));
    let reifier = ConstraintDeactivation::<DefaultTerm>::new(
        DefaultTerm::iri(iri("PersonShape-name")),
        "http://www.w3.org/ns/shacl#minCount".parse().unwrap(),
        DefaultTerm::literal(Literal::string("1")),
        boolean(true),
    );
    golden::assert_golden(
        "core12_deactivation",
        &(shape, computed, constraint, reifier.clone()),
    );
    let serde_json::Value::Object(mut fields) = serde_json::to_value(&reifier).unwrap() else {
        panic!("a record serializes as an object")
    };
    fields.insert("reifier".to_owned(), serde_json::Value::Null);
    assert!(serde_json::from_value::<ConstraintDeactivation>(fields.into()).is_err());
}

#[test]
fn deactivation_expressions_decode_under_the_expression_budget() {
    let wide = NodeExpression::<DefaultTerm>::try_new(ExpressionForm::Concat(vec![
        Box::new(
            NodeExpression::focus_node()
        );
        4000
    ]))
    .unwrap();
    let shape = NodeShape::<DefaultTerm>::new(DefaultTerm::iri(iri("Wide"))).with_deactivated(wide);
    let mut json = serde_json::to_value(&shape).unwrap();
    golden::round_trip(&shape);
    let members = json["deactivated"]["form"]["Concat"]
        .as_array_mut()
        .unwrap();
    let copy = members.clone();
    for _ in 0..10 {
        members.extend(copy.iter().cloned());
    }
    let error = serde_json::from_value::<NodeShape>(json).unwrap_err();
    assert!(error.to_string().contains("budget"), "{error}");
}

//! Checked inference metadata and streamed declaration regressions.

use std::cmp::Ordering;

use iri_rfc3987::Iri;
use rdf_structural_model::{BlankNodeId, Literal, Subject, Term};
use serde::{Serialize, de::DeserializeOwned};
use shacl_structural_model::inference::{
    InferenceModelError, RuleNumber, RuleSchedule, RuleSet, RuleSetInclusion, RuleSetMembership,
    ShapeRuleLink,
};

fn iri(value: &str) -> Iri<String> {
    value.parse().unwrap()
}
fn number(value: &str, datatype: &str) -> Literal {
    Literal::typed(
        value,
        iri(&format!("http://www.w3.org/2001/XMLSchema#{datatype}")),
    )
    .unwrap()
}
fn round_trip<T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug>(value: &T) {
    assert_eq!(
        *value,
        serde_json::from_slice(&serde_json::to_vec(value).unwrap()).unwrap()
    );
    assert_eq!(
        *value,
        postcard::from_bytes(&postcard::to_allocvec(value).unwrap()).unwrap()
    );
}

#[test]
fn exact_order_retains_identity_separate_from_value_comparison() {
    let a = RuleNumber::try_from_literal(number("+0001.00", "decimal")).unwrap();
    let b = RuleNumber::try_from_literal(number("1", "integer")).unwrap();
    assert_ne!(a, b);
    assert_eq!(a.cmp_value(&b), Ordering::Equal);
    for (small, large) in [
        ("9007199254740992", "9007199254740993"),
        (
            "340282366920938463463374607431768211456",
            "340282366920938463463374607431768211457",
        ),
        ("-0.000000000000000000002", "-0.000000000000000000001"),
        ("-10", "-2"),
        ("-0", "0.1"),
        ("1.009", "1.01"),
    ] {
        let small = RuleNumber::try_from_literal(number(small, "decimal")).unwrap();
        let large = RuleNumber::try_from_literal(number(large, "decimal")).unwrap();
        assert_eq!(small.cmp_value(&large), Ordering::Less);
        assert_eq!(large.cmp_value(&small), Ordering::Greater);
        round_trip(&small);
    }
    round_trip(&a);
    round_trip(
        &RuleSchedule::new()
            .with_layer(a)
            .with_order(b)
            .with_run_once(false),
    );
    assert_ne!(
        RuleSchedule::<String>::new(),
        RuleSchedule::new().with_run_once(false)
    );
    assert_eq!(RuleSchedule::<String>::new().run_once(), None);
}

#[test]
fn numeric_checks_apply_to_construction_and_nested_decoding() {
    for (lexical, datatype) in [
        ("1.0", "integer"),
        ("1e2", "decimal"),
        ("NaN", "decimal"),
        ("", "integer"),
        ("+", "decimal"),
        (".", "decimal"),
        (" 1", "integer"),
        ("1", "double"),
        ("1", "string"),
    ] {
        let literal = number(lexical, datatype);
        assert!(matches!(
            RuleNumber::try_from_literal(literal.clone()),
            Err(InferenceModelError::InvalidRuleNumber)
        ));
        let bytes = postcard::to_allocvec(&literal).unwrap();
        assert!(postcard::from_bytes::<RuleNumber>(&bytes).is_err());
        let invalid = serde_json::to_value(literal).unwrap();
        let schedule = serde_json::json!({"layer": invalid, "order": null, "run_once": null});
        assert!(serde_json::from_value::<RuleSchedule>(schedule).is_err());
    }
    assert!(
        serde_json::from_str::<RuleSchedule>(
            r#"{"layer":null,"order":null,"run_once":null,"scheduler":"AF"}"#
        )
        .is_err()
    );
}

#[test]
fn references_are_independent_streamable_records_and_keep_blank_scope() {
    let blank =
        Subject::BlankNode(BlankNodeId::try_new(iri("urn:scope:a"), "s".to_owned()).unwrap());
    let rule = Subject::Iri(iri("urn:rule"));
    let link = ShapeRuleLink::new(blank.clone(), rule.clone());
    round_trip(&link);
    assert_eq!(link.shape(), &blank);
    let set = RuleSet::new(iri("urn:rules"));
    round_trip(&set);
    round_trip(&RuleSetMembership::new(iri("urn:rules"), rule));
    round_trip(&RuleSetInclusion::new(iri("urn:rules"), iri("urn:other")));
    // Inclusion edges retain identity, including cycles; this model does not resolve them.
    round_trip(&RuleSetInclusion::new(iri("urn:rules"), iri("urn:rules")));
    let shape_iri = iri("urn:shape");
    let rule_iri = iri("urn:rule");
    let borrowed = ShapeRuleLink::new(
        Subject::Iri(shape_iri.borrow()),
        Subject::Iri(rule_iri.borrow()),
    );
    assert_eq!(borrowed.shape(), &Subject::Iri(shape_iri.borrow()));
    let invalid = serde_json::json!({"shape": serde_json::to_value(Term::<String>::literal(Literal::string("invalid"))).unwrap(),
                                    "rule": serde_json::to_value(blank).unwrap()});
    assert!(serde_json::from_value::<ShapeRuleLink>(invalid).is_err());
}

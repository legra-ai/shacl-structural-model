//! Restricted draft expression checks and nested decoding budgets.
use iri_rfc3987::Iri;
use rdf_structural_model::{BlankNodeId, Literal, Subject, Term, Triple};
use serde::{Serialize, de::DeserializeOwned};
use shacl_structural_model::{
    ShaclPath,
    inference::{DecodeLimits, DirectExpression, InferenceModelError, TripleRuleDeclaration},
};

fn iri(text: &str) -> Iri<String> {
    text.parse().unwrap()
}
fn blank(scope: &str) -> BlankNodeId {
    BlankNodeId::try_new(iri(scope), "b".to_owned()).unwrap()
}
fn round_trip<T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug>(v: &T) {
    assert_eq!(
        *v,
        serde_json::from_slice(&serde_json::to_vec(v).unwrap()).unwrap()
    );
    assert_eq!(
        *v,
        postcard::from_bytes(&postcard::to_allocvec(v).unwrap()).unwrap()
    );
}
#[test]
fn direct_constants_preserve_exact_terms_and_exclude_bare_blanks() {
    for term in [
        Term::Iri(iri("urn:constant")),
        Term::literal(Literal::string("exact")),
        Term::triple(Triple::new(
            Subject::BlankNode(blank("urn:scope")),
            iri("urn:p"),
            Term::triple(Triple::new(
                Subject::Iri(iri("urn:s")),
                iri("urn:p"),
                Term::BlankNode(blank("urn:other")),
            )),
        )),
    ] {
        let expression = DirectExpression::try_constant(term.clone()).unwrap();
        assert_eq!(expression.constant(), Some(&term));
        round_trip(&expression);
        let declaration = TripleRuleDeclaration::new(Subject::Iri(iri("urn:rule")))
            .with_subject(expression.clone())
            .with_object(expression);
        round_trip(&declaration);
        assert!(declaration.predicate().is_none());
    }
    assert!(matches!(
        DirectExpression::try_constant(Term::BlankNode(blank("urn:scope"))),
        Err(InferenceModelError::BlankConstant)
    ));
    let invalid = serde_json::json!({"Constant": Term::<String>::BlankNode(blank("urn:scope"))});
    assert!(serde_json::from_value::<DirectExpression>(invalid).is_err());
}
#[test]
fn path_values_preserve_structure_and_reject_unsupported_focus() {
    let path = ShaclPath::Sequence(vec![
        ShaclPath::Predicate(iri("urn:p")),
        ShaclPath::Inverse(Box::new(ShaclPath::Predicate(iri("urn:q")))),
    ]);
    let expression = DirectExpression::try_path_values(path.clone(), None).unwrap();
    assert_eq!(expression.path(), Some(&path));
    round_trip(&expression);
    assert!(matches!(
        DirectExpression::try_path_values(path.clone(), Some(&Term::Iri(iri("urn:focus")))),
        Err(InferenceModelError::UnsupportedFocusExpression)
    ));
    let invalid = serde_json::json!({"PathValues": {"path":path,"focus_node":Term::<String>::Iri(iri("urn:focus"))}});
    assert!(serde_json::from_value::<DirectExpression>(invalid).is_err());
    for bad in [
        ShaclPath::<Iri<String>>::Sequence(vec![]),
        ShaclPath::Alternative(vec![ShaclPath::Predicate(iri("urn:p"))]),
    ] {
        assert!(matches!(
            DirectExpression::try_path_values(bad, None),
            Err(InferenceModelError::InvalidPathArity)
        ));
    }
    assert!(serde_json::from_str::<DirectExpression>(r#"{"This":null}"#).is_err());
    assert!(
        serde_json::from_str::<DirectExpression>(
            r#"{"PathValues":{"path":{"Predicate":"urn:p"},"nodes":null}}"#
        )
        .is_err()
    );
}
#[test]
fn explicit_decode_budgets_fail_before_loading_nested_data() {
    let expression =
        DirectExpression::try_constant(Term::literal(Literal::string("x".repeat(1000)))).unwrap();
    let json = serde_json::to_vec(&expression).unwrap();
    let mut deserializer = serde_json::Deserializer::from_slice(&json);
    assert!(
        DirectExpression::<String>::deserialize_with_limits(
            &mut deserializer,
            DecodeLimits::new(16, 64, 64)
        )
        .is_err()
    );
    let binary = postcard::to_allocvec(&expression).unwrap();
    let mut deserializer = postcard::Deserializer::from_bytes(&binary);
    assert!(
        DirectExpression::<String>::deserialize_with_limits(
            &mut deserializer,
            DecodeLimits::new(16, 64, 64)
        )
        .is_err()
    );
    let valid =
        DirectExpression::try_path_values(ShaclPath::Predicate(iri("urn:p")), None).unwrap();
    let json = serde_json::to_vec(&valid).unwrap();
    let mut deserializer = serde_json::Deserializer::from_slice(&json);
    assert!(
        DirectExpression::<String>::deserialize_with_limits(
            &mut deserializer,
            DecodeLimits::new(1, 1, 1)
        )
        .is_err()
    );
    let mut path = ShaclPath::Predicate(iri("urn:p"));
    for _ in 0..70 {
        path = ShaclPath::Inverse(Box::new(path));
    }
    assert!(matches!(
        DirectExpression::try_path_values(path, None),
        Err(InferenceModelError::ExpressionBudgetExceeded)
    ));
}

//! Inference triple rules hold full SHACL 1.2 node expressions under decode budgets.
use iri_rfc3987::Iri;
use rdf_structural_model::{BlankNodeId, Literal, Subject, Term, Triple};
use serde::{Serialize, de::DeserializeOwned};
use shacl_structural_model::{
    DecodeLimits, ExpressionError, ExpressionForm, ShaclPath,
    inference::{RuleExpression, TripleRuleDeclaration},
};

/// The wire shape of a path-values expression, bypassing constructor checks.
#[derive(Serialize)]
struct UncheckedExpression {
    source: Option<Term>,
    form: UncheckedForm,
}

#[derive(Serialize)]
enum UncheckedForm {
    #[allow(dead_code)]
    Constant(Term),
    #[allow(dead_code)]
    Empty,
    #[allow(dead_code)]
    Var(String),
    #[allow(dead_code)]
    List,
    PathValues {
        path: ShaclPath,
        path_node: Option<Term>,
        focus_node: Option<Box<UncheckedExpression>>,
    },
}

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
fn path_values(
    path: ShaclPath,
    focus: Option<RuleExpression>,
) -> Result<RuleExpression, ExpressionError> {
    RuleExpression::try_new(ExpressionForm::PathValues {
        path,
        path_node: None,
        focus_node: focus.map(Box::new),
    })
}

#[test]
fn constants_preserve_exact_terms_in_rule_slots() {
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
        let expression = RuleExpression::constant(term.clone());
        assert_eq!(expression.as_constant(), Some(&term));
        round_trip(&expression);
        let declaration = TripleRuleDeclaration::new(Subject::Iri(iri("urn:rule")))
            .with_subject(expression.clone())
            .with_object(expression);
        round_trip(&declaration);
        assert!(declaration.predicate().is_none());
    }
}

#[test]
fn path_values_accept_explicit_focus_and_reject_bad_arity() {
    let path = ShaclPath::Sequence(vec![
        ShaclPath::Predicate(iri("urn:p")),
        ShaclPath::Inverse(Box::new(ShaclPath::Predicate(iri("urn:q")))),
    ]);
    let focus = RuleExpression::constant(Term::Iri(iri("urn:focus")));
    let expression = path_values(path.clone(), Some(focus)).unwrap();
    round_trip(&expression);
    round_trip(
        &TripleRuleDeclaration::new(Subject::BlankNode(blank("urn:rule")))
            .with_predicate(expression),
    );
    for bad in [
        ShaclPath::<Iri<String>>::Sequence(vec![]),
        ShaclPath::Alternative(vec![ShaclPath::Predicate(iri("urn:p"))]),
    ] {
        assert_eq!(
            path_values(bad.clone(), None),
            Err(ExpressionError::InvalidPathArity)
        );
        let wire = UncheckedExpression {
            source: None,
            form: UncheckedForm::PathValues {
                path: bad,
                path_node: None,
                focus_node: None,
            },
        };
        let json = serde_json::to_string(&wire).unwrap();
        assert!(serde_json::from_str::<RuleExpression>(&json).is_err());
        let bytes = postcard::to_allocvec(&wire).unwrap();
        assert!(postcard::from_bytes::<RuleExpression>(&bytes).is_err());
    }
    assert!(serde_json::from_str::<RuleExpression>(r#"{"source":null,"form":"This"}"#).is_err());
}

#[test]
fn nested_bad_arity_is_rejected_below_a_valid_root() {
    let wire = UncheckedExpression {
        source: None,
        form: UncheckedForm::PathValues {
            path: ShaclPath::Predicate(iri("urn:p")),
            path_node: None,
            focus_node: Some(Box::new(UncheckedExpression {
                source: None,
                form: UncheckedForm::PathValues {
                    path: ShaclPath::Sequence(vec![ShaclPath::Predicate(iri("urn:p"))]),
                    path_node: None,
                    focus_node: None,
                },
            })),
        },
    };
    let bytes = postcard::to_allocvec(&wire).unwrap();
    assert!(postcard::from_bytes::<RuleExpression>(&bytes).is_err());
}

#[test]
fn explicit_decode_budgets_fail_before_loading_nested_data() {
    let expression = RuleExpression::constant(Term::literal(Literal::string("x".repeat(1000))));
    let json = serde_json::to_vec(&expression).unwrap();
    let mut deserializer = serde_json::Deserializer::from_slice(&json);
    assert!(
        RuleExpression::<String>::deserialize_with_limits(
            &mut deserializer,
            DecodeLimits::new(16, 64, 64)
        )
        .is_err()
    );
    let binary = postcard::to_allocvec(&expression).unwrap();
    let mut deserializer = postcard::Deserializer::from_bytes(&binary);
    assert!(
        RuleExpression::<String>::deserialize_with_limits(
            &mut deserializer,
            DecodeLimits::new(16, 64, 64)
        )
        .is_err()
    );
    let mut path = ShaclPath::Predicate(iri("urn:p"));
    for _ in 0..70 {
        path = ShaclPath::Inverse(Box::new(path));
    }
    assert_eq!(
        path_values(path, None),
        Err(ExpressionError::BudgetExceeded)
    );
}

#[test]
fn borrowed_constant_decodes_without_text_materialization() {
    let expression = RuleExpression::constant(Term::Iri(iri("urn:borrowed")));
    let json = serde_json::to_string(&expression).unwrap();
    let borrowed: RuleExpression<&str> = serde_json::from_str(&json).unwrap();
    assert_eq!(
        borrowed.as_constant(),
        Some(&Term::Iri(iri("urn:borrowed").borrow()))
    );
    let bytes = postcard::to_allocvec(&expression).unwrap();
    let borrowed: RuleExpression<&str> = postcard::from_bytes(&bytes).unwrap();
    assert_eq!(
        borrowed.as_constant(),
        Some(&Term::Iri(iri("urn:borrowed").borrow()))
    );
}

#[test]
fn individual_wire_budgets_stop_deep_or_wide_paths() {
    let mut path = ShaclPath::Predicate(iri("urn:p"));
    for _ in 0..20 {
        path = ShaclPath::Inverse(Box::new(path));
    }
    let expression = path_values(path, None).unwrap();
    let json = serde_json::to_vec(&expression).unwrap();
    let mut decoder = serde_json::Deserializer::from_slice(&json);
    let error = RuleExpression::<String>::deserialize_with_limits(
        &mut decoder,
        DecodeLimits::new(5, 10000, 10000),
    )
    .unwrap_err();
    assert!(error.to_string().contains("decode budget"));
    let wide = path_values(
        ShaclPath::Sequence(vec![ShaclPath::Predicate(iri("urn:p")); 20]),
        None,
    )
    .unwrap();
    let bytes = postcard::to_allocvec(&wide).unwrap();
    let mut decoder = postcard::Deserializer::from_bytes(&bytes);
    assert!(
        RuleExpression::<String>::deserialize_with_limits(
            &mut decoder,
            DecodeLimits::new(128, 10, 10000)
        )
        .is_err()
    );
    let path = ShaclPath::Sequence(vec![ShaclPath::Predicate(iri("urn:p")); 4096]);
    assert_eq!(
        path_values(path, None),
        Err(ExpressionError::BudgetExceeded)
    );
}

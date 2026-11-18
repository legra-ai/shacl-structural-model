//! SHACL 1.2 Node Expressions, Working Draft 16 November 2026: every
//! node-expression function, its function name and output contract, golden
//! vectors, unknown-field rejection and structural and decode budgets.

#[path = "support/forms.rs"]
mod forms;
#[path = "support/golden.rs"]
mod golden;

use shacl_structural_model::{
    ArgumentKey, ConstantList, DecodeLimits, DefaultTerm, ExpressionError, FunctionName,
    NodeExpression, OutputContract, ShaclPath, SparqlFunction, VariableName, vocabulary,
};

use forms::{Form, every_form, iri, literal, var};

#[test]
fn every_draft_function_has_its_name_contract_and_encoding() {
    let forms = every_form();
    for (expression, name, contract) in &forms {
        match expression.function_name() {
            FunctionName::BuiltIn(actual) => assert_eq!(actual, *name),
            FunctionName::NamedParameter => {
                assert_eq!(*name, vocabulary::NAMED_PARAMETER_EXPRESSION);
            }
            other => panic!("unexpected function name {other:?}"),
        }
        assert_eq!(expression.output_contract(), *contract, "{name}");
        golden::round_trip(expression);
    }
    let expressions: Vec<_> = forms
        .into_iter()
        .map(|(expression, ..)| expression)
        .collect();
    golden::assert_golden("core12_node_expressions", &expressions);
}

#[test]
fn constants_and_custom_list_functions_name_themselves() {
    let constant = NodeExpression::constant(literal("42"));
    assert_eq!(constant.function_name(), FunctionName::Constant);
    assert_eq!(constant.output_contract(), OutputContract::ExactlyOne);
    let custom = NodeExpression::try_new(Form::CustomList {
        function: iri("spacedConcat"),
        arguments: vec![constant.clone().into(), var("x")],
    })
    .unwrap();
    assert_eq!(
        custom.function_name(),
        FunctionName::ListParameter(&iri("spacedConcat"))
    );
    assert_eq!(custom.display(), "spacedConcat");
    let parameter_arg = NodeExpression::try_new(Form::Arg(ArgumentKey::Parameter(iri("param"))));
    assert_eq!(
        parameter_arg.unwrap().output_contract(),
        OutputContract::ArgumentExpression
    );
}

#[test]
fn contracts_report_order_and_distinctness() {
    assert_eq!(OutputContract::Set.is_ordered(), Some(false));
    assert_eq!(OutputContract::Set.is_distinct(), Some(true));
    assert_eq!(OutputContract::OrderedSequence.is_distinct(), Some(false));
    assert_eq!(OutputContract::DistinctSequence.is_ordered(), Some(true));
    assert_eq!(OutputContract::SelectedBranch.is_ordered(), None);
}

#[test]
fn sparql_function_table_matches_the_draft() {
    assert_eq!(SparqlFunction::ALL.len(), 77);
    for function in SparqlFunction::ALL {
        assert_eq!(SparqlFunction::from_iri(function.as_iri()), Some(*function));
    }
    assert_eq!(
        SparqlFunction::EncodeForUri.as_iri(),
        "http://www.w3.org/ns/sparql#encode-for-uri"
    );
    assert_eq!(
        SparqlFunction::from_iri("http://www.w3.org/ns/sparql#nope"),
        None
    );
    let unbound: Vec<_> = SparqlFunction::ALL
        .iter()
        .filter(|function| function.handles_unbound())
        .map(|function| function.local_name())
        .collect();
    assert_eq!(
        unbound,
        ["bound", "coalesce", "if", "logical-and", "logical-or"]
    );
}

#[test]
fn checked_operands_reject_draft_violations() {
    assert!(VariableName::focus_node().is_focus_node());
    assert_eq!(
        VariableName::try_new(""),
        Err(ExpressionError::EmptyVariableName)
    );
    assert_eq!(
        NodeExpression::try_new(Form::CustomNamed(Vec::new())),
        Err(ExpressionError::NoNamedArgument)
    );
    assert_eq!(
        NodeExpression::try_new(Form::PathValues {
            path: ShaclPath::Alternative(vec![ShaclPath::Predicate(iri("p"))]),
            path_node: None,
            focus_node: None,
        }),
        Err(ExpressionError::InvalidPathArity)
    );
    for encoded in [
        r#"{"source":null,"form":{"Var":""}}"#,
        r#"{"source":null,"form":{"CustomNamed":[]}}"#,
        r#"{"source":null,"form":{"Distinct":{"source":null,"form":{"Var":""}}}}"#,
        r#"{"source":null,"form":"Empty","depth":1}"#,
        r#"{"source":null,"form":{"Exists":{"source":null,"form":"Empty","extra":1}}}"#,
        r#"{"source":null,"form":{"If":{"condition":{"source":null,"form":"Empty"},"branches":{"Then":{"source":null,"form":"Empty"}},"else":null}}}"#,
    ] {
        assert!(
            serde_json::from_str::<NodeExpression>(encoded).is_err(),
            "{encoded}"
        );
    }
}

fn nested(depth: usize) -> NodeExpression {
    let mut expression = NodeExpression::focus_node();
    for _ in 1..depth {
        expression = NodeExpression::try_new(Form::Distinct(Box::new(expression))).unwrap();
    }
    expression
}

#[test]
fn structural_budgets_cover_nested_and_wide_forms() {
    let deepest = nested(NodeExpression::<DefaultTerm>::MAX_DEPTH);
    assert_eq!(deepest.depth(), 64);
    assert_eq!(
        NodeExpression::try_new(Form::Exists(Box::new(deepest))),
        Err(ExpressionError::BudgetExceeded)
    );
    let wide =
        vec![Box::new(NodeExpression::focus_node()); NodeExpression::<DefaultTerm>::MAX_NODES];
    assert_eq!(
        NodeExpression::try_new(Form::Concat(wide.clone())),
        Err(ExpressionError::BudgetExceeded)
    );
    let fits = NodeExpression::try_new(Form::Concat(wide[1..].to_vec())).unwrap();
    assert_eq!(fits.node_count(), 4096);
    let long = ConstantList::new(literal("x"), vec![literal("y"); 4096]);
    assert_eq!(
        NodeExpression::try_new(Form::List(long)),
        Err(ExpressionError::BudgetExceeded)
    );
}

#[test]
fn decode_budgets_refuse_deep_wide_and_long_inputs() {
    let deep = nested(30);
    let json = serde_json::to_vec(&deep).unwrap();
    let mut decoder = serde_json::Deserializer::from_slice(&json);
    let error = NodeExpression::<DefaultTerm>::deserialize_with_limits(
        &mut decoder,
        DecodeLimits::new(32, 100_000, 100_000),
    )
    .unwrap_err();
    assert!(error.to_string().contains("decode budget"));
    let wide = NodeExpression::try_new(Form::Concat(vec![var("x"); 64])).unwrap();
    let bytes = postcard::to_allocvec(&wide).unwrap();
    let mut decoder = postcard::Deserializer::from_bytes(&bytes);
    assert!(
        NodeExpression::<DefaultTerm>::deserialize_with_limits(
            &mut decoder,
            DecodeLimits::new(128, 100, 100_000)
        )
        .is_err()
    );
    let long = NodeExpression::try_new(Form::Var(VariableName::try_new("v".repeat(512)).unwrap()));
    let json = serde_json::to_vec(&long.unwrap()).unwrap();
    let mut decoder = serde_json::Deserializer::from_slice(&json);
    assert!(
        NodeExpression::<DefaultTerm>::deserialize_with_limits(
            &mut decoder,
            DecodeLimits::new(128, 100, 256)
        )
        .is_err()
    );
    let mut decoder = serde_json::Deserializer::from_slice(&json);
    assert!(
        NodeExpression::<DefaultTerm>::deserialize_with_limits(
            &mut decoder,
            DecodeLimits::default()
        )
        .is_ok()
    );
}

//! Node-expression targets, values and shape references from SHACL 1.2 Core,
//! Working Draft 16 November 2026 (sections 3.1.3.1, 3.1.3.3, 3.1.3.6,
//! 3.1.3.7 and 6.9.4.2), and Node Expressions, Working Draft 16 November 2026
//! (sections 6 and 7.2).

#[path = "support/golden.rs"]
mod golden;

use iri_rfc3987::Iri;
use rdf_structural_model::{BlankNodeId, Literal};
use shacl_structural_model::{
    Aggregate, ArgumentKey, ClassDeclaration, Component, DefaultTerm, ExplicitShapeTarget,
    ExpressionForm, ExpressionFunction, NodeExpression, NodeShape, Parameter, ParameterStyle,
    PropertyShape, ShaclPath, SparqlExpressionText, SparqlFunction, Target, VariableName,
    vocabulary,
};

fn iri(local: &str) -> Iri<String> {
    format!("https://example.org/{local}").parse().unwrap()
}

fn blank(label: &str) -> DefaultTerm {
    DefaultTerm::BlankNode(
        BlankNodeId::try_new("urn:shapes".parse().unwrap(), label.to_owned()).unwrap(),
    )
}

fn literal(text: &str) -> DefaultTerm {
    DefaultTerm::literal(Literal::string(text))
}

fn checked(form: ExpressionForm, source: &str) -> NodeExpression {
    NodeExpression::try_new(form)
        .unwrap()
        .with_source(blank(source))
}

fn path_values(local: &str, source: &str) -> Box<NodeExpression> {
    Box::new(checked(
        ExpressionForm::PathValues {
            path: ShaclPath::Predicate(iri(local)),
            path_node: None,
            focus_node: None,
        },
        source,
    ))
}

/// The display-name derivation of Node Expressions example 4.
fn display_name() -> NodeExpression {
    let concat = checked(
        ExpressionForm::Sparql {
            function: SparqlFunction::Concat,
            arguments: vec![
                path_values("firstName", "first"),
                Box::new(NodeExpression::constant(literal(" "))),
                path_values("lastName", "last"),
            ],
        },
        "concat",
    );
    checked(
        ExpressionForm::Sparql {
            function: SparqlFunction::Coalesce,
            arguments: vec![path_values("fullName", "full"), Box::new(concat)],
        },
        "coalesce",
    )
}

#[test]
fn property_shapes_derive_values_and_default_values_from_expressions() {
    let shape = PropertyShape::<DefaultTerm>::new(
        DefaultTerm::iri(iri("Person-displayName")),
        ShaclPath::Predicate(iri("displayName")),
    )
    .with_values(display_name());
    assert_eq!(
        shape.values.as_ref().map(NodeExpression::node_count),
        Some(9)
    );
    let loves = PropertyShape::<DefaultTerm>::new(
        DefaultTerm::iri(iri("Person-loves")),
        ShaclPath::Predicate(iri("loves")),
    )
    .with_default_value(NodeExpression::focus_node());
    assert!(matches!(
        loves.default_value.as_ref().map(NodeExpression::form),
        Some(ExpressionForm::Var(name)) if name.as_str() == VariableName::FOCUS_NODE
    ));
    golden::assert_golden("core12_values", &[shape, loves]);
}

#[test]
fn targets_cover_expressions_where_shapes_and_shape_classes() {
    let instances = checked(
        ExpressionForm::InstancesOf(Box::new(NodeExpression::constant(DefaultTerm::iri(iri(
            "Concept",
        ))))),
        "instances",
    );
    let shape = NodeShape::<DefaultTerm>::new(DefaultTerm::iri(iri("Person")))
        .with_target(Target::Node(NodeExpression::constant(DefaultTerm::iri(
            iri("Alice"),
        ))))
        .with_target(Target::Node(instances))
        .with_target(Target::Where(blank("adult")))
        .with_target(Target::ImplicitClass {
            class: DefaultTerm::iri(iri("Person")),
            declaration: ClassDeclaration::ShapeClass,
        });
    let kinds: Vec<_> = shape.targets.iter().map(Target::kind_name).collect();
    assert_eq!(
        kinds,
        ["targetNode", "targetNode", "targetWhere", "implicitClass"]
    );
    assert!(shape.targets.iter().all(|target| target.iri().is_none()));
    assert_eq!(
        shape.targets[1].display(),
        format!("{:?}", blank("instances"))
    );
    golden::assert_golden("core12_targets", &shape);
    let assertion = ExplicitShapeTarget::<DefaultTerm> {
        node: DefaultTerm::iri(iri("Alice")),
        shape: iri("PersonShape"),
    };
    golden::round_trip(&assertion);
    assert!(
        serde_json::from_str::<ExplicitShapeTarget>(
            r#"{"node":{"Iri":"urn:a"},"shape":"urn:s","graph":null}"#
        )
        .is_err()
    );
}

#[test]
fn node_by_expression_is_a_component_over_an_expression() {
    let path = ShaclPath::Sequence(vec![
        ShaclPath::Predicate(iri("dataSet")),
        ShaclPath::Predicate(iri("structure")),
        ShaclPath::Predicate(iri("hasShape")),
    ]);
    let expression = checked(
        ExpressionForm::PathValues {
            path,
            path_node: None,
            focus_node: None,
        },
        "shapeLookup",
    );
    let component: Component = Component::NodeByExpression(Box::new(expression));
    assert_eq!(component.kind_name(), "nodeByExpression");
    assert_eq!(
        component.display_value(),
        format!("{:?}", blank("shapeLookup"))
    );
    golden::assert_golden("core12_node_by_expression", &component);
}

#[test]
fn custom_functions_declare_bodies_and_key_parameters() {
    let average = checked(
        ExpressionForm::Sparql {
            function: SparqlFunction::Divide,
            arguments: vec![
                Box::new(checked(
                    ExpressionForm::Aggregate {
                        aggregate: Aggregate::Sum,
                        input: Box::new(checked(
                            ExpressionForm::Arg(ArgumentKey::Parameter(iri("average"))),
                            "arg",
                        )),
                    },
                    "sum",
                )),
                Box::new(NodeExpression::constant(literal("2"))),
            ],
        },
        "body",
    );
    let named = ExpressionFunction::<DefaultTerm>::new(
        iri("AverageExpression"),
        ParameterStyle::Named,
        average,
    )
    .with_parameter(Parameter::new(blank("param"), iri("average")).with_key_parameter(true))
    .with_parameter(Parameter::new(blank("other"), iri("precision")));
    let keys: Vec<_> = named
        .key_parameters()
        .map(|parameter| &parameter.path)
        .collect();
    assert_eq!(keys, [&iri("average")]);
    assert_eq!(
        ParameterStyle::Named.class_iri(),
        vocabulary::NAMED_PARAMETER_EXPRESSION_FUNCTION
    );
    let list = ExpressionFunction::<DefaultTerm>::new(
        iri("spacedConcat"),
        ParameterStyle::List,
        checked(
            ExpressionForm::SparqlExpr(SparqlExpressionText::new(literal(
                "CONCAT($arg0, ' ', $arg1)",
            ))),
            "sparqlExpr",
        ),
    );
    assert_eq!(
        ParameterStyle::List.expression_class_iri(),
        vocabulary::LIST_PARAMETER_EXPRESSION
    );
    golden::assert_golden("core12_expression_functions", &[named, list]);
}

//! Portable declarations for SPARQL and SHACL Advanced Features consumers.

use iri_rfc3987::Iri;
use shacl_structural_model::{
    Component, CustomConstraint, CustomConstraintComponent, CustomTarget, ExpressionConstraint,
    ExpressionKind, NodeExpression, NodeShape, Parameter, ParameterValues, PrefixDeclaration,
    PrefixDeclarations, ResultAnnotation, RuleKind, ShaclFunction, ShaclPath, ShaclRule,
    SparqlConstraint, SparqlQuery, SparqlSelectOrAsk, SparqlTarget, SparqlTargetType,
    SparqlValidator, Target,
};

fn iri(value: &str) -> Iri<String> {
    value.parse().unwrap()
}

fn query(text: &str) -> SparqlQuery<String> {
    SparqlQuery::new(text.to_owned()).with_prefixes("_:prefixes".to_owned())
}

fn expression(id: &str, kind: ExpressionKind<String>) -> NodeExpression<String> {
    NodeExpression::new(id.to_owned(), kind)
}

fn round_trip<T>(value: &T)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let encoded = serde_json::to_string(value).unwrap();
    assert_eq!(&serde_json::from_str::<T>(&encoded).unwrap(), value);
}

#[test]
fn sparql_constraints_keep_source_identity_messages_prefixes_and_annotations() {
    let mut constraint = SparqlConstraint::new(
        "_:constraint".to_owned(),
        query("SELECT $this ?value WHERE { $this ex:p ?value }"),
    );
    constraint.deactivated = true;
    constraint.messages = vec!["\"Required\"@en".to_owned(), "\"Verplicht\"@nl".to_owned()];
    constraint.annotations.push(ResultAnnotation {
        id: "_:annotation".to_owned(),
        property: iri("urn:detail"),
        variable: Some("value".to_owned()),
        values: vec!["\"default\"@en".to_owned()],
    });
    let shape = NodeShape::<String>::new("_:shape".to_owned())
        .with_component(Component::Sparql(Box::new(constraint)));
    round_trip(&shape);
    let Component::Sparql(constraint) = &shape.components[0] else {
        panic!("SPARQL declaration lost")
    };
    assert_eq!(constraint.id, "_:constraint");
    assert_eq!(constraint.select.prefixes, ["_:prefixes"]);

    let declarations = PrefixDeclarations {
        id: "_:prefixes".to_owned(),
        imports: vec!["urn:shared-prefixes".to_owned()],
        declarations: vec![PrefixDeclaration {
            id: "_:prefix".to_owned(),
            prefix: "ex".to_owned(),
            namespace: "\"urn:example:\"^^<http://www.w3.org/2001/XMLSchema#anyURI>".to_owned(),
        }],
    };
    round_trip(&declarations);
}

#[test]
fn custom_components_keep_all_parameter_values_and_validator_roles() {
    let mut parameter = Parameter::<String, i128>::new("_:param".to_owned(), iri("urn:limit"));
    parameter.optional = true;
    parameter.order = Some("+0002.000000000000000001".to_owned());
    parameter.names = vec!["\"Limit\"@en".to_owned()];
    parameter.descriptions = vec!["\"Maximum\"@en".to_owned()];
    parameter.components.push(Component::MaxCount(i128::MAX));
    let mut component = CustomConstraintComponent::new(iri("urn:LimitComponent"));
    component.parameters.push(parameter);
    component
        .label_templates
        .push("\"Limit {$limit}\"@en".to_owned());
    for (validators, select) in [
        (&mut component.validators, false),
        (&mut component.node_validators, true),
        (&mut component.property_validators, true),
    ] {
        let q = if select {
            SparqlSelectOrAsk::Select(query("SELECT $this WHERE {}"))
        } else {
            SparqlSelectOrAsk::Ask(query("ASK { FILTER ($value <= $limit) }"))
        };
        let mut validator = SparqlValidator::new(format!("_:validator{select}"), q);
        validator.messages.push("\"Over limit\"@en".to_owned());
        validators.push(validator);
    }
    round_trip(&component);
    let instance: Component<String> = Component::Custom(CustomConstraint {
        component: component.id,
        parameters: vec![ParameterValues {
            path: iri("urn:limit"),
            values: vec!["+0001".to_owned(), "18446744073709551616".to_owned()],
        }],
    });
    round_trip(&instance);
    assert_eq!(instance.kind_name(), "custom");
}

#[test]
fn direct_and_parameterized_targets_and_functions_retain_their_declarations() {
    let mut target = SparqlTarget::new(
        "_:target".to_owned(),
        query("SELECT ?this WHERE { ?this a ex:Person }"),
    );
    target.ask = Some(query("ASK { $this a ex:Person }"));
    let mut target_type = SparqlTargetType::<String>::new(iri("urn:People"), target.select.clone());
    target_type.ask = target.ask.clone();
    target_type
        .parameters
        .push(Parameter::new("_:country".to_owned(), iri("urn:country")));
    target_type
        .label_templates
        .push("\"People in {$country}\"@en".to_owned());
    round_trip(&target_type);
    let shape = NodeShape::<String>::new("_:shape".to_owned())
        .with_target(Target::Sparql(Box::new(target)))
        .with_target(Target::Custom(CustomTarget {
            id: "_:instance".to_owned(),
            target_type: target_type.id,
            parameters: vec![ParameterValues {
                path: iri("urn:country"),
                values: vec!["urn:NL".to_owned()],
            }],
        }));
    round_trip(&shape);
    for query in [
        SparqlSelectOrAsk::Ask(query("ASK { FILTER ($arg > 0) }")),
        SparqlSelectOrAsk::Select(query("SELECT ($arg * 2 AS ?result) WHERE {}")),
    ] {
        let mut function = ShaclFunction::<String>::new(iri("urn:function"), query);
        function
            .parameters
            .push(Parameter::new("_:argument".to_owned(), iri("urn:arg")));
        function.return_type = Some(iri(match &function.query {
            SparqlSelectOrAsk::Ask(_) => "http://www.w3.org/2001/XMLSchema#boolean",
            SparqlSelectOrAsk::Select(_) => "http://www.w3.org/2001/XMLSchema#integer",
        }));
        round_trip(&function);
    }
}

fn nested_expression() -> NodeExpression<String> {
    let focus = expression("http://www.w3.org/ns/shacl#this", ExpressionKind::This);
    let constant = expression("\"constant\"@en", ExpressionKind::Constant);
    let path = expression(
        "_:pathExpr",
        ExpressionKind::Path {
            path: ShaclPath::Inverse(Box::new(ShaclPath::Predicate(iri("urn:parent")))),
            path_node: Some("_:inversePath".to_owned()),
            nodes: Some(Box::new(focus.clone())),
        },
    );
    let union = expression("_:union", ExpressionKind::Union(vec![path, constant]));
    let intersection = expression(
        "_:intersection",
        ExpressionKind::Intersection(vec![union, focus]),
    );
    let filtered = expression(
        "_:filter",
        ExpressionKind::Filter {
            shape: "_:condition".to_owned(),
            nodes: Box::new(intersection),
        },
    );
    expression(
        "_:call",
        ExpressionKind::Function {
            name: iri("urn:compute"),
            args: vec![filtered],
        },
    )
}

#[test]
fn triple_rules_and_expression_constraints_preserve_nested_expressions_and_metadata() {
    let expression = nested_expression();
    round_trip(&expression);
    let mut rule = ShaclRule::new(
        "_:rule".to_owned(),
        RuleKind::Triple(Box::new(shacl_structural_model::TripleRule {
            subject: NodeExpression::new(
                "http://www.w3.org/ns/shacl#this".to_owned(),
                ExpressionKind::This,
            ),
            predicate: NodeExpression::new("urn:derived".to_owned(), ExpressionKind::Constant),
            object: expression.clone(),
        })),
    );
    rule.order = Some("+0009007199254740993.000000000000000001".to_owned());
    rule.conditions = vec!["_:condition".to_owned(), "urn:OtherCondition".to_owned()];
    rule.deactivated = true;
    // A resource may provide more than one supported rule type.
    rule.kinds.push(RuleKind::Sparql(query(
        "CONSTRUCT { $this ex:derived 1 } WHERE {}",
    )));
    let shape = NodeShape::<String>::new("_:shape".to_owned())
        .with_rule(rule)
        .with_component(Component::Expression(Box::new(ExpressionConstraint {
            expression,
            messages: vec!["\"Expected true\"@en".to_owned()],
        })));
    round_trip(&shape);
    assert_eq!(shape.rules[0].conditions[0], "_:condition");
    assert_eq!(shape.rules[0].kinds.len(), 2);
}

#[test]
fn callers_can_use_opaque_terms_without_clone_debug_or_serde() {
    struct Handle(u64);
    let shape = NodeShape::<Handle, i128>::new(Handle(1))
        .with_property_shape(Handle(2))
        .with_target(Target::Node(Handle(3)))
        .with_message(Handle(4))
        .with_component(Component::Node(Handle(2)))
        .with_component(Component::MinCount(i128::MAX));
    assert_eq!(shape.id.0, 1);
    assert_eq!(shape.messages[0].0, 4);
    let _: ShaclFunction<Handle> = ShaclFunction::new(
        iri("urn:function"),
        SparqlSelectOrAsk::Ask(SparqlQuery::new(Handle(5))),
    );
}

//! Caller-owned named roles preserve lexical identity independently of RDF values.

use serde::{Deserialize, Serialize};
use shacl_structural_model::{
    Component, CustomConstraint, CustomConstraintComponent, ExpressionConstraint, ExpressionKind,
    NodeExpression, NodeShape, Parameter, ParameterValues, PropertyShape, RuleKind, ShaclFunction,
    ShaclPath, ShaclRule, SparqlQuery, SparqlSelectOrAsk, SparqlTargetType, Target,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum LexicalName {
    Named(String),
}

impl AsRef<str> for LexicalName {
    fn as_ref(&self) -> &str {
        let Self::Named(value) = self;
        value
    }
}

fn named(value: &str) -> LexicalName {
    LexicalName::Named(value.to_owned())
}

type Constraint = Component<String, u64, u8, String, LexicalName>;
type Node = NodeShape<String, u64, u8, String, LexicalName>;
type Property = PropertyShape<String, u64, u8, String, LexicalName>;
type Declaration = Parameter<String, u64, u8, String, LexicalName>;
type Custom = CustomConstraintComponent<String, u64, u8, String, LexicalName>;
type Function = ShaclFunction<String, u64, u8, String, LexicalName>;
type TargetType = SparqlTargetType<String, u64, u8, String, LexicalName>;

fn roundtrip<T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug>(value: &T) {
    let json = serde_json::to_string(value).unwrap();
    assert_eq!(&serde_json::from_str::<T>(&json).unwrap(), value);
    let bytes = postcard::to_allocvec(value).unwrap();
    assert_eq!(&postcard::from_bytes::<T>(&bytes).unwrap(), value);
}

#[test]
fn lexical_named_roles_roundtrip_through_nested_shapes_rules_and_parameter_owners() {
    for lexical in [
        "",
        "relative",
        "relative/path",
        "not a standard IRI",
        "urn:absolute",
    ] {
        let expression = NodeExpression::<String, LexicalName>::new(
            "expression".to_owned(),
            ExpressionKind::Function {
                name: named(lexical),
                args: vec![NodeExpression::new(
                    "path-expression".to_owned(),
                    ExpressionKind::Path {
                        path: ShaclPath::Predicate(named(lexical)),
                        path_node: None,
                        nodes: None,
                    },
                )],
            },
        );
        let mut node = Node::new("shape".to_owned())
            .with_target(Target::Class(named(lexical)))
            .with_target(Target::SubjectsOf(named(lexical)))
            .with_target(Target::ObjectsOf(named(lexical)))
            .with_component(Constraint::Class(named(lexical)))
            .with_component(Constraint::Datatype(named(lexical)))
            .with_component(Constraint::Closed {
                is_closed: true,
                ignored_properties: vec![named(lexical)],
            })
            .with_component(Constraint::Expression(Box::new(ExpressionConstraint {
                expression: expression.clone(),
                messages: vec![],
            })));
        node.rules.push(ShaclRule::new(
            "rule".to_owned(),
            RuleKind::Triple(Box::new(shacl_structural_model::TripleRule {
                subject: expression.clone(),
                predicate: expression.clone(),
                object: expression,
            })),
        ));
        let property = Property::new("property".to_owned(), ShaclPath::Predicate(named(lexical)))
            .with_component(Constraint::Equals(ShaclPath::Predicate(named(lexical))));
        let mut parameter = Declaration::new("parameter".to_owned(), named(lexical));
        parameter
            .components
            .push(Constraint::Datatype(named(lexical)));
        let mut custom = Custom::new(named(lexical));
        custom.parameters.push(parameter.clone());
        node.components.push(Constraint::Custom(CustomConstraint {
            component: named(lexical),
            parameters: vec![ParameterValues {
                path: named(lexical),
                values: vec!["argument".to_owned()],
            }],
        }));
        let query = SparqlQuery::new("SELECT ?this WHERE {}".to_owned());
        let mut function = Function::new(named(lexical), SparqlSelectOrAsk::Select(query.clone()));
        function.return_type = Some(named(lexical));
        function.parameters.push(parameter.clone());
        let mut target_type = TargetType::new("target-type".to_owned(), query);
        target_type.parameters.push(parameter);
        assert_eq!(node.target_class_iris().next(), Some(&named(lexical)));
        roundtrip(&(node, property, custom, function, target_type));
    }
}

#[test]
fn named_role_deserialization_respects_default_and_caller_domains() {
    for invalid in [
        r#"{"Predicate":"relative"}"#,
        r#"{"Predicate":"not a standard IRI"}"#,
    ] {
        assert!(serde_json::from_str::<ShaclPath>(invalid).is_err());
    }
    assert!(
        serde_json::from_str::<ShaclPath<LexicalName>>(r#"{"Predicate":{"Literal":"relative"}}"#)
            .is_err()
    );
    let path: ShaclPath<LexicalName> =
        serde_json::from_str(r#"{"Predicate":{"Named":"relative"}}"#).unwrap();
    assert_eq!(path, ShaclPath::Predicate(named("relative")));
}

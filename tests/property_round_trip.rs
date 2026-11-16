//! Generated nested shapes, paths and expressions survive JSON and Postcard.
//!
//! Every generated value must decode to an equal value from both formats, and
//! a JSON object gaining an unknown field must be rejected, never discarded.

use iri_rfc3987::Iri;
use proptest::prelude::*;
use rdf_structural_model::{BlankNodeId, Literal};
use serde::Serialize;
use serde::de::DeserializeOwned;
use shacl_structural_model::{
    Component, DefaultTerm, ExpressionConstraint, ExpressionKind, LanguageTag, NodeExpression,
    NodeKind, NodeShape, PropertyShape, Severity, ShaclPath, Target,
};

fn iri() -> impl Strategy<Value = Iri<String>> {
    "[a-z]{1,6}".prop_map(|local| format!("urn:x:{local}").parse().unwrap())
}

fn term() -> impl Strategy<Value = DefaultTerm> {
    prop_oneof![
        iri().prop_map(DefaultTerm::iri),
        "[a-z][a-z0-9]{0,5}".prop_map(|label| DefaultTerm::BlankNode(
            BlankNodeId::try_new("urn:scope".parse().unwrap(), label).unwrap()
        )),
        ".{0,8}".prop_map(|text| DefaultTerm::literal(Literal::string(text))),
    ]
}

fn path() -> impl Strategy<Value = ShaclPath> {
    iri()
        .prop_map(ShaclPath::Predicate)
        .prop_recursive(4, 24, 3, |inner| {
            prop_oneof![
                prop::collection::vec(inner.clone(), 2..4).prop_map(ShaclPath::Sequence),
                prop::collection::vec(inner.clone(), 2..4).prop_map(ShaclPath::Alternative),
                inner
                    .clone()
                    .prop_map(|path| ShaclPath::Inverse(Box::new(path))),
                inner
                    .clone()
                    .prop_map(|path| ShaclPath::ZeroOrMore(Box::new(path))),
                inner
                    .clone()
                    .prop_map(|path| ShaclPath::OneOrMore(Box::new(path))),
                inner.prop_map(|path| ShaclPath::ZeroOrOne(Box::new(path))),
            ]
        })
}

fn expression() -> impl Strategy<Value = NodeExpression> {
    let leaf = prop_oneof![
        term().prop_map(|node| NodeExpression::new(node, ExpressionKind::This)),
        term().prop_map(|node| NodeExpression::new(node, ExpressionKind::Constant)),
    ];
    leaf.prop_recursive(4, 24, 3, |inner| {
        let nested = prop::collection::vec(inner.clone(), 0..3);
        prop_oneof![
            (
                term(),
                path(),
                prop::option::of(term()),
                prop::option::of(inner.clone())
            )
                .prop_map(|(node, path, path_node, nodes)| NodeExpression::new(
                    node,
                    ExpressionKind::Path {
                        path,
                        path_node,
                        nodes: nodes.map(Box::new),
                    }
                )),
            (term(), term(), inner.clone()).prop_map(|(node, shape, nodes)| NodeExpression::new(
                node,
                ExpressionKind::Filter {
                    shape,
                    nodes: Box::new(nodes),
                }
            )),
            (term(), iri(), nested.clone()).prop_map(|(node, name, args)| NodeExpression::new(
                node,
                ExpressionKind::Function { name, args }
            )),
            (term(), nested.clone())
                .prop_map(|(node, list)| NodeExpression::new(node, ExpressionKind::Union(list))),
            (term(), nested).prop_map(|(node, list)| NodeExpression::new(
                node,
                ExpressionKind::Intersection(list)
            )),
        ]
    })
}

fn language() -> impl Strategy<Value = LanguageTag> {
    prop::sample::select(vec!["en", "nl-BE", "zh-Hant-TW", "de-1996", "x-private"])
        .prop_map(|tag| LanguageTag::try_new(tag).unwrap())
}

fn component() -> impl Strategy<Value = Component> {
    prop_oneof![
        iri().prop_map(Component::Class),
        prop::sample::select(vec![
            NodeKind::Iri,
            NodeKind::Literal,
            NodeKind::BlankNodeOrIri
        ])
        .prop_map(Component::NodeKind),
        term().prop_map(Component::MinCount),
        (term(), prop::option::of(term()))
            .prop_map(|(pattern, flags)| Component::Pattern { pattern, flags }),
        prop::collection::vec(language(), 0..4).prop_map(Component::LanguageIn),
        path().prop_map(Component::Equals),
        prop::collection::vec(term(), 0..4).prop_map(Component::Or),
        term().prop_map(Component::Not),
        (term(), prop::option::of(term()), any::<bool>()).prop_map(|(shape, min, disjoint)| {
            Component::QualifiedValueShape {
                shape,
                min_count: min,
                max_count: None,
                disjoint,
            }
        }),
        (expression(), prop::collection::vec(term(), 0..2)).prop_map(|(expression, messages)| {
            Component::Expression(Box::new(ExpressionConstraint {
                expression,
                messages,
            }))
        }),
        any::<bool>().prop_map(Component::SingleLine),
    ]
}

fn severity() -> impl Strategy<Value = Severity> {
    prop_oneof![
        Just(Severity::Violation),
        Just(Severity::Warning),
        Just(Severity::Info),
        iri().prop_map(Severity::Custom),
    ]
}

fn node_shape() -> impl Strategy<Value = NodeShape> {
    (
        term(),
        prop::collection::vec(component(), 0..4),
        prop::collection::vec(term(), 0..3),
        severity(),
        any::<bool>(),
    )
        .prop_map(|(id, components, targets, severity, deactivated)| {
            let mut shape = NodeShape::new(id)
                .with_severity(severity)
                .with_deactivated(deactivated);
            shape.components = components;
            shape.targets = targets.into_iter().map(Target::Node).collect();
            shape
        })
}

fn property_shape() -> impl Strategy<Value = PropertyShape> {
    (
        term(),
        path(),
        prop::collection::vec(component(), 0..4),
        prop::option::of(term()),
    )
        .prop_map(|(id, path, components, path_node)| {
            let mut shape = PropertyShape::new(id, path);
            shape.components = components;
            shape.path_node = path_node;
            shape
        })
}

fn assert_round_trips<T>(value: &T) -> Result<(), TestCaseError>
where
    T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let json = serde_json::to_string(value).unwrap();
    prop_assert_eq!(&serde_json::from_str::<T>(&json).unwrap(), value);
    let binary = postcard::to_allocvec(value).unwrap();
    prop_assert_eq!(&postcard::from_bytes::<T>(&binary).unwrap(), value);
    Ok(())
}

fn assert_unknown_field_rejected<T>(value: &T) -> Result<(), TestCaseError>
where
    T: Serialize + DeserializeOwned,
{
    let serde_json::Value::Object(mut fields) = serde_json::to_value(value).unwrap() else {
        return Err(TestCaseError::fail(
            "a struct must serialize as a JSON object",
        ));
    };
    fields.insert("unknown".to_owned(), serde_json::Value::Bool(true));
    let encoded = serde_json::to_string(&fields).unwrap();
    prop_assert!(serde_json::from_str::<T>(&encoded).is_err());
    Ok(())
}

proptest! {
    #[test]
    fn paths_round_trip(path in path()) {
        assert_round_trips(&path)?;
    }

    #[test]
    fn expressions_round_trip_and_reject_unknown_fields(expression in expression()) {
        assert_round_trips(&expression)?;
        assert_unknown_field_rejected(&expression)?;
    }

    #[test]
    fn node_shapes_round_trip_and_reject_unknown_fields(shape in node_shape()) {
        assert_round_trips(&shape)?;
        assert_unknown_field_rejected(&shape)?;
    }

    #[test]
    fn property_shapes_round_trip_and_reject_unknown_fields(shape in property_shape()) {
        assert_round_trips(&shape)?;
        assert_unknown_field_rejected(&shape)?;
    }
}

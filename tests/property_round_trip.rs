//! Generated nested shapes, paths and expressions survive JSON and Postcard.
//!
//! Every generated value must decode to an equal value from both formats, and
//! a JSON object gaining an unknown field must be rejected, never discarded.

#[path = "support/strategies.rs"]
mod strategies;

use iri_rfc3987::Iri;
use proptest::prelude::*;
use serde::Serialize;
use serde::de::DeserializeOwned;
use shacl_structural_model::{
    Closedness, CodeIdentifier, Component, ExpressionConstraint, Intent, LanguageTag, NamedOrList,
    NodeExpression, NodeKind, NodeShape, PropertyShape, Severity, ShapeAnnotations, Target,
};
use strategies::{expression, iri, path, term};

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
            NodeKind::BlankNodeOrIri,
            NodeKind::TripleTerm
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
        expression().prop_map(|expression| Component::NodeByExpression(Box::new(expression))),
        any::<bool>().prop_map(Component::SingleLine),
        core12_component(),
    ]
}

fn named_or_list() -> impl Strategy<Value = NamedOrList<Iri<String>>> {
    prop_oneof![
        iri().prop_map(NamedOrList::One),
        prop::collection::vec(iri(), 0..4).prop_map(NamedOrList::List),
    ]
}

fn core12_component() -> impl Strategy<Value = Component> {
    prop_oneof![
        (
            prop::sample::select(vec![
                Closedness::Open,
                Closedness::Closed,
                Closedness::ByTypes
            ]),
            prop::collection::vec(iri(), 0..3)
        )
            .prop_map(|(closedness, ignored_properties)| Component::Closed {
                closedness,
                ignored_properties,
            }),
        path().prop_map(Component::SubsetOf),
        term().prop_map(Component::SomeValue),
        (term(), prop::option::of(any::<bool>())).prop_map(|(shape, reification_required)| {
            Component::ReifierShape {
                shape,
                reification_required,
            }
        }),
        named_or_list().prop_map(Component::RootClass),
        named_or_list().prop_map(Component::UniqueValuesFor),
    ]
}

fn severity() -> impl Strategy<Value = Severity> {
    prop_oneof![
        Just(Severity::Violation),
        Just(Severity::Warning),
        Just(Severity::Info),
        Just(Severity::Debug),
        Just(Severity::Trace),
        iri().prop_map(Severity::Custom),
    ]
}

fn annotations() -> impl Strategy<Value = ShapeAnnotations> {
    (
        prop::collection::vec((term(), prop::option::of(any::<bool>())), 0..3),
        prop::collection::vec(term(), 0..2),
        prop::option::of("[a-zA-Z_][a-zA-Z0-9_]{0,8}"),
        prop::option::of(iri()),
        prop::collection::vec(term(), 0..2),
    )
        .prop_map(
            |(intents, agent_instructions, code_identifier, code_type, units)| ShapeAnnotations {
                intents: intents
                    .into_iter()
                    .map(|(statement, formalized)| Intent {
                        statement,
                        formalized,
                    })
                    .collect(),
                agent_instructions,
                code_identifier: code_identifier.map(|name| CodeIdentifier::try_new(name).unwrap()),
                code_type,
                units,
            },
        )
}

fn node_shape() -> impl Strategy<Value = NodeShape> {
    (
        term(),
        prop::collection::vec(component(), 0..4),
        prop::collection::vec(term(), 0..3),
        severity(),
        prop::option::of(expression()),
        annotations(),
    )
        .prop_map(
            |(id, components, targets, severity, deactivated, annotations)| {
                let mut shape = NodeShape::new(id)
                    .with_severity(severity)
                    .with_annotations(annotations);
                shape.deactivated = deactivated;
                shape.components = components;
                shape.targets = targets
                    .iter()
                    .cloned()
                    .map(|target| Target::Node(NodeExpression::constant(target)))
                    .collect();
                shape.targets.extend(targets.into_iter().map(Target::Where));
                shape
            },
        )
}

fn property_shape() -> impl Strategy<Value = PropertyShape> {
    (
        term(),
        path(),
        prop::collection::vec(component(), 0..4),
        prop::option::of(term()),
        prop::option::of(expression()),
        prop::option::of(expression()),
        prop::collection::vec(expression(), 0..2),
        (
            prop::option::of(expression()),
            annotations(),
            prop::option::of(term()),
        ),
    )
        .prop_map(
            |(id, path, components, path_node, values, default_value, targets, extra)| {
                let (deactivated, annotations, group) = extra;
                let mut shape = PropertyShape::new(id, path).with_annotations(annotations);
                shape.deactivated = deactivated;
                shape.group = group;
                shape.components = components;
                shape.path_node = path_node;
                shape.values = values;
                shape.default_value = default_value;
                shape.targets = targets.into_iter().map(Target::Node).collect();
                shape
            },
        )
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

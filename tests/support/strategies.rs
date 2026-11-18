//! Bounded-depth proptest strategies shared by the generated round-trip tests.

use iri_rfc3987::Iri;
use proptest::prelude::*;
use rdf_structural_model::{BlankNodeId, Literal};
use shacl_structural_model::{
    Aggregate, ArgumentKey, ConstantList, DefaultTerm, ExpressionForm, IfBranches, NamedArgument,
    NodeExpression, ShaclPath, SparqlExpressionText, SparqlFunction, VariableName,
};

type Form = ExpressionForm;
type Child = Box<NodeExpression>;

pub fn iri() -> impl Strategy<Value = Iri<String>> {
    "[a-z]{1,6}".prop_map(|local| format!("urn:x:{local}").parse().unwrap())
}

pub fn term() -> BoxedStrategy<DefaultTerm> {
    prop_oneof![
        iri().prop_map(DefaultTerm::iri),
        "[a-z][a-z0-9]{0,5}".prop_map(|label| DefaultTerm::BlankNode(
            BlankNodeId::try_new("urn:scope".parse().unwrap(), label).unwrap()
        )),
        ".{0,8}".prop_map(|text| DefaultTerm::literal(Literal::string(text))),
    ]
    .boxed()
}

/// Paths of depth at most four, with valid sequence/alternative arity.
pub fn path() -> BoxedStrategy<ShaclPath> {
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
        .boxed()
}

fn checked(form: Form, source: Option<DefaultTerm>) -> NodeExpression {
    let expression = NodeExpression::try_new(form).unwrap();
    match source {
        Some(source) => expression.with_source(source),
        None => expression,
    }
}

fn text() -> impl Strategy<Value = SparqlExpressionText<DefaultTerm>> {
    (term(), prop::option::of(term()))
        .prop_map(|(text, prefixes)| SparqlExpressionText { text, prefixes })
}

fn leaf() -> BoxedStrategy<Form> {
    prop_oneof![
        term().prop_map(Form::Constant),
        Just(Form::Empty),
        "[a-zA-Z]{1,8}".prop_map(|name| Form::Var(VariableName::try_new(name).unwrap())),
        (term(), prop::collection::vec(term(), 0..3))
            .prop_map(|(first, rest)| Form::List(ConstantList::new(first, rest))),
        term().prop_map(Form::NodesMatching),
        text().prop_map(Form::Select),
        text().prop_map(Form::SparqlExpr),
        iri().prop_map(|key| Form::Arg(ArgumentKey::Parameter(key))),
        term().prop_map(|key| Form::Arg(ArgumentKey::Position(key))),
    ]
    .boxed()
}

fn branches(inner: BoxedStrategy<Child>) -> BoxedStrategy<IfBranches<Child>> {
    prop_oneof![
        inner.clone().prop_map(IfBranches::Then),
        inner.clone().prop_map(IfBranches::Else),
        (inner.clone(), inner).prop_map(|(then, otherwise)| IfBranches::Both { then, otherwise }),
    ]
    .boxed()
}

fn unary(inner: BoxedStrategy<Child>) -> BoxedStrategy<Form> {
    let aggregate = prop::sample::select(vec![
        Aggregate::Count,
        Aggregate::Min,
        Aggregate::Max,
        Aggregate::Sum,
    ]);
    prop_oneof![
        inner.clone().prop_map(Form::Exists),
        inner.clone().prop_map(Form::Distinct),
        inner.clone().prop_map(Form::InstancesOf),
        (aggregate, inner.clone())
            .prop_map(|(aggregate, input)| Form::Aggregate { aggregate, input }),
        (
            path(),
            prop::option::of(term()),
            prop::option::of(inner.clone())
        )
            .prop_map(|(path, path_node, focus_node)| Form::PathValues {
                path,
                path_node,
                focus_node,
            }),
        (inner.clone(), prop::option::of(inner.clone()))
            .prop_map(|(flat_map, nodes)| Form::FlatMap { flat_map, nodes }),
        (term(), prop::option::of(inner.clone()))
            .prop_map(|(shape, nodes)| Form::FindFirst { shape, nodes }),
        (term(), prop::option::of(inner))
            .prop_map(|(shape, nodes)| Form::MatchAll { shape, nodes }),
    ]
    .boxed()
}

fn binary(inner: BoxedStrategy<Child>) -> BoxedStrategy<Form> {
    prop_oneof![
        (inner.clone(), branches(inner.clone())).prop_map(|(condition, branches)| Form::If {
            condition,
            branches
        }),
        (inner.clone(), inner.clone()).prop_map(|(remove, nodes)| Form::Remove { remove, nodes }),
        (term(), inner.clone()).prop_map(|(shape, nodes)| Form::FilterShape { shape, nodes }),
        (term(), inner.clone()).prop_map(|(limit, nodes)| Form::Limit { limit, nodes }),
        (term(), inner.clone()).prop_map(|(offset, nodes)| Form::Offset { offset, nodes }),
        (
            inner.clone(),
            inner.clone(),
            prop::option::of(any::<bool>())
        )
            .prop_map(|(order_by, nodes, descending)| Form::OrderBy {
                order_by,
                nodes,
                descending,
            }),
        (inner.clone(), inner).prop_map(|(node, shape)| Form::ConformsToShape { node, shape }),
    ]
    .boxed()
}

fn variadic(inner: BoxedStrategy<Child>) -> BoxedStrategy<Form> {
    let list = prop::collection::vec(inner.clone(), 0..3);
    let function = prop::sample::select(SparqlFunction::ALL.to_vec());
    prop_oneof![
        list.clone().prop_map(Form::Intersection),
        list.clone().prop_map(Form::Concat),
        (function, list.clone()).prop_map(|(function, arguments)| Form::Sparql {
            function,
            arguments,
        }),
        (iri(), list).prop_map(|(function, arguments)| Form::CustomList {
            function,
            arguments,
        }),
        prop::collection::vec((iri(), inner), 1..3).prop_map(|arguments| Form::CustomNamed(
            arguments
                .into_iter()
                .map(|(parameter, value)| NamedArgument::new(parameter, value))
                .collect()
        )),
    ]
    .boxed()
}

/// Every SHACL 1.2 node-expression form, nested at most four levels deep.
pub fn expression() -> impl Strategy<Value = NodeExpression> {
    (leaf(), prop::option::of(term()))
        .prop_map(|(form, source)| checked(form, source))
        .prop_recursive(4, 32, 3, |inner| {
            let inner = inner.prop_map(Box::new).boxed();
            (
                prop_oneof![unary(inner.clone()), binary(inner.clone()), variadic(inner)],
                prop::option::of(term()),
            )
                .prop_map(|(form, source)| checked(form, source))
        })
}

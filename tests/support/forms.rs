//! One expression per SHACL 1.2 node-expression function, with its draft
//! function name and output contract.

#![allow(dead_code)]

use iri_rfc3987::Iri;
use rdf_structural_model::Literal;
use shacl_structural_model::OutputContract::{
    AtMostOne, DistinctSequence, ExactlyOne, FunctionBody, OrderedSequence, SelectedBranch, Set,
};
use shacl_structural_model::{
    Aggregate, ArgumentKey, ConstantList, DefaultTerm, ExpressionForm, IfBranches, NamedArgument,
    NodeExpression, OutputContract, ShaclPath, SparqlExpressionText, SparqlFunction, VariableName,
    vocabulary,
};

pub type Form = ExpressionForm;

pub fn iri(local: &str) -> Iri<String> {
    format!("https://example.org/{local}").parse().unwrap()
}

pub fn constant(local: &str) -> Box<NodeExpression> {
    Box::new(NodeExpression::constant(DefaultTerm::iri(iri(local))))
}

pub fn literal(text: &str) -> DefaultTerm {
    DefaultTerm::literal(Literal::string(text))
}

pub fn path_values(local: &str) -> Box<NodeExpression> {
    Box::new(
        NodeExpression::try_new(Form::PathValues {
            path: ShaclPath::Predicate(iri(local)),
            path_node: None,
            focus_node: None,
        })
        .unwrap(),
    )
}

pub fn var(name: &str) -> Box<NodeExpression> {
    Box::new(NodeExpression::try_new(Form::Var(VariableName::try_new(name).unwrap())).unwrap())
}

pub type Entry = (NodeExpression, &'static str, OutputContract);

/// Basic node expressions (4.1) and the first list operators (4.2).
fn basic_forms() -> Vec<Entry> {
    [
        (Form::Empty, vocabulary::EMPTY_EXPRESSION, AtMostOne),
        (
            Form::Var(VariableName::focus_node()),
            vocabulary::VAR_EXPRESSION,
            AtMostOne,
        ),
        (
            Form::List(ConstantList::new(literal("a"), vec![literal("b")])),
            vocabulary::LIST_EXPRESSION,
            OrderedSequence,
        ),
        (
            Form::PathValues {
                path: ShaclPath::ZeroOrMore(Box::new(ShaclPath::Predicate(iri("broader")))),
                path_node: None,
                focus_node: Some(constant("focus")),
            },
            vocabulary::PATH_VALUES_EXPRESSION,
            Set,
        ),
        (
            Form::Exists(path_values("capitalOf")),
            vocabulary::EXISTS_EXPRESSION,
            ExactlyOne,
        ),
        (
            Form::If {
                condition: path_values("flag"),
                branches: IfBranches::Both {
                    then: constant("blue"),
                    otherwise: constant("red"),
                },
            },
            vocabulary::IF_EXPRESSION,
            SelectedBranch,
        ),
        (
            Form::Distinct(path_values("p")),
            vocabulary::DISTINCT_EXPRESSION,
            DistinctSequence,
        ),
        (
            Form::Intersection(vec![path_values("p"), path_values("q")]),
            vocabulary::INTERSECTION_EXPRESSION,
            Set,
        ),
        (
            Form::Concat(vec![path_values("parent"), path_values("sibling")]),
            vocabulary::CONCAT_EXPRESSION,
            OrderedSequence,
        ),
    ]
    .into_iter()
    .map(|(form, name, contract)| (NodeExpression::try_new(form).unwrap(), name, contract))
    .collect()
}

/// The remaining list operators (4.2) and sequence operations (4.3).
fn sequence_forms() -> Vec<Entry> {
    [
        (
            Form::Remove {
                remove: path_values("onLeave"),
                nodes: path_values("author"),
            },
            vocabulary::REMOVE_EXPRESSION,
            OrderedSequence,
        ),
        (
            Form::FilterShape {
                shape: DefaultTerm::iri(iri("Male")),
                nodes: path_values("child"),
            },
            vocabulary::FILTER_SHAPE_EXPRESSION,
            OrderedSequence,
        ),
        (
            Form::Limit {
                limit: literal("2"),
                nodes: path_values("child"),
            },
            vocabulary::LIMIT_EXPRESSION,
            OrderedSequence,
        ),
        (
            Form::Offset {
                offset: literal("1"),
                nodes: path_values("child"),
            },
            vocabulary::OFFSET_EXPRESSION,
            OrderedSequence,
        ),
        (
            Form::OrderBy {
                order_by: path_values("dateOfBirth"),
                nodes: path_values("child"),
                descending: Some(false),
            },
            vocabulary::ORDER_BY_EXPRESSION,
            OrderedSequence,
        ),
        (
            Form::FlatMap {
                flat_map: path_values("skill"),
                nodes: Some(path_values("employee")),
            },
            vocabulary::FLAT_MAP_EXPRESSION,
            OrderedSequence,
        ),
        (
            Form::FindFirst {
                shape: DefaultTerm::iri(iri("Senior")),
                nodes: None,
            },
            vocabulary::FIND_FIRST_EXPRESSION,
            AtMostOne,
        ),
        (
            Form::MatchAll {
                shape: DefaultTerm::iri(iri("Active")),
                nodes: None,
            },
            vocabulary::MATCH_ALL_EXPRESSION,
            ExactlyOne,
        ),
    ]
    .into_iter()
    .map(|(form, name, contract)| (NodeExpression::try_new(form).unwrap(), name, contract))
    .collect()
}

/// Aggregations (4.4), miscellaneous (4.5), SPARQL (5) and custom forms (6).
fn other_forms() -> Vec<Entry> {
    let text = || SparqlExpressionText::new(literal("SELECT ?x WHERE {}"));
    [
        (
            Form::Aggregate {
                aggregate: Aggregate::Count,
                input: path_values("p"),
            },
            vocabulary::COUNT_EXPRESSION,
            ExactlyOne,
        ),
        (
            Form::Aggregate {
                aggregate: Aggregate::Min,
                input: path_values("p"),
            },
            vocabulary::MIN_EXPRESSION,
            AtMostOne,
        ),
        (
            Form::Aggregate {
                aggregate: Aggregate::Max,
                input: path_values("p"),
            },
            vocabulary::MAX_EXPRESSION,
            AtMostOne,
        ),
        (
            Form::Aggregate {
                aggregate: Aggregate::Sum,
                input: path_values("p"),
            },
            vocabulary::SUM_EXPRESSION,
            ExactlyOne,
        ),
        (
            Form::InstancesOf(constant("Person")),
            vocabulary::INSTANCES_OF_EXPRESSION,
            Set,
        ),
        (
            Form::NodesMatching(DefaultTerm::iri(iri("BigCompany"))),
            vocabulary::NODES_MATCHING_EXPRESSION,
            Set,
        ),
        (
            Form::ConformsToShape {
                node: var("focusNode"),
                shape: constant("CompanyShape"),
            },
            vocabulary::CONFORMS_TO_SHAPE,
            AtMostOne,
        ),
        (
            Form::Sparql {
                function: SparqlFunction::Coalesce,
                arguments: vec![path_values("fullName"), constant("fallback")],
            },
            SparqlFunction::Coalesce.as_iri(),
            AtMostOne,
        ),
        (
            Form::Select(text()),
            vocabulary::SELECT_EXPRESSION,
            OrderedSequence,
        ),
        (
            Form::SparqlExpr(
                SparqlExpressionText::new(literal("STRLEN(STR($this))"))
                    .with_prefixes(DefaultTerm::iri(iri("prefixes"))),
            ),
            vocabulary::SPARQL_EXPR_EXPRESSION,
            OrderedSequence,
        ),
        (
            Form::CustomNamed(vec![NamedArgument::new(iri("average"), path_values("p"))]),
            vocabulary::NAMED_PARAMETER_EXPRESSION,
            FunctionBody,
        ),
        (
            Form::Arg(ArgumentKey::Position(literal("0"))),
            vocabulary::ARG_EXPRESSION,
            AtMostOne,
        ),
    ]
    .into_iter()
    .map(|(form, name, contract)| (NodeExpression::try_new(form).unwrap(), name, contract))
    .collect()
}

/// Every form, in draft section order.
pub fn every_form() -> Vec<Entry> {
    let mut forms = basic_forms();
    forms.extend(sequence_forms());
    forms.extend(other_forms());
    forms
}

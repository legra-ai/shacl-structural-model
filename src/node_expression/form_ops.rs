//! Structural operations over [`ExpressionForm`] operands.

use super::{Aggregate, ArgumentKey, ExpressionForm, NamedArgument, OutputContract};
use crate::vocabulary as v;

/// The function name of an expression (Node Expressions section 3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FunctionName<'a, Named> {
    /// `sh:IRIExpression`, `sh:LiteralExpression` or `sh:TripleTermExpression`,
    /// decided by the consumer's term type.
    Constant,
    /// A built-in `shnex:`, `sh:` or `sparql:` function IRI.
    BuiltIn(&'static str),
    /// A custom list parameter function, named by its list parameter property.
    ListParameter(&'a Named),
    /// A custom named parameter function, resolved by the consumer from its
    /// key parameters.
    NamedParameter,
}

impl<Value, Named, Child> ExpressionForm<Value, Named, Child> {
    /// The function name, from the draft's definition of each form.
    #[must_use]
    pub fn function_name(&self) -> FunctionName<'_, Named> {
        FunctionName::BuiltIn(match self {
            Self::Constant(_) => return FunctionName::Constant,
            Self::CustomNamed(_) => return FunctionName::NamedParameter,
            Self::CustomList { function, .. } => return FunctionName::ListParameter(function),
            Self::Sparql { function, .. } => function.as_iri(),
            Self::Empty => v::EMPTY_EXPRESSION,
            Self::Var(_) => v::VAR_EXPRESSION,
            Self::List(_) => v::LIST_EXPRESSION,
            Self::PathValues { .. } => v::PATH_VALUES_EXPRESSION,
            Self::Exists(_) => v::EXISTS_EXPRESSION,
            Self::If { .. } => v::IF_EXPRESSION,
            Self::Distinct(_) => v::DISTINCT_EXPRESSION,
            Self::Intersection(_) => v::INTERSECTION_EXPRESSION,
            Self::Concat(_) => v::CONCAT_EXPRESSION,
            Self::Remove { .. } => v::REMOVE_EXPRESSION,
            Self::FilterShape { .. } => v::FILTER_SHAPE_EXPRESSION,
            Self::Limit { .. } => v::LIMIT_EXPRESSION,
            Self::Offset { .. } => v::OFFSET_EXPRESSION,
            Self::OrderBy { .. } => v::ORDER_BY_EXPRESSION,
            Self::FlatMap { .. } => v::FLAT_MAP_EXPRESSION,
            Self::FindFirst { .. } => v::FIND_FIRST_EXPRESSION,
            Self::MatchAll { .. } => v::MATCH_ALL_EXPRESSION,
            Self::Aggregate { aggregate, .. } => aggregate.function_iri(),
            Self::InstancesOf(_) => v::INSTANCES_OF_EXPRESSION,
            Self::NodesMatching(_) => v::NODES_MATCHING_EXPRESSION,
            Self::ConformsToShape { .. } => v::CONFORMS_TO_SHAPE,
            Self::Select(_) => v::SELECT_EXPRESSION,
            Self::SparqlExpr(_) => v::SPARQL_EXPR_EXPRESSION,
            Self::Arg(_) => v::ARG_EXPRESSION,
        })
    }

    /// The output-sequence contract the draft defines for this form.
    #[must_use]
    pub const fn output_contract(&self) -> OutputContract {
        match self {
            Self::Constant(_)
            | Self::Exists(_)
            | Self::MatchAll { .. }
            | Self::Aggregate {
                aggregate: Aggregate::Count | Aggregate::Sum,
                ..
            } => OutputContract::ExactlyOne,
            Self::Empty
            | Self::Var(_)
            | Self::FindFirst { .. }
            | Self::Aggregate { .. }
            | Self::ConformsToShape { .. }
            | Self::Sparql { .. }
            | Self::CustomList { .. }
            | Self::Arg(ArgumentKey::Position(_)) => OutputContract::AtMostOne,
            Self::List(_)
            | Self::Concat(_)
            | Self::Remove { .. }
            | Self::FilterShape { .. }
            | Self::Limit { .. }
            | Self::Offset { .. }
            | Self::OrderBy { .. }
            | Self::FlatMap { .. }
            | Self::Select(_)
            | Self::SparqlExpr(_) => OutputContract::OrderedSequence,
            Self::Distinct(_) => OutputContract::DistinctSequence,
            Self::PathValues { .. }
            | Self::Intersection(_)
            | Self::InstancesOf(_)
            | Self::NodesMatching(_) => OutputContract::Set,
            Self::If { .. } => OutputContract::SelectedBranch,
            Self::CustomNamed(_) => OutputContract::FunctionBody,
            Self::Arg(ArgumentKey::Parameter(_)) => OutputContract::ArgumentExpression,
        }
    }

    /// Visit every nested expression in declaration order.
    pub fn for_each_child(&self, mut visit: impl FnMut(&Child)) {
        match self {
            Self::Constant(_)
            | Self::Empty
            | Self::Var(_)
            | Self::List(_)
            | Self::NodesMatching(_)
            | Self::Select(_)
            | Self::SparqlExpr(_)
            | Self::Arg(_) => {}
            Self::PathValues { focus_node, .. } => focus_node.iter().for_each(visit),
            Self::FlatMap { flat_map, nodes } => {
                visit(flat_map);
                nodes.iter().for_each(visit);
            }
            Self::FindFirst { nodes, .. } | Self::MatchAll { nodes, .. } => {
                nodes.iter().for_each(visit);
            }
            Self::Exists(child)
            | Self::Distinct(child)
            | Self::InstancesOf(child)
            | Self::FilterShape { nodes: child, .. }
            | Self::Limit { nodes: child, .. }
            | Self::Offset { nodes: child, .. }
            | Self::Aggregate { input: child, .. } => visit(child),
            Self::If {
                condition,
                branches,
            } => {
                visit(condition);
                branches.then().into_iter().for_each(&mut visit);
                branches.otherwise().into_iter().for_each(visit);
            }
            Self::Remove {
                remove: a,
                nodes: b,
            }
            | Self::OrderBy {
                order_by: a,
                nodes: b,
                ..
            }
            | Self::ConformsToShape { node: a, shape: b } => {
                visit(a);
                visit(b);
            }
            Self::Intersection(children)
            | Self::Concat(children)
            | Self::Sparql {
                arguments: children,
                ..
            }
            | Self::CustomList {
                arguments: children,
                ..
            } => children.iter().for_each(visit),
            Self::CustomNamed(arguments) => arguments.iter().for_each(|arg| visit(&arg.value)),
        }
    }
}

impl Aggregate {
    /// The aggregation's `shnex:` function name.
    #[must_use]
    pub const fn function_iri(self) -> &'static str {
        match self {
            Self::Count => v::COUNT_EXPRESSION,
            Self::Min => v::MIN_EXPRESSION,
            Self::Max => v::MAX_EXPRESSION,
            Self::Sum => v::SUM_EXPRESSION,
        }
    }
}

impl<Named, Child> NamedArgument<Named, Child> {
    /// Pair a parameter predicate with its argument expression.
    #[must_use]
    pub const fn new(parameter: Named, value: Child) -> Self {
        Self { parameter, value }
    }
}

//! Rebuild an [`ExpressionForm`] with a different nested expression type.

use super::{ExpressionForm, ExpressionForm as F, NamedArgument};

impl<Value, Named, Child> ExpressionForm<Value, Named, Child> {
    /// Replace every nested expression, failing on the first rejected one.
    ///
    /// # Errors
    /// Returns the first error produced by `map`.
    pub fn try_map_children<Mapped, Error>(
        self,
        mut map: impl FnMut(Child) -> Result<Mapped, Error>,
    ) -> Result<ExpressionForm<Value, Named, Mapped>, Error> {
        Ok(match self {
            Self::Constant(value) => F::Constant(value),
            Self::Empty => F::Empty,
            Self::Var(name) => F::Var(name),
            Self::List(list) => F::List(list),
            Self::NodesMatching(shape) => F::NodesMatching(shape),
            Self::Select(text) => F::Select(text),
            Self::SparqlExpr(text) => F::SparqlExpr(text),
            Self::Arg(key) => F::Arg(key),
            Self::PathValues {
                path,
                path_node,
                focus_node,
            } => F::PathValues {
                path,
                path_node,
                focus_node: map_optional(focus_node, &mut map)?,
            },
            Self::Exists(child) => F::Exists(map(child)?),
            Self::Distinct(child) => F::Distinct(map(child)?),
            Self::InstancesOf(child) => F::InstancesOf(map(child)?),
            Self::If {
                condition,
                branches,
            } => F::If {
                condition: map(condition)?,
                branches: branches.try_map(&mut map)?,
            },
            Self::Intersection(children) => F::Intersection(map_all(children, &mut map)?),
            Self::Concat(children) => F::Concat(map_all(children, &mut map)?),
            Self::Remove { remove, nodes } => F::Remove {
                remove: map(remove)?,
                nodes: map(nodes)?,
            },
            Self::FilterShape { shape, nodes } => F::FilterShape {
                shape,
                nodes: map(nodes)?,
            },
            Self::Limit { limit, nodes } => F::Limit {
                limit,
                nodes: map(nodes)?,
            },
            Self::Offset { offset, nodes } => F::Offset {
                offset,
                nodes: map(nodes)?,
            },
            Self::OrderBy {
                order_by,
                nodes,
                descending,
            } => F::OrderBy {
                order_by: map(order_by)?,
                nodes: map(nodes)?,
                descending,
            },
            Self::FlatMap { flat_map, nodes } => F::FlatMap {
                flat_map: map(flat_map)?,
                nodes: map_optional(nodes, &mut map)?,
            },
            Self::FindFirst { shape, nodes } => F::FindFirst {
                shape,
                nodes: map_optional(nodes, &mut map)?,
            },
            Self::MatchAll { shape, nodes } => F::MatchAll {
                shape,
                nodes: map_optional(nodes, &mut map)?,
            },
            Self::Aggregate { aggregate, input } => F::Aggregate {
                aggregate,
                input: map(input)?,
            },
            Self::ConformsToShape { node, shape } => F::ConformsToShape {
                node: map(node)?,
                shape: map(shape)?,
            },
            Self::Sparql {
                function,
                arguments,
            } => F::Sparql {
                function,
                arguments: map_all(arguments, &mut map)?,
            },
            Self::CustomNamed(arguments) => F::CustomNamed(
                arguments
                    .into_iter()
                    .map(|argument| {
                        Ok(NamedArgument::new(argument.parameter, map(argument.value)?))
                    })
                    .collect::<Result<_, Error>>()?,
            ),
            Self::CustomList {
                function,
                arguments,
            } => F::CustomList {
                function,
                arguments: map_all(arguments, &mut map)?,
            },
        })
    }
}

fn map_all<Child, Mapped, Error>(
    children: Vec<Child>,
    map: &mut impl FnMut(Child) -> Result<Mapped, Error>,
) -> Result<Vec<Mapped>, Error> {
    children.into_iter().map(map).collect()
}

fn map_optional<Child, Mapped, Error>(
    child: Option<Child>,
    map: &mut impl FnMut(Child) -> Result<Mapped, Error>,
) -> Result<Option<Mapped>, Error> {
    child.map(map).transpose()
}

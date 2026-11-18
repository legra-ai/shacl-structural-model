//! The node-expression functions of SHACL 1.2 Node Expressions and SPARQL.

use iri_rfc3987::Iri;

use super::{
    Aggregate, ArgumentKey, ConstantList, IfBranches, NamedArgument, NodeExpression,
    SparqlExpressionText, SparqlFunction, VariableName,
};
use crate::ShaclPath;

/// One node-expression function and its operands.
///
/// Sections cite SHACL 1.2 Node Expressions, Working Draft 16 November 2026,
/// unless they name SHACL 1.2 SPARQL Extensions, Working Draft 22 October 2026.
/// `Child` is the nested expression type. It defaults to a checked
/// [`NodeExpression`]; the crate also instantiates it for budgeted decoding.
/// Built-in named parameter functions keep their parameters as typed fields;
/// list parameter functions keep positional argument lists.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Value: serde::Serialize, Named: serde::Serialize, Child: serde::Serialize",
        deserialize = "Value: serde::Deserialize<'de>, Named: serde::Deserialize<'de>, Child: super::child::DecodedChild<'de>"
    )
)]
pub enum ExpressionForm<
    Value = crate::DefaultTerm,
    Named = Iri<String>,
    Child = Box<NodeExpression<Value, Named>>,
> {
    /// An IRI, literal or triple-term constant (3.1.1 to 3.1.3).
    ///
    /// A blank node is never a constant; the consumer checks its own term type.
    Constant(Value),
    /// The empty expression `[]` (4.1.1).
    Empty,
    /// `shnex:var` (4.1.2); `"focusNode"` replaces the AF `sh:this`.
    Var(VariableName),
    /// A list expression of constants (4.1.3).
    List(ConstantList<Value>),
    /// `shnex:pathValues` with optional `shnex:focusNode` (4.1.4).
    PathValues {
        /// The SHACL property path.
        path: ShaclPath<Named>,
        /// The original path node, when retained by the caller.
        path_node: Option<Value>,
        /// The focus-node expression; absence means the evaluation's focus node.
        focus_node: Option<Child>,
    },
    /// `shnex:exists` (4.1.5).
    Exists(Child),
    /// `shnex:if` with `shnex:then` and/or `shnex:else` (4.1.6).
    If {
        /// The `shnex:if` condition.
        condition: Child,
        /// The declared branches.
        branches: IfBranches<Child>,
    },
    /// `shnex:distinct` (4.2.1).
    Distinct(Child),
    /// `shnex:intersection` over a SHACL list of expressions (4.2.2).
    Intersection(Vec<Child>),
    /// `shnex:concat` over a SHACL list of expressions (4.2.3); replaces AF `sh:union`.
    Concat(Vec<Child>),
    /// `shnex:remove` from `shnex:nodes` (4.2.4); replaces AF `sh:minus`.
    Remove {
        /// The `shnex:remove` expression.
        remove: Child,
        /// The `shnex:nodes` input.
        nodes: Child,
    },
    /// `shnex:filterShape` over `shnex:nodes` (4.2.5).
    FilterShape {
        /// The IRI or blank-node shape reference.
        shape: Value,
        /// The `shnex:nodes` input.
        nodes: Child,
    },
    /// `shnex:limit` over `shnex:nodes` (4.2.6).
    Limit {
        /// The caller's non-negative `xsd:integer` literal.
        limit: Value,
        /// The `shnex:nodes` input.
        nodes: Child,
    },
    /// `shnex:offset` over `shnex:nodes` (4.2.7).
    Offset {
        /// The caller's non-negative `xsd:integer` literal.
        offset: Value,
        /// The `shnex:nodes` input.
        nodes: Child,
    },
    /// `shnex:orderBy` over `shnex:nodes`, with optional `shnex:desc` (4.2.8).
    OrderBy {
        /// The sort-key expression, evaluated per input node.
        order_by: Child,
        /// The `shnex:nodes` input.
        nodes: Child,
        /// `shnex:desc`; absence (ascending) stays distinct from explicit `false`.
        descending: Option<bool>,
    },
    /// `shnex:flatMap` (4.3.1).
    FlatMap {
        /// The expression evaluated with each input node as focus node.
        flat_map: Child,
        /// Optional `shnex:nodes`; absence means the focus node.
        nodes: Option<Child>,
    },
    /// `shnex:findFirst` (4.3.2).
    FindFirst {
        /// The IRI or blank-node shape reference.
        shape: Value,
        /// Optional `shnex:nodes`; absence means the focus node.
        nodes: Option<Child>,
    },
    /// `shnex:matchAll` (4.3.3).
    MatchAll {
        /// The IRI or blank-node shape reference.
        shape: Value,
        /// Optional `shnex:nodes`; absence means the focus node.
        nodes: Option<Child>,
    },
    /// `shnex:count`, `shnex:min`, `shnex:max` or `shnex:sum` (4.4).
    Aggregate {
        /// The aggregation.
        aggregate: Aggregate,
        /// The aggregated input expression.
        input: Child,
    },
    /// `shnex:instancesOf` (4.5.1).
    InstancesOf(Child),
    /// `shnex:nodesMatching` (4.5.2).
    NodesMatching(Value),
    /// The `shnex:conformsToShape` list parameter function (4.5.3).
    ConformsToShape {
        /// The first argument: the node to validate.
        node: Child,
        /// The second argument: an expression producing the shape IRI.
        shape: Child,
    },
    /// A SHACL SPARQL function expression `sparql:<NAME>` (5).
    Sparql {
        /// The function.
        function: SparqlFunction,
        /// The positional arguments in list order.
        arguments: Vec<Child>,
    },
    /// `sh:select` (SHACL-SPARQL 6.1).
    Select(SparqlExpressionText<Value>),
    /// `sh:sparqlExpr` (SHACL-SPARQL 6.2).
    SparqlExpr(SparqlExpressionText<Value>),
    /// A custom named parameter expression (6.1), keyed by parameter predicate.
    ///
    /// At least one argument is required; the consumer resolves the function
    /// from its key parameters.
    CustomNamed(Vec<NamedArgument<Named, Child>>),
    /// A custom list parameter expression (6.2).
    CustomList {
        /// The function IRI, which is its list parameter property.
        function: Named,
        /// The positional arguments in list order.
        arguments: Vec<Child>,
    },
    /// `shnex:arg` (6.3).
    Arg(ArgumentKey<Value, Named>),
}

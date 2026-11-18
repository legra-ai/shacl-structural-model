//! SHACL 1.2 node expressions, the targets and values that use them, and
//! custom node-expression function declarations.
//!
//! The model follows SHACL 1.2 Node Expressions, Working Draft 16 November
//! 2026, and the SPARQL-based forms of SHACL 1.2 SPARQL Extensions, Working
//! Draft 22 October 2026. It represents declarations; it evaluates nothing.

mod child;
mod constraint;
mod contract;
mod display;
mod error;
mod expression;
mod footprint;
mod form;
mod form_map;
mod form_ops;
mod function;
mod parts;
mod sparql_function;

pub use child::DecodedChild;
pub use constraint::ExpressionConstraint;
pub use contract::OutputContract;
pub use error::ExpressionError;
pub use expression::NodeExpression;
use footprint::Footprint;
pub use form::ExpressionForm;
pub use form_ops::FunctionName;
pub use function::{ExpressionFunction, ParameterStyle};
pub use parts::{
    Aggregate, ArgumentKey, ConstantList, IfBranches, NamedArgument, SparqlExpressionText,
    VariableName,
};
pub use sparql_function::SparqlFunction;

/// The node-expression draft this module follows.
pub const NODE_EXPRESSIONS_DRAFT: &str =
    "https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/";
/// The SHACL-SPARQL draft whose select and SPARQL expr expressions are modelled.
pub const SPARQL_DRAFT: &str = "https://www.w3.org/TR/2026/WD-shacl12-sparql-20261022/";

//! SHACL-AF node expressions. These values describe expressions, not execution.

use iri_rfc3987::Iri;

use crate::ShaclPath;

/// A node expression paired with its original RDF node.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodeExpression<Value = crate::DefaultTerm> {
    /// The source node: `sh:this`, a constant term, or an expression blank node.
    pub node: Value,
    /// The expression's structural operation.
    pub kind: ExpressionKind<Value>,
}

impl<Value> NodeExpression<Value> {
    /// Pair an expression with its original RDF node without inspecting it.
    #[must_use]
    pub fn new(node: Value, kind: ExpressionKind<Value>) -> Self {
        Self { node, kind }
    }
}

/// The seven node expression forms defined by SHACL-AF.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ExpressionKind<Value = crate::DefaultTerm> {
    /// The focus node (`sh:this`).
    This,
    /// The containing expression's `node` is its constant RDF term.
    Constant,
    /// A `sh:path` expression with an optional `sh:nodes` input expression.
    Path {
        /// The parsed path structure.
        path: ShaclPath,
        /// The original path node, when retained by the caller.
        path_node: Option<Value>,
        /// An explicit input expression; absence means the focus node.
        nodes: Option<Box<NodeExpression<Value>>>,
    },
    /// A `sh:filterShape` expression.
    Filter {
        /// The filter shape's IRI or blank-node reference.
        shape: Value,
        /// The `sh:nodes` input expression.
        nodes: Box<NodeExpression<Value>>,
    },
    /// A SHACL function call.
    Function {
        /// The function IRI.
        name: Iri<String>,
        /// Arguments in RDF-list order.
        args: Vec<NodeExpression<Value>>,
    },
    /// A `sh:union` expression, retaining RDF-list order.
    Union(Vec<NodeExpression<Value>>),
    /// A `sh:intersection` expression, retaining RDF-list order.
    Intersection(Vec<NodeExpression<Value>>),
}

/// A `sh:expression` constraint with expression-specific messages.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExpressionConstraint<Value = crate::DefaultTerm> {
    /// The expression, including its source node for result reporting.
    pub expression: NodeExpression<Value>,
    /// RDF `sh:message` literals attached to the expression.
    pub messages: Vec<Value>,
}

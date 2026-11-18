//! The SHACL vocabulary IRIs this crate decodes and encodes.
//!
//! Every `sh:` IRI used by the model is declared here once, so decoding and
//! encoding share one spelling. The constants are full IRIs in the
//! `http://www.w3.org/ns/shacl#` namespace, followed by the SHACL 1.2 node
//! expression function names of the `shnex:` and `sparql:` namespaces.

/// Expand a SHACL local name into its full IRI literal.
macro_rules! sh {
    ($local:literal) => {
        concat!("http://www.w3.org/ns/shacl#", $local)
    };
}

/// Expand a SHACL 1.2 node-expression local name into its full IRI literal.
macro_rules! shnex {
    ($local:literal) => {
        concat!("http://www.w3.org/ns/shacl-node-expr#", $local)
    };
}

/// The SHACL namespace IRI, `sh:`.
pub const NAMESPACE: &str = sh!("");

/// `sh:Violation`, the most severe standard result severity.
pub const VIOLATION: &str = sh!("Violation");
/// `sh:Warning`, the intermediate standard result severity.
pub const WARNING: &str = sh!("Warning");
/// `sh:Info`, the least severe standard severity that reports a violation.
pub const INFO: &str = sh!("Info");
/// `sh:Debug`, a SHACL 1.2 severity for a debug message that is not a violation.
pub const DEBUG: &str = sh!("Debug");
/// `sh:Trace`, the least severe SHACL 1.2 standard severity.
pub const TRACE: &str = sh!("Trace");

/// `sh:ByTypes`, the SHACL 1.2 `sh:closed` value that closes a shape by types.
pub const BY_TYPES: &str = sh!("ByTypes");

/// `sh:IRI`, the node kind of IRIs.
pub const IRI: &str = sh!("IRI");
/// `sh:BlankNode`, the node kind of blank nodes.
pub const BLANK_NODE: &str = sh!("BlankNode");
/// `sh:Literal`, the node kind of literals.
pub const LITERAL: &str = sh!("Literal");
/// `sh:BlankNodeOrIRI`, the node kind of blank nodes and IRIs.
pub const BLANK_NODE_OR_IRI: &str = sh!("BlankNodeOrIRI");
/// `sh:BlankNodeOrLiteral`, the node kind of blank nodes and literals.
pub const BLANK_NODE_OR_LITERAL: &str = sh!("BlankNodeOrLiteral");
/// `sh:IRIOrLiteral`, the node kind of IRIs and literals.
pub const IRI_OR_LITERAL: &str = sh!("IRIOrLiteral");
/// `sh:TripleTerm`, the SHACL 1.2 node kind of triple terms (Core 6.10.1.3).
pub const TRIPLE_TERM: &str = sh!("TripleTerm");

/// `sh:Graph`, the class of graphs (Core 6.1).
pub const GRAPH: &str = sh!("Graph");
/// `sh:DataGraph`, the class of graphs in the data-graph role (Core 6.2).
pub const DATA_GRAPH_CLASS: &str = sh!("DataGraph");
/// `sh:ShapesGraph`, the class of graphs in the shapes-graph role (Core 6.1).
pub const SHAPES_GRAPH_CLASS: &str = sh!("ShapesGraph");
/// `sh:RulesGraph`, the class of graphs in the rules-graph role (Inference Rules 5).
pub const RULES_GRAPH_CLASS: &str = sh!("RulesGraph");
/// `sh:shapesGraph`, a data graph's link to a shapes graph (Core 6.5).
pub const SHAPES_GRAPH: &str = sh!("shapesGraph");
/// `sh:dataGraph`, a shapes graph's link to a data graph (Core 6.6).
pub const DATA_GRAPH: &str = sh!("dataGraph");
/// `sh:suggestedShapesGraph`, an ontology's suggested shapes graph.
pub const SUGGESTED_SHAPES_GRAPH: &str = sh!("suggestedShapesGraph");
/// `sh:RulesEntailment`, the SHACL rules entailment regime (Inference Rules 6.1).
pub const RULES_ENTAILMENT: &str = sh!("RulesEntailment");

/// `sh:SPARQLRule`, the SHACL SPARQL rule type (Inference Rules 11.1).
pub const SPARQL_RULE: &str = sh!("SPARQLRule");
/// `sh:TripleRule`, the triple rule type (Inference Rules 11.2).
pub const TRIPLE_RULE: &str = sh!("TripleRule");
/// `sh:SPARQLRuleTemplate`, the class of SPARQL rule templates (Inference Rules 12).
pub const SPARQL_RULE_TEMPLATE: &str = sh!("SPARQLRuleTemplate");

/// The SHACL 1.2 node-expression namespace IRI, `shnex:`.
pub const NODE_EXPR_NAMESPACE: &str = shnex!("");
/// The SPARQL function namespace IRI, `sparql:`, of SHACL 1.2 Node Expressions section 5.
pub const SPARQL_NAMESPACE: &str = "http://www.w3.org/ns/sparql#";

/// `sh:IRIExpression`, the function name of an IRI constant (Node Expressions 3.1.1).
pub const IRI_EXPRESSION: &str = sh!("IRIExpression");
/// `sh:LiteralExpression`, the function name of a literal constant (Node Expressions 3.1.2).
pub const LITERAL_EXPRESSION: &str = sh!("LiteralExpression");
/// `sh:TripleTermExpression`, the function name of a triple-term constant (Node Expressions 3.1.3).
pub const TRIPLE_TERM_EXPRESSION: &str = sh!("TripleTermExpression");
/// `sh:SelectExpression`, a SHACL 1.2 SPARQL select expression (SHACL-SPARQL 6.1).
pub const SELECT_EXPRESSION: &str = sh!("SelectExpression");
/// `sh:SPARQLExprExpression`, a SHACL 1.2 SPARQL expr expression (SHACL-SPARQL 6.2).
pub const SPARQL_EXPR_EXPRESSION: &str = sh!("SPARQLExprExpression");
/// `sh:NamedParameterExpression`, the class of custom named parameter expressions (Node Expressions 6.1).
pub const NAMED_PARAMETER_EXPRESSION: &str = sh!("NamedParameterExpression");
/// `sh:ListParameterExpression`, the class of custom list parameter expressions (Node Expressions 6.2).
pub const LIST_PARAMETER_EXPRESSION: &str = sh!("ListParameterExpression");
/// `shnex:EmptyExpression` (SHACL 1.2 Node Expressions section 4.1.1).
pub const EMPTY_EXPRESSION: &str = shnex!("EmptyExpression");
/// `shnex:VarExpression` (SHACL 1.2 Node Expressions section 4.1.2).
pub const VAR_EXPRESSION: &str = shnex!("VarExpression");
/// `shnex:ListExpression` (SHACL 1.2 Node Expressions section 4.1.3).
pub const LIST_EXPRESSION: &str = shnex!("ListExpression");
/// `shnex:PathValuesExpression` (SHACL 1.2 Node Expressions section 4.1.4).
pub const PATH_VALUES_EXPRESSION: &str = shnex!("PathValuesExpression");
/// `shnex:ExistsExpression` (SHACL 1.2 Node Expressions section 4.1.5).
pub const EXISTS_EXPRESSION: &str = shnex!("ExistsExpression");
/// `shnex:IfExpression` (SHACL 1.2 Node Expressions section 4.1.6).
pub const IF_EXPRESSION: &str = shnex!("IfExpression");
/// `shnex:DistinctExpression` (SHACL 1.2 Node Expressions section 4.2.1).
pub const DISTINCT_EXPRESSION: &str = shnex!("DistinctExpression");
/// `shnex:IntersectionExpression` (SHACL 1.2 Node Expressions section 4.2.2).
pub const INTERSECTION_EXPRESSION: &str = shnex!("IntersectionExpression");
/// `shnex:ConcatExpression` (SHACL 1.2 Node Expressions section 4.2.3).
pub const CONCAT_EXPRESSION: &str = shnex!("ConcatExpression");
/// `shnex:RemoveExpression` (SHACL 1.2 Node Expressions section 4.2.4).
pub const REMOVE_EXPRESSION: &str = shnex!("RemoveExpression");
/// `shnex:FilterShapeExpression` (SHACL 1.2 Node Expressions section 4.2.5).
pub const FILTER_SHAPE_EXPRESSION: &str = shnex!("FilterShapeExpression");
/// `shnex:LimitExpression` (SHACL 1.2 Node Expressions section 4.2.6).
pub const LIMIT_EXPRESSION: &str = shnex!("LimitExpression");
/// `shnex:OffsetExpression` (SHACL 1.2 Node Expressions section 4.2.7).
pub const OFFSET_EXPRESSION: &str = shnex!("OffsetExpression");
/// `shnex:OrderByExpression` (SHACL 1.2 Node Expressions section 4.2.8).
pub const ORDER_BY_EXPRESSION: &str = shnex!("OrderByExpression");
/// `shnex:FlatMapExpression` (SHACL 1.2 Node Expressions section 4.3.1).
pub const FLAT_MAP_EXPRESSION: &str = shnex!("FlatMapExpression");
/// `shnex:FindFirstExpression` (SHACL 1.2 Node Expressions section 4.3.2).
pub const FIND_FIRST_EXPRESSION: &str = shnex!("FindFirstExpression");
/// `shnex:MatchAllExpression` (SHACL 1.2 Node Expressions section 4.3.3).
pub const MATCH_ALL_EXPRESSION: &str = shnex!("MatchAllExpression");
/// `shnex:CountExpression` (SHACL 1.2 Node Expressions section 4.4.1).
pub const COUNT_EXPRESSION: &str = shnex!("CountExpression");
/// `shnex:MinExpression` (SHACL 1.2 Node Expressions section 4.4.2).
pub const MIN_EXPRESSION: &str = shnex!("MinExpression");
/// `shnex:MaxExpression` (SHACL 1.2 Node Expressions section 4.4.3).
pub const MAX_EXPRESSION: &str = shnex!("MaxExpression");
/// `shnex:SumExpression` (SHACL 1.2 Node Expressions section 4.4.4).
pub const SUM_EXPRESSION: &str = shnex!("SumExpression");
/// `shnex:InstancesOfExpression` (SHACL 1.2 Node Expressions section 4.5.1).
pub const INSTANCES_OF_EXPRESSION: &str = shnex!("InstancesOfExpression");
/// `shnex:NodesMatchingExpression` (SHACL 1.2 Node Expressions section 4.5.2).
pub const NODES_MATCHING_EXPRESSION: &str = shnex!("NodesMatchingExpression");
/// `shnex:conformsToShape` (SHACL 1.2 Node Expressions section 4.5.3).
pub const CONFORMS_TO_SHAPE: &str = shnex!("conformsToShape");
/// `shnex:ArgExpression` (SHACL 1.2 Node Expressions section 6.3).
pub const ARG_EXPRESSION: &str = shnex!("ArgExpression");
/// `sh:NamedParameterExpressionFunction`, the class of custom named parameter functions (Node Expressions 6.1).
pub const NAMED_PARAMETER_EXPRESSION_FUNCTION: &str = sh!("NamedParameterExpressionFunction");
/// `sh:ListParameterExpressionFunction`, the class of custom list parameter functions (Node Expressions 6.2).
pub const LIST_PARAMETER_EXPRESSION_FUNCTION: &str = sh!("ListParameterExpressionFunction");

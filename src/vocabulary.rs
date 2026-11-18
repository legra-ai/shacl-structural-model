//! The SHACL vocabulary IRIs this crate decodes and encodes.
//!
//! Every `sh:` IRI used by the model is declared here once, so decoding and
//! encoding share one spelling. The constants are full IRIs in the
//! `http://www.w3.org/ns/shacl#` namespace.

/// Expand a SHACL local name into its full IRI literal.
macro_rules! sh {
    ($local:literal) => {
        concat!("http://www.w3.org/ns/shacl#", $local)
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

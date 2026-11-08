//! Restricted, checked declarations for the SHACL 1.2 Inference Rules draft.
//!
//! This edition is distinct from the crate's AF declarations. It contains no
//! scheduler, graph resolver, validation or execution. Membership/include/link
//! records can be streamed individually; no graph-sized aggregate is required.

mod number;
mod records;

pub use number::RuleNumber;
/// IRI or scoped blank-node identity, using the shared RDF subject model.
pub use rdf_structural_model::Subject as ResourceRef;
pub use records::{RuleSchedule, RuleSet, RuleSetInclusion, RuleSetMembership, ShapeRuleLink};

/// The exact inference draft described by this restricted model.
pub const INFERENCE_DRAFT: &str = "https://www.w3.org/TR/2026/WD-shacl12-inference-rules-20261028/";
/// The exact node-expression draft used by this restricted model.
pub const EXPRESSION_DRAFT: &str = "https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261028/";

/// Checked declaration errors; graph resolution and execution errors are consumer-owned.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum InferenceModelError {
    /// Layer/order was not a valid exact integer or decimal literal.
    #[error("rule layer/order requires a valid xsd:integer or xsd:decimal literal")]
    InvalidRuleNumber,
}

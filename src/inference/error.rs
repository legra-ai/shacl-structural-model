//! The pinned draft and checked declaration errors.

/// The exact inference draft described by this model.
pub const INFERENCE_DRAFT: &str = "https://www.w3.org/TR/2026/WD-shacl12-inference-rules-20261110/";

/// Checked declaration errors; graph resolution and execution errors are consumer-owned.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum InferenceModelError {
    /// Layer/order was not a valid exact integer or decimal literal.
    #[error("rule layer/order requires a valid xsd:integer or xsd:decimal literal")]
    InvalidRuleNumber,
    /// `sh:construct` was not an `xsd:string` literal.
    #[error("sh:construct requires an xsd:string literal")]
    InvalidConstructQuery,
}

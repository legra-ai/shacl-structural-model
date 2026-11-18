//! Errors raised while checking node-expression structure.

/// A structural rule of SHACL 1.2 node expressions, or a model budget, was violated.
///
/// Evaluation failures (Node Expressions section 3.3) are consumer-owned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum ExpressionError {
    /// A sequence or alternative path had fewer than two members.
    #[error("sequence and alternative paths require at least two members")]
    InvalidPathArity,
    /// A `shnex:var` name was empty (section 4.1.2 requires `sh:minLength 1`).
    #[error("a shnex:var name must not be empty")]
    EmptyVariableName,
    /// A custom named parameter expression had no argument (section 6.1).
    #[error("a custom named parameter expression requires a key argument")]
    NoNamedArgument,
    /// The tree exceeded the model's structural depth or node budget.
    #[error("node expression structural budget exceeded")]
    BudgetExceeded,
}

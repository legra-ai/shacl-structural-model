//! `sh:intent` statements and their `sh:formalized` reifier (Core section 6.11.2).

/// One `sh:intent` statement of a shape.
///
/// SHACL 1.2 Core, Working Draft 16 November 2026, section 6.11.2. The
/// statement is a caller-owned RDF literal. `formalized` is the
/// `sh:formalized` value of a reifier of the `sh:intent` triple; `None` means
/// the triple has no such reifier, distinct from explicit `false`.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct Intent<Value = crate::DefaultTerm> {
    /// The human-readable intended rule, assumption or constraint.
    pub statement: Value,
    /// The `sh:formalized` reifier value, if declared.
    pub formalized: Option<bool>,
}

impl<Value> Intent<Value> {
    /// An intent statement without a `sh:formalized` reifier.
    #[must_use]
    pub const fn new(statement: Value) -> Self {
        Self {
            statement,
            formalized: None,
        }
    }

    /// Record the `sh:formalized` reifier value.
    #[must_use]
    pub fn with_formalized(mut self, formalized: bool) -> Self {
        self.formalized = Some(formalized);
        self
    }

    /// Whether the shapes graph declares this intent as formalized in SHACL.
    #[must_use]
    pub fn is_formalized(&self) -> bool {
        self.formalized == Some(true)
    }
}

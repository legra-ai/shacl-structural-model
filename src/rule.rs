//! SHACL Advanced Features rule declarations.

/// A SHACL-AF rule declaration.
#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ShaclRule {
    /// A `sh:SPARQLRule` represented by its raw CONSTRUCT query.
    Sparql {
        /// The CONSTRUCT query text.
        construct_query: String,
    },
}

//! SPARQL rules and rules typed by a SPARQL rule template.

use borrow_or_share::Bos;
use iri_rfc3987::Iri;
use rdf_structural_model::{Subject, Term};

use super::{ConstructQuery, RuleMetadata};
use crate::ParameterValues;

/// A `sh:SPARQLRule` declaration (Inference Rules section 11.1), not an executable rule.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Text: serde::Serialize",
        deserialize = "Text: serde::Deserialize<'de>, Iri<Text>: serde::Deserialize<'de>"
    )
)]
pub struct SparqlRuleDeclaration<Text: Bos<str> + AsRef<str> = String> {
    id: Subject<Text>,
    metadata: RuleMetadata<Text>,
    query: ConstructQuery<Text>,
}

impl<Text: Bos<str> + AsRef<str>> SparqlRuleDeclaration<Text> {
    /// A rule with its CONSTRUCT query and no metadata.
    #[must_use]
    pub const fn new(id: Subject<Text>, query: ConstructQuery<Text>) -> Self {
        Self {
            id,
            metadata: RuleMetadata::new(),
            query,
        }
    }
    /// Set the common rule declarations.
    #[must_use]
    pub fn with_metadata(mut self, metadata: RuleMetadata<Text>) -> Self {
        self.metadata = metadata;
        self
    }
    /// The rule's identity.
    #[must_use]
    pub const fn id(&self) -> &Subject<Text> {
        &self.id
    }
    /// The common rule declarations.
    #[must_use]
    pub const fn metadata(&self) -> &RuleMetadata<Text> {
        &self.metadata
    }
    /// The CONSTRUCT query.
    #[must_use]
    pub const fn query(&self) -> &ConstructQuery<Text> {
        &self.query
    }
}

/// A rule that is an instance of a SPARQL rule template (Inference Rules section 12).
///
/// Arguments keep every value of each parameter predicate; the consumer
/// checks required parameters and builds pre-bindings.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Text: serde::Serialize",
        deserialize = "Text: serde::Deserialize<'de>, Iri<Text>: serde::Deserialize<'de>"
    )
)]
pub struct TemplateRuleDeclaration<Text: Bos<str> + AsRef<str> = String> {
    id: Subject<Text>,
    template: Iri<Text>,
    metadata: RuleMetadata<Text>,
    arguments: Vec<ParameterValues<Term<Text>, Iri<Text>>>,
}

impl<Text: Bos<str> + AsRef<str>> TemplateRuleDeclaration<Text> {
    /// A rule typed by `template`, with no arguments or metadata.
    #[must_use]
    pub const fn new(id: Subject<Text>, template: Iri<Text>) -> Self {
        Self {
            id,
            template,
            metadata: RuleMetadata::new(),
            arguments: Vec::new(),
        }
    }
    /// Set the common rule declarations.
    #[must_use]
    pub fn with_metadata(mut self, metadata: RuleMetadata<Text>) -> Self {
        self.metadata = metadata;
        self
    }
    /// Add the values of one template parameter.
    #[must_use]
    pub fn with_argument(mut self, argument: ParameterValues<Term<Text>, Iri<Text>>) -> Self {
        self.arguments.push(argument);
        self
    }
    /// The rule's identity.
    #[must_use]
    pub const fn id(&self) -> &Subject<Text> {
        &self.id
    }
    /// The template IRI, the rule's rule type.
    #[must_use]
    pub const fn template(&self) -> &Iri<Text> {
        &self.template
    }
    /// The common rule declarations.
    #[must_use]
    pub const fn metadata(&self) -> &RuleMetadata<Text> {
        &self.metadata
    }
    /// The parameter values in caller order.
    #[must_use]
    pub fn arguments(&self) -> &[ParameterValues<Term<Text>, Iri<Text>>] {
        &self.arguments
    }
}

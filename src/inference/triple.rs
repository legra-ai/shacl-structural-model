//! Independent triple-rule declarations with optional expression slots.
use borrow_or_share::Bos;
use iri_rfc3987::Iri;
use rdf_structural_model::{Subject, Term};

use super::RuleMetadata;
use crate::NodeExpression;

/// A SHACL 1.2 node expression over the shared RDF term model.
pub type RuleExpression<Text = String> = NodeExpression<Term<Text>, Iri<Text>>;

/// A `sh:TripleRule` declaration (section 11.2), not an executable rule.
///
/// Omitted slots remain omitted. No global/shape status is guessed here; the
/// consumer examines independent shape links across its relevant shapes graph.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Text: serde::Serialize",
        deserialize = "Text: serde::Deserialize<'de>, Iri<Text>: serde::Deserialize<'de>"
    )
)]
pub struct TripleRuleDeclaration<Text: Bos<str> + AsRef<str> = String> {
    id: Subject<Text>,
    metadata: RuleMetadata<Text>,
    subject: Option<RuleExpression<Text>>,
    predicate: Option<RuleExpression<Text>>,
    object: Option<RuleExpression<Text>>,
}
impl<Text: Bos<str> + AsRef<str>> TripleRuleDeclaration<Text> {
    /// Create a declaration with all expression slots and scheduling metadata absent.
    #[must_use]
    pub const fn new(id: Subject<Text>) -> Self {
        Self {
            id,
            metadata: RuleMetadata::new(),
            subject: None,
            predicate: None,
            object: None,
        }
    }
    /// Set the common rule declarations without applying execution defaults.
    #[must_use]
    pub fn with_metadata(mut self, metadata: RuleMetadata<Text>) -> Self {
        self.metadata = metadata;
        self
    }
    /// Set the subject expression.
    #[must_use]
    pub fn with_subject(mut self, subject: RuleExpression<Text>) -> Self {
        self.subject = Some(subject);
        self
    }
    /// Set the predicate expression.
    #[must_use]
    pub fn with_predicate(mut self, predicate: RuleExpression<Text>) -> Self {
        self.predicate = Some(predicate);
        self
    }
    /// Set the object expression.
    #[must_use]
    pub fn with_object(mut self, object: RuleExpression<Text>) -> Self {
        self.object = Some(object);
        self
    }
    /// The rule's identity.
    #[must_use]
    pub const fn id(&self) -> &Subject<Text> {
        &self.id
    }
    /// The common rule declarations, including scheduling.
    #[must_use]
    pub const fn metadata(&self) -> &RuleMetadata<Text> {
        &self.metadata
    }
    /// The optional subject expression.
    #[must_use]
    pub const fn subject(&self) -> Option<&RuleExpression<Text>> {
        self.subject.as_ref()
    }
    /// The optional predicate expression.
    #[must_use]
    pub const fn predicate(&self) -> Option<&RuleExpression<Text>> {
        self.predicate.as_ref()
    }
    /// The optional object expression.
    #[must_use]
    pub const fn object(&self) -> Option<&RuleExpression<Text>> {
        self.object.as_ref()
    }
}

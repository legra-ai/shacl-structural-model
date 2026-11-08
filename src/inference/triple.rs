//! Independent triple-rule declarations with optional expression slots.
use borrow_or_share::Bos;
use iri_rfc3987::Iri;
use rdf_structural_model::Subject;

use super::{DirectExpression, RuleSchedule};

/// A restricted draft triple-rule declaration, not an executable rule.
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
    schedule: RuleSchedule<Text>,
    subject: Option<DirectExpression<Text>>,
    predicate: Option<DirectExpression<Text>>,
    object: Option<DirectExpression<Text>>,
}
impl<Text: Bos<str> + AsRef<str>> TripleRuleDeclaration<Text> {
    /// Create a declaration with all expression slots and scheduling metadata absent.
    #[must_use]
    pub const fn new(id: Subject<Text>) -> Self {
        Self {
            id,
            schedule: RuleSchedule::new(),
            subject: None,
            predicate: None,
            object: None,
        }
    }
    /// Set explicit scheduling metadata without applying execution defaults.
    #[must_use]
    pub fn with_schedule(mut self, schedule: RuleSchedule<Text>) -> Self {
        self.schedule = schedule;
        self
    }
    /// Set the subject expression.
    #[must_use]
    pub fn with_subject(mut self, subject: DirectExpression<Text>) -> Self {
        self.subject = Some(subject);
        self
    }
    /// Set the predicate expression.
    #[must_use]
    pub fn with_predicate(mut self, predicate: DirectExpression<Text>) -> Self {
        self.predicate = Some(predicate);
        self
    }
    /// Set the object expression.
    #[must_use]
    pub fn with_object(mut self, object: DirectExpression<Text>) -> Self {
        self.object = Some(object);
        self
    }
    /// The rule's identity.
    #[must_use]
    pub const fn id(&self) -> &Subject<Text> {
        &self.id
    }
    /// Explicit scheduling metadata.
    #[must_use]
    pub const fn schedule(&self) -> &RuleSchedule<Text> {
        &self.schedule
    }
    /// The optional subject expression.
    #[must_use]
    pub const fn subject(&self) -> Option<&DirectExpression<Text>> {
        self.subject.as_ref()
    }
    /// The optional predicate expression.
    #[must_use]
    pub const fn predicate(&self) -> Option<&DirectExpression<Text>> {
        self.predicate.as_ref()
    }
    /// The optional object expression.
    #[must_use]
    pub const fn object(&self) -> Option<&DirectExpression<Text>> {
        self.object.as_ref()
    }
}

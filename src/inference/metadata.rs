//! Declarations common to every rule type (Inference Rules sections 3.3 to 3.8 and 10).

use borrow_or_share::Bos;
use iri_rfc3987::Iri;
use rdf_structural_model::Subject;

use super::{RuleProcessor, RuleSchedule};

/// Scheduling, conditions, deactivation, expected predicates and processors of one rule.
///
/// Absence is retained: an absent `sh:deactivated` is `None`, distinct from
/// explicit `false`. Conditions are shape references the consumer resolves.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Text: serde::Serialize",
        deserialize = "Text: serde::Deserialize<'de>, Iri<Text>: serde::Deserialize<'de>"
    )
)]
pub struct RuleMetadata<Text: Bos<str> + AsRef<str> = String> {
    schedule: RuleSchedule<Text>,
    conditions: Vec<Subject<Text>>,
    deactivated: Option<bool>,
    expected_predicates: Vec<Iri<Text>>,
    processors: Vec<RuleProcessor<Text>>,
}

impl<Text: Bos<str> + AsRef<str>> RuleMetadata<Text> {
    /// Metadata with every declaration absent.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            schedule: RuleSchedule::new(),
            conditions: Vec::new(),
            deactivated: None,
            expected_predicates: Vec::new(),
            processors: Vec::new(),
        }
    }
    /// Set `sh:layer`, `sh:order` and `sh:runOnce` (sections 3.5 to 3.7).
    #[must_use]
    pub fn with_schedule(mut self, schedule: RuleSchedule<Text>) -> Self {
        self.schedule = schedule;
        self
    }
    /// Add a `sh:condition` shape (section 3.3).
    #[must_use]
    pub fn with_condition(mut self, shape: Subject<Text>) -> Self {
        self.conditions.push(shape);
        self
    }
    /// Set the boolean `sh:deactivated` (section 3.4).
    #[must_use]
    pub const fn with_deactivated(mut self, deactivated: bool) -> Self {
        self.deactivated = Some(deactivated);
        self
    }
    /// Add a `sh:expectedPredicate` (section 3.8).
    #[must_use]
    pub fn with_expected_predicate(mut self, predicate: Iri<Text>) -> Self {
        self.expected_predicates.push(predicate);
        self
    }
    /// Add a `sh:ruleProcessor` (section 10).
    #[must_use]
    pub fn with_processor(mut self, processor: RuleProcessor<Text>) -> Self {
        self.processors.push(processor);
        self
    }
    /// The scheduling declarations.
    #[must_use]
    pub const fn schedule(&self) -> &RuleSchedule<Text> {
        &self.schedule
    }
    /// The condition shapes.
    #[must_use]
    pub fn conditions(&self) -> &[Subject<Text>] {
        &self.conditions
    }
    /// The explicit deactivation, if declared.
    #[must_use]
    pub const fn deactivated(&self) -> Option<bool> {
        self.deactivated
    }
    /// Whether the rule is explicitly deactivated with `true`.
    #[must_use]
    pub const fn is_deactivated(&self) -> bool {
        matches!(self.deactivated, Some(true))
    }
    /// The predicates whose derived triples the rule expects.
    #[must_use]
    pub fn expected_predicates(&self) -> &[Iri<Text>] {
        &self.expected_predicates
    }
    /// The requested custom processors.
    #[must_use]
    pub fn processors(&self) -> &[RuleProcessor<Text>] {
        &self.processors
    }
}

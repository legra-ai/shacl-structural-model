//! Independent records; no graph-sized membership collection or graph resolution.

use borrow_or_share::Bos;
use iri_rfc3987::Iri;
use rdf_structural_model::Subject;

/// A named rule-set declaration.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Text: serde::Serialize",
        deserialize = "Text: serde::Deserialize<'de>, Iri<Text>: serde::Deserialize<'de>"
    )
)]
pub struct RuleSet<Text: Bos<str> + AsRef<str> = String> {
    id: Iri<Text>,
}
impl<Text: Bos<str> + AsRef<str>> RuleSet<Text> {
    /// Construct one record from already checked RDF identities.
    #[must_use]
    pub const fn new(id: Iri<Text>) -> Self {
        Self { id }
    }
    /// The rule-set IRI.
    #[must_use]
    pub const fn id(&self) -> &Iri<Text> {
        &self.id
    }
}

/// One sh:hasRule edge.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Text: serde::Serialize",
        deserialize = "Text: serde::Deserialize<'de>, Iri<Text>: serde::Deserialize<'de>"
    )
)]
pub struct RuleSetMembership<Text: Bos<str> + AsRef<str> = String> {
    rule_set: Iri<Text>,
    rule: Subject<Text>,
}
impl<Text: Bos<str> + AsRef<str>> RuleSetMembership<Text> {
    /// Construct one record from already checked RDF identities.
    #[must_use]
    pub const fn new(rule_set: Iri<Text>, rule: Subject<Text>) -> Self {
        Self { rule_set, rule }
    }
    /// The owning rule-set IRI.
    #[must_use]
    pub const fn rule_set(&self) -> &Iri<Text> {
        &self.rule_set
    }
    /// The member rule identity.
    #[must_use]
    pub const fn rule(&self) -> &Subject<Text> {
        &self.rule
    }
}

/// One sh:includesRuleSet edge, without closure resolution.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Text: serde::Serialize",
        deserialize = "Text: serde::Deserialize<'de>, Iri<Text>: serde::Deserialize<'de>"
    )
)]
pub struct RuleSetInclusion<Text: Bos<str> + AsRef<str> = String> {
    rule_set: Iri<Text>,
    included: Iri<Text>,
}
impl<Text: Bos<str> + AsRef<str>> RuleSetInclusion<Text> {
    /// Construct one record from already checked RDF identities.
    #[must_use]
    pub const fn new(rule_set: Iri<Text>, included: Iri<Text>) -> Self {
        Self { rule_set, included }
    }
    /// The including rule-set IRI.
    #[must_use]
    pub const fn rule_set(&self) -> &Iri<Text> {
        &self.rule_set
    }
    /// The included rule-set IRI.
    #[must_use]
    pub const fn included(&self) -> &Iri<Text> {
        &self.included
    }
}

/// One sh:rule association; global status requires graph-wide consumer analysis.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Text: serde::Serialize",
        deserialize = "Text: serde::Deserialize<'de>, Iri<Text>: serde::Deserialize<'de>"
    )
)]
pub struct ShapeRuleLink<Text: Bos<str> + AsRef<str> = String> {
    shape: Subject<Text>,
    rule: Subject<Text>,
}
impl<Text: Bos<str> + AsRef<str>> ShapeRuleLink<Text> {
    /// Construct one record from already checked RDF identities.
    #[must_use]
    pub const fn new(shape: Subject<Text>, rule: Subject<Text>) -> Self {
        Self { shape, rule }
    }
    /// The IRI or scoped blank-node shape.
    #[must_use]
    pub const fn shape(&self) -> &Subject<Text> {
        &self.shape
    }
    /// The rule identity.
    #[must_use]
    pub const fn rule(&self) -> &Subject<Text> {
        &self.rule
    }
}

/// Scheduling declarations only. Absence is retained; consumers apply defaults.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Text: serde::Serialize",
        deserialize = "Text: serde::Deserialize<'de>, Iri<Text>: serde::Deserialize<'de>"
    )
)]
pub struct RuleSchedule<Text: Bos<str> + AsRef<str> = String> {
    layer: Option<super::RuleNumber<Text>>,
    order: Option<super::RuleNumber<Text>>,
    run_once: Option<bool>,
}
impl<Text: Bos<str> + AsRef<str>> RuleSchedule<Text> {
    /// Construct declarations with all three properties absent.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            layer: None,
            order: None,
            run_once: None,
        }
    }
    /// Set the explicitly declared layer.
    #[must_use]
    pub fn with_layer(mut self, layer: super::RuleNumber<Text>) -> Self {
        self.layer = Some(layer);
        self
    }
    /// Set the explicitly declared order.
    #[must_use]
    pub fn with_order(mut self, order: super::RuleNumber<Text>) -> Self {
        self.order = Some(order);
        self
    }
    /// Retain explicit true or false, rather than substituting for absence.
    #[must_use]
    pub const fn with_run_once(mut self, run_once: bool) -> Self {
        self.run_once = Some(run_once);
        self
    }
    /// The declared layer, if present.
    #[must_use]
    pub const fn layer(&self) -> Option<&super::RuleNumber<Text>> {
        self.layer.as_ref()
    }
    /// The declared order, if present.
    #[must_use]
    pub const fn order(&self) -> Option<&super::RuleNumber<Text>> {
        self.order.as_ref()
    }
    /// The explicit run-once declaration, if present.
    #[must_use]
    pub const fn run_once(&self) -> Option<bool> {
        self.run_once
    }
}

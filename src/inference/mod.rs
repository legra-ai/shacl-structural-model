//! Checked declarations for the SHACL 1.2 Inference Rules draft.
//!
//! This edition is distinct from the crate's AF declarations. It contains no
//! scheduler, graph resolver, validation or execution. Membership, include,
//! link, typing and reifier records can be streamed individually; no
//! graph-sized aggregate is required.

mod construct;
mod error;
mod metadata;
mod number;
mod processor;
mod records;
mod reifier;
mod rule_type;
mod sparql_rule;
mod template;
mod triple;

pub use construct::ConstructQuery;
pub use error::{INFERENCE_DRAFT, InferenceModelError};
pub use metadata::RuleMetadata;
pub use number::RuleNumber;
pub use processor::RuleProcessor;
/// IRI or scoped blank-node identity, using the shared RDF subject model.
pub use rdf_structural_model::Subject as ResourceRef;
pub use records::{RuleSchedule, RuleSet, RuleSetInclusion, RuleSetMembership, ShapeRuleLink};
pub use reifier::{InferenceAnnotation, InferredTripleReifier};
pub use rule_type::{RuleType, RuleTyping};
pub use sparql_rule::{SparqlRuleDeclaration, TemplateRuleDeclaration};
pub use template::{SparqlRuleTemplate, TemplateParameter};
pub use triple::{RuleExpression, TripleRuleDeclaration};

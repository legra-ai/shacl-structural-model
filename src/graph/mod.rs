//! Graph-level declarations: graph roles, graph links, entailment and
//! incomplete data.
//!
//! These follow SHACL 1.2 Core, Working Draft 16 November 2026 (sections 1.4,
//! 6.1, 6.2, 6.5, 6.6 and 6.9.3.1) and SHACL 1.2 Inference Rules, Working Draft
//! 10 November 2026 (sections 5 and 6). Each record is one triple-sized
//! declaration that a consumer streams beside its shapes; the model resolves
//! no graph, import or entailment regime.

mod entailment;
mod incomplete;
mod link;
mod role;

pub use entailment::{Entailment, EntailmentDeclaration};
pub use incomplete::{IncompleteNode, IncompleteUriStart};
pub use link::{GraphLink, GraphLinkKind};
pub use role::{GraphRole, GraphTypeDeclaration};

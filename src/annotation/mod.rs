//! SHACL 1.2 Core non-validating shape characteristics (Core section 6.11).
//!
//! These declarations follow SHACL 1.2 Core, Working Draft 16 November 2026.
//! SHACL processors ignore them during validation; the model retains them for
//! form builders, code generators and natural-language agents.

mod code_identifier;
mod intent;
mod property_group;
mod shape_annotations;

pub use code_identifier::CodeIdentifier;
pub use intent::Intent;
pub use property_group::PropertyGroup;
pub use shape_annotations::ShapeAnnotations;

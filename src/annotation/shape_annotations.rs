//! The non-validating characteristics shared by node and property shapes.

use iri_rfc3987::Iri;

use super::{CodeIdentifier, Intent};

/// `sh:intent`, `sh:agentInstruction`, `sh:codeIdentifier`, `sh:codeType` and
/// `sh:unit` declarations of one shape.
///
/// SHACL 1.2 Core, Working Draft 16 November 2026, sections 6.11.2 to 6.11.6.
/// Literal values are caller-owned RDF terms that keep language tags and
/// directions; `units` may also hold IRIs. The consumer checks datatypes and
/// cardinality of the source triples.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Value: serde::Serialize, Named: serde::Serialize",
        deserialize = "Value: serde::Deserialize<'de>, Named: serde::Deserialize<'de>"
    )
)]
pub struct ShapeAnnotations<Value = crate::DefaultTerm, Named = Iri<String>> {
    /// `sh:intent` statements (6.11.2), each with its `sh:formalized` reifier.
    pub intents: Vec<Intent<Value>>,
    /// `sh:agentInstruction` literals (6.11.3).
    pub agent_instructions: Vec<Value>,
    /// The `sh:codeIdentifier` name (6.11.4).
    pub code_identifier: Option<CodeIdentifier>,
    /// The `sh:codeType` datatype IRI (6.11.5).
    pub code_type: Option<Named>,
    /// `sh:unit` literals or IRIs (6.11.6).
    pub units: Vec<Value>,
}

impl<Value, Named> ShapeAnnotations<Value, Named> {
    /// Annotations with every characteristic absent.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            intents: Vec::new(),
            agent_instructions: Vec::new(),
            code_identifier: None,
            code_type: None,
            units: Vec::new(),
        }
    }

    /// Add an `sh:intent` statement.
    #[must_use]
    pub fn with_intent(mut self, intent: Intent<Value>) -> Self {
        self.intents.push(intent);
        self
    }

    /// Add an `sh:agentInstruction` literal.
    #[must_use]
    pub fn with_agent_instruction(mut self, instruction: Value) -> Self {
        self.agent_instructions.push(instruction);
        self
    }

    /// Set the `sh:codeIdentifier`.
    #[must_use]
    pub fn with_code_identifier(mut self, identifier: CodeIdentifier) -> Self {
        self.code_identifier = Some(identifier);
        self
    }

    /// Set the `sh:codeType` datatype.
    #[must_use]
    pub fn with_code_type(mut self, datatype: Named) -> Self {
        self.code_type = Some(datatype);
        self
    }

    /// Add an `sh:unit` value.
    #[must_use]
    pub fn with_unit(mut self, unit: Value) -> Self {
        self.units.push(unit);
        self
    }

    /// Whether no characteristic is declared.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.intents.is_empty()
            && self.agent_instructions.is_empty()
            && self.code_identifier.is_none()
            && self.code_type.is_none()
            && self.units.is_empty()
    }
}

impl<Value, Named> Default for ShapeAnnotations<Value, Named> {
    fn default() -> Self {
        Self::new()
    }
}

//! Checked `sh:construct` queries with their `sh:prefixes` references.

use borrow_or_share::Bos;
use iri_rfc3987::Iri;
use rdf_structural_model::{Literal, Subject};
use serde::{Deserialize, Deserializer};

use super::InferenceModelError;

const XSD_STRING: &str = "http://www.w3.org/2001/XMLSchema#string";

/// A SPARQL CONSTRUCT declaration of a SPARQL rule or rule template.
///
/// Inference Rules sections 11.1 and 12: the one `sh:construct` value is an
/// `xsd:string` literal, and `sh:prefixes` references prefix declarations.
/// The query text is not parsed here.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
#[serde(bound(serialize = "Text: serde::Serialize"))]
pub struct ConstructQuery<Text: Bos<str> + AsRef<str> = String> {
    construct: Literal<Text>,
    prefixes: Vec<Subject<Text>>,
}

impl<Text: Bos<str> + AsRef<str>> ConstructQuery<Text> {
    /// Check that `construct` is an `xsd:string` literal.
    ///
    /// # Errors
    /// Returns [`InferenceModelError::InvalidConstructQuery`] for another datatype.
    pub fn try_new(construct: Literal<Text>) -> Result<Self, InferenceModelError> {
        if construct.metadata().datatype().as_str() == XSD_STRING {
            Ok(Self {
                construct,
                prefixes: Vec::new(),
            })
        } else {
            Err(InferenceModelError::InvalidConstructQuery)
        }
    }
    /// Add a `sh:prefixes` reference.
    #[must_use]
    pub fn with_prefixes(mut self, prefixes: Subject<Text>) -> Self {
        self.prefixes.push(prefixes);
        self
    }
    /// The query literal.
    #[must_use]
    pub const fn construct(&self) -> &Literal<Text> {
        &self.construct
    }
    /// The query text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.construct.lexical()
    }
    /// The prefix-declaration references.
    #[must_use]
    pub fn prefixes(&self) -> &[Subject<Text>] {
        &self.prefixes
    }
}

#[derive(Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(deserialize = "Text: Deserialize<'de>, Iri<Text>: Deserialize<'de>")
)]
struct Wire<Text: Bos<str> + AsRef<str>> {
    construct: Literal<Text>,
    prefixes: Vec<Subject<Text>>,
}

impl<'de, Text> Deserialize<'de> for ConstructQuery<Text>
where
    Text: Bos<str> + AsRef<str> + Deserialize<'de>,
    Iri<Text>: Deserialize<'de>,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let wire = Wire::deserialize(deserializer)?;
        let mut query = Self::try_new(wire.construct).map_err(serde::de::Error::custom)?;
        query.prefixes = wire.prefixes;
        Ok(query)
    }
}

//! Reifier annotations on inferred triples (Inference Rules sections 7 and 9).

use borrow_or_share::Bos;
use iri_rfc3987::Iri;
use rdf_structural_model::{Subject, Triple};

/// What a reifier in the inference graph says about its inferred triple.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(bound(
    serialize = "Text: serde::Serialize",
    deserialize = "Text: serde::Deserialize<'de>, Iri<Text>: serde::Deserialize<'de>"
))]
pub enum InferenceAnnotation<Text: Bos<str> + AsRef<str> = String> {
    /// `sh:tempTriple` (section 7); `true` marks a temporary triple that the
    /// engine deletes at the end of the inference process.
    TempTriple(bool),
    /// `sh:sourceRule` (section 9): the rule that produced the triple.
    SourceRule(Subject<Text>),
}

/// One reifier triple `reifier annotation` with its `rdf:reifies` triple.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Text: serde::Serialize",
        deserialize = "Text: serde::Deserialize<'de>, Iri<Text>: serde::Deserialize<'de>"
    )
)]
pub struct InferredTripleReifier<Text: Bos<str> + AsRef<str> = String> {
    reifier: Subject<Text>,
    triple: Triple<Text>,
    annotation: InferenceAnnotation<Text>,
}

impl<Text: Bos<str> + AsRef<str>> InferredTripleReifier<Text> {
    /// Annotate `triple` through `reifier`.
    #[must_use]
    pub const fn new(
        reifier: Subject<Text>,
        triple: Triple<Text>,
        annotation: InferenceAnnotation<Text>,
    ) -> Self {
        Self {
            reifier,
            triple,
            annotation,
        }
    }
    /// The reifier node.
    #[must_use]
    pub const fn reifier(&self) -> &Subject<Text> {
        &self.reifier
    }
    /// The reified inferred triple.
    #[must_use]
    pub const fn triple(&self) -> &Triple<Text> {
        &self.triple
    }
    /// The annotation.
    #[must_use]
    pub const fn annotation(&self) -> &InferenceAnnotation<Text> {
        &self.annotation
    }
    /// Whether the annotation marks a temporary triple.
    #[must_use]
    pub const fn is_temporary(&self) -> bool {
        matches!(self.annotation, InferenceAnnotation::TempTriple(true))
    }
}

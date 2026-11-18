//! `sh:SPARQLRuleTemplate` declarations (Inference Rules section 12).

use borrow_or_share::Bos;
use iri_rfc3987::Iri;
use rdf_structural_model::Term;

use super::ConstructQuery;
use crate::Parameter;

/// A template parameter, declared as for SPARQL-based constraint components.
pub type TemplateParameter<Text = String> =
    Parameter<Term<Text>, Term<Text>, Term<Text>, Term<Text>, Iri<Text>>;

/// A SPARQL rule template: a rule type whose instances are rules.
///
/// The template IRI is the rule type of its instances
/// ([`super::TemplateRuleDeclaration`]). Its parameters map to pre-bound
/// variables of the CONSTRUCT query.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Text: serde::Serialize",
        deserialize = "Text: serde::Deserialize<'de>, Iri<Text>: serde::Deserialize<'de>"
    )
)]
pub struct SparqlRuleTemplate<Text: Bos<str> + AsRef<str> = String> {
    id: Iri<Text>,
    parameters: Vec<TemplateParameter<Text>>,
    query: ConstructQuery<Text>,
}

impl<Text: Bos<str> + AsRef<str>> SparqlRuleTemplate<Text> {
    /// A template with its CONSTRUCT query and no parameters.
    #[must_use]
    pub const fn new(id: Iri<Text>, query: ConstructQuery<Text>) -> Self {
        Self {
            id,
            parameters: Vec::new(),
            query,
        }
    }
    /// Add a `sh:parameter` declaration.
    #[must_use]
    pub fn with_parameter(mut self, parameter: TemplateParameter<Text>) -> Self {
        self.parameters.push(parameter);
        self
    }
    /// The template IRI.
    #[must_use]
    pub const fn id(&self) -> &Iri<Text> {
        &self.id
    }
    /// The parameter declarations.
    #[must_use]
    pub fn parameters(&self) -> &[TemplateParameter<Text>] {
        &self.parameters
    }
    /// Parameters an instance must supply (not `sh:optional true`).
    pub fn required_parameters(&self) -> impl Iterator<Item = &TemplateParameter<Text>> {
        self.parameters
            .iter()
            .filter(|parameter| !parameter.optional)
    }
    /// The CONSTRUCT query.
    #[must_use]
    pub const fn query(&self) -> &ConstructQuery<Text> {
        &self.query
    }
}

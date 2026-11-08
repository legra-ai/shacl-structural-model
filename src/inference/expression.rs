//! Restricted 1.2 expressions, distinct from AF node expressions.

use borrow_or_share::Bos;
use iri_rfc3987::Iri;
use rdf_structural_model::Term;
use serde::{Deserialize, Deserializer};

use super::{DecodeLimits, InferenceModelError, decode};
use crate::ShaclPath;

/// One direct constant or path-values declaration, never an evaluated sequence.
///
/// Explicit `shnex:focusNode` expressions are unsupported in this slice and fail
/// rather than being discarded. Omission means the consumer's current focus.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
#[serde(transparent, bound(serialize = "Text: serde::Serialize"))]
pub struct DirectExpression<Text: Bos<str> + AsRef<str> = String>(ExpressionInput<Text>);

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "Text: serde::Serialize",
        deserialize = "Text: serde::Deserialize<'de>, Iri<Text>: serde::Deserialize<'de>"
    )
)]
enum ExpressionInput<Text: Bos<str> + AsRef<str>> {
    Constant(Term<Text>),
    PathValues {
        path: ShaclPath<Iri<Text>>,
        focus_node: Option<Term<Text>>,
    },
}

impl<Text: Bos<str> + AsRef<str>> DirectExpression<Text> {
    /// Construct an IRI, literal or triple-term constant.
    ///
    /// # Errors
    /// Bare blank nodes are expressions, not constants. Structural depth above
    /// 64 or more than 4096 term/path nodes is rejected before further traversal.
    pub fn try_constant(term: Term<Text>) -> Result<Self, InferenceModelError> {
        if matches!(term, Term::BlankNode(_)) {
            return Err(InferenceModelError::BlankConstant);
        }
        let mut nodes = 4096;
        check_term(&term, 0, &mut nodes)?;
        Ok(Self(ExpressionInput::Constant(term)))
    }

    /// Construct a path-values expression with its exact shared property path.
    ///
    /// # Errors
    /// Explicit focus expressions, malformed sequence/alternative arity, depth
    /// above 64 and more than 4096 nodes return typed errors. The optional
    /// argument makes unsupported encountered focus input impossible to ignore.
    pub fn try_path_values(
        path: ShaclPath<Iri<Text>>,
        focus_node: Option<&Term<Text>>,
    ) -> Result<Self, InferenceModelError> {
        if focus_node.is_some() {
            return Err(InferenceModelError::UnsupportedFocusExpression);
        }
        let mut nodes = 4096;
        check_path(&path, 0, &mut nodes)?;
        Ok(Self(ExpressionInput::PathValues {
            path,
            focus_node: None,
        }))
    }

    /// The exact constant term, if this is a constant expression.
    #[must_use]
    pub const fn constant(&self) -> Option<&Term<Text>> {
        match &self.0 {
            ExpressionInput::Constant(term) => Some(term),
            ExpressionInput::PathValues { .. } => None,
        }
    }
    /// The `shnex:pathValues` parameter, if present.
    #[must_use]
    pub const fn path(&self) -> Option<&ShaclPath<Iri<Text>>> {
        match &self.0 {
            ExpressionInput::Constant(_) => None,
            ExpressionInput::PathValues { path, .. } => Some(path),
        }
    }

    /// Decode with explicit wire limits, while retaining all constructor checks.
    ///
    /// # Errors
    /// Returns the format's error for exceeded budgets or invalid/unsupported
    /// declarations. Also cap the input source bytes before invoking a decoder.
    pub fn deserialize_with_limits<'de, D>(
        deserializer: D,
        limits: DecodeLimits,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
        Text: Deserialize<'de>,
        Iri<Text>: Deserialize<'de>,
    {
        let input = decode::decode::<ExpressionInput<Text>, D>(deserializer, limits)?;
        match input {
            ExpressionInput::Constant(term) => Self::try_constant(term),
            ExpressionInput::PathValues { path, focus_node } => {
                Self::try_path_values(path, focus_node.as_ref())
            }
        }
        .map_err(serde::de::Error::custom)
    }
}
impl<'de, Text> Deserialize<'de> for DirectExpression<Text>
where
    Text: Bos<str> + AsRef<str> + Deserialize<'de>,
    Iri<Text>: Deserialize<'de>,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::deserialize_with_limits(deserializer, DecodeLimits::default())
    }
}
fn visit(depth: usize, nodes: &mut usize) -> Result<(), InferenceModelError> {
    if depth >= 64 || *nodes == 0 {
        return Err(InferenceModelError::ExpressionBudgetExceeded);
    }
    *nodes -= 1;
    Ok(())
}
fn check_term<Text: Bos<str> + AsRef<str>>(
    term: &Term<Text>,
    depth: usize,
    nodes: &mut usize,
) -> Result<(), InferenceModelError> {
    visit(depth, nodes)?;
    match term {
        Term::Triple(triple) => check_term(triple.object(), depth + 1, nodes),
        Term::Iri(_) | Term::Literal(_) | Term::BlankNode(_) => Ok(()),
        Term::Extension(never) => match *never {},
    }
}
fn check_path<Text: Bos<str> + AsRef<str>>(
    path: &ShaclPath<Iri<Text>>,
    depth: usize,
    nodes: &mut usize,
) -> Result<(), InferenceModelError> {
    visit(depth, nodes)?;
    match path {
        ShaclPath::Sequence(paths) | ShaclPath::Alternative(paths) => {
            if paths.len() < 2 {
                return Err(InferenceModelError::InvalidPathArity);
            }
            for path in paths {
                check_path(path, depth + 1, nodes)?;
            }
            Ok(())
        }
        ShaclPath::Predicate(_) => Ok(()),
        ShaclPath::Inverse(path)
        | ShaclPath::ZeroOrMore(path)
        | ShaclPath::OneOrMore(path)
        | ShaclPath::ZeroOrOne(path) => check_path(path, depth + 1, nodes),
    }
}

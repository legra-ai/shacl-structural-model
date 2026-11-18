//! Budgeted decoding: an unchecked wire tree, checked bottom-up after decoding.

use serde::{Deserialize, Deserializer};

use super::{ExpressionError, ExpressionForm, NodeExpression};
use crate::DecodeLimits;

/// Nested expression types that may be decoded as part of an [`ExpressionForm`].
///
/// Sealed: only the crate's budgeted wire tree implements it, so a public form
/// cannot be decoded without the root's [`DecodeLimits`] and checks.
pub trait DecodedChild<'de>: Deserialize<'de> + sealed::Sealed {}

mod sealed {
    pub trait Sealed {}
}

/// The serialized shape of [`NodeExpression`], before its constructor checks.
#[derive(Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(deserialize = "Value: Deserialize<'de>, Named: Deserialize<'de>")
)]
struct WireExpression<Value, Named> {
    source: Option<Value>,
    form: ExpressionForm<Value, Named, Box<WireExpression<Value, Named>>>,
}

impl<Value, Named> sealed::Sealed for Box<WireExpression<Value, Named>> {}
impl<'de, Value: Deserialize<'de>, Named: Deserialize<'de>> DecodedChild<'de>
    for Box<WireExpression<Value, Named>>
{
}

impl<Value, Named> WireExpression<Value, Named> {
    /// Check the tree bottom-up; recursion is bounded by the decode depth.
    fn into_checked(self) -> Result<NodeExpression<Value, Named>, ExpressionError> {
        let form = self
            .form
            .try_map_children(|child| child.into_checked().map(Box::new))?;
        let expression = NodeExpression::try_new(form)?;
        Ok(match self.source {
            Some(source) => expression.with_source(source),
            None => expression,
        })
    }
}

impl<Value, Named> NodeExpression<Value, Named> {
    /// Decode with explicit wire limits, then apply every constructor check.
    ///
    /// # Errors
    /// Returns the format's error for exceeded budgets, unknown fields and
    /// structurally invalid expressions. Also cap the source bytes first.
    pub fn deserialize_with_limits<'de, D>(
        deserializer: D,
        limits: DecodeLimits,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
        Value: Deserialize<'de>,
        Named: Deserialize<'de>,
    {
        crate::decode::decode::<WireExpression<Value, Named>, D>(deserializer, limits)?
            .into_checked()
            .map_err(serde::de::Error::custom)
    }
}

impl<'de, Value: Deserialize<'de>, Named: Deserialize<'de>> Deserialize<'de>
    for NodeExpression<Value, Named>
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::deserialize_with_limits(deserializer, DecodeLimits::default())
    }
}

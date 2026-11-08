//! Format-neutral input budgets applied before nested visitor allocation.

use serde::de::{self, DeserializeSeed};
mod deserializer;
mod visitor;
use serde::{Deserialize, Deserializer};

/// Limits for one expression's serialized input, independent of graph size.
///
/// Items count deserializer calls; depth counts nested wire values, not RDF
/// triples. Text bytes include enum tags, field names and literal/IRI content.
/// A caller must additionally cap the source buffer; a deserializer may own or
/// allocate input text before handing it to a Serde visitor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodeLimits {
    depth: usize,
    items: usize,
    text_bytes: usize,
}
impl DecodeLimits {
    /// Set explicit wire depth, item and total text-byte limits.
    #[must_use]
    pub const fn new(depth: usize, items: usize, text_bytes: usize) -> Self {
        Self {
            depth,
            items,
            text_bytes,
        }
    }
}
impl Default for DecodeLimits {
    fn default() -> Self {
        Self::new(128, 32_768, 1_048_576)
    }
}

pub(super) fn decode<'de, T: Deserialize<'de>, D: Deserializer<'de>>(
    d: D,
    limits: DecodeLimits,
) -> Result<T, D::Error> {
    let mut limits = limits;
    T::deserialize(Bounded {
        inner: d,
        limits: &mut limits,
        depth: 0,
    })
}

struct Bounded<'a, D> {
    inner: D,
    limits: &'a mut DecodeLimits,
    depth: usize,
}
impl<D> Bounded<'_, D> {
    fn enter<E: de::Error>(&mut self) -> Result<(), E> {
        if self.depth >= self.limits.depth || self.limits.items == 0 {
            return Err(E::custom("inference expression decode budget exceeded"));
        }
        self.limits.items -= 1;
        Ok(())
    }
}
struct BoundedVisitor<'a, V> {
    inner: V,
    limits: &'a mut DecodeLimits,
    depth: usize,
}
struct Seed<'a, S> {
    inner: S,
    limits: &'a mut DecodeLimits,
    depth: usize,
}
impl<'de, S: DeserializeSeed<'de>> DeserializeSeed<'de> for Seed<'_, S> {
    type Value = S::Value;
    fn deserialize<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        self.inner.deserialize(Bounded {
            inner: d,
            limits: self.limits,
            depth: self.depth,
        })
    }
}

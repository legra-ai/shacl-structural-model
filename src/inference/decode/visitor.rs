//! Budgeted nested visitors and collection access.
use super::{Bounded, BoundedVisitor, DecodeLimits, Seed};
use serde::{
    Deserializer,
    de::{self, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor},
};

impl<'de, V: Visitor<'de>> Visitor<'de> for BoundedVisitor<'_, V> {
    type Value = V::Value;
    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.expecting(f)
    }
    fn visit_bool<E: de::Error>(self, value: bool) -> Result<Self::Value, E> {
        self.inner.visit_bool(value)
    }
    fn visit_i8<E: de::Error>(self, value: i8) -> Result<Self::Value, E> {
        self.inner.visit_i8(value)
    }
    fn visit_i16<E: de::Error>(self, value: i16) -> Result<Self::Value, E> {
        self.inner.visit_i16(value)
    }
    fn visit_i32<E: de::Error>(self, value: i32) -> Result<Self::Value, E> {
        self.inner.visit_i32(value)
    }
    fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E> {
        self.inner.visit_i64(value)
    }
    fn visit_i128<E: de::Error>(self, value: i128) -> Result<Self::Value, E> {
        self.inner.visit_i128(value)
    }
    fn visit_u8<E: de::Error>(self, value: u8) -> Result<Self::Value, E> {
        self.inner.visit_u8(value)
    }
    fn visit_u16<E: de::Error>(self, value: u16) -> Result<Self::Value, E> {
        self.inner.visit_u16(value)
    }
    fn visit_u32<E: de::Error>(self, value: u32) -> Result<Self::Value, E> {
        self.inner.visit_u32(value)
    }
    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
        self.inner.visit_u64(value)
    }
    fn visit_u128<E: de::Error>(self, value: u128) -> Result<Self::Value, E> {
        self.inner.visit_u128(value)
    }
    fn visit_f32<E: de::Error>(self, value: f32) -> Result<Self::Value, E> {
        self.inner.visit_f32(value)
    }
    fn visit_f64<E: de::Error>(self, value: f64) -> Result<Self::Value, E> {
        self.inner.visit_f64(value)
    }
    fn visit_char<E: de::Error>(self, value: char) -> Result<Self::Value, E> {
        self.inner.visit_char(value)
    }
    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        self.limits.text_bytes = self
            .limits
            .text_bytes
            .checked_sub(value.len())
            .ok_or_else(|| E::custom("inference expression text budget exceeded"))?;
        self.inner.visit_str(value)
    }
    fn visit_borrowed_str<E: de::Error>(self, value: &'de str) -> Result<Self::Value, E> {
        self.limits.text_bytes = self
            .limits
            .text_bytes
            .checked_sub(value.len())
            .ok_or_else(|| E::custom("inference expression text budget exceeded"))?;
        self.inner.visit_borrowed_str(value)
    }
    fn visit_string<E: de::Error>(self, value: String) -> Result<Self::Value, E> {
        self.limits.text_bytes = self
            .limits
            .text_bytes
            .checked_sub(value.len())
            .ok_or_else(|| E::custom("inference expression text budget exceeded"))?;
        self.inner.visit_string(value)
    }
    fn visit_bytes<E: de::Error>(self, value: &[u8]) -> Result<Self::Value, E> {
        self.limits.text_bytes = self
            .limits
            .text_bytes
            .checked_sub(value.len())
            .ok_or_else(|| E::custom("inference expression text budget exceeded"))?;
        self.inner.visit_bytes(value)
    }
    fn visit_borrowed_bytes<E: de::Error>(self, value: &'de [u8]) -> Result<Self::Value, E> {
        self.limits.text_bytes = self
            .limits
            .text_bytes
            .checked_sub(value.len())
            .ok_or_else(|| E::custom("inference expression text budget exceeded"))?;
        self.inner.visit_borrowed_bytes(value)
    }
    fn visit_byte_buf<E: de::Error>(self, value: Vec<u8>) -> Result<Self::Value, E> {
        self.limits.text_bytes = self
            .limits
            .text_bytes
            .checked_sub(value.len())
            .ok_or_else(|| E::custom("inference expression text budget exceeded"))?;
        self.inner.visit_byte_buf(value)
    }
    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        self.inner.visit_none()
    }
    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
        self.inner.visit_unit()
    }
    fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        self.inner.visit_some(Bounded {
            inner: d,
            limits: self.limits,
            depth: self.depth,
        })
    }
    fn visit_newtype_struct<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        self.inner.visit_newtype_struct(Bounded {
            inner: d,
            limits: self.limits,
            depth: self.depth,
        })
    }
    fn visit_seq<A: SeqAccess<'de>>(self, a: A) -> Result<Self::Value, A::Error> {
        self.inner.visit_seq(Access {
            inner: a,
            limits: self.limits,
            depth: self.depth,
        })
    }
    fn visit_map<A: MapAccess<'de>>(self, a: A) -> Result<Self::Value, A::Error> {
        self.inner.visit_map(Access {
            inner: a,
            limits: self.limits,
            depth: self.depth,
        })
    }
    fn visit_enum<A: EnumAccess<'de>>(self, a: A) -> Result<Self::Value, A::Error> {
        self.inner.visit_enum(Access {
            inner: a,
            limits: self.limits,
            depth: self.depth,
        })
    }
}
struct Access<'a, A> {
    inner: A,
    limits: &'a mut DecodeLimits,
    depth: usize,
}
impl<'de, A: SeqAccess<'de>> SeqAccess<'de> for Access<'_, A> {
    type Error = A::Error;
    fn next_element_seed<T: DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, A::Error> {
        self.inner.next_element_seed(Seed {
            inner: seed,
            limits: self.limits,
            depth: self.depth,
        })
    }
    // Do not trust attacker-controlled lengths for preallocation.
    fn size_hint(&self) -> Option<usize> {
        Some(0)
    }
}
impl<'de, A: MapAccess<'de>> MapAccess<'de> for Access<'_, A> {
    type Error = A::Error;
    fn next_key_seed<T: DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, A::Error> {
        self.inner.next_key_seed(Seed {
            inner: seed,
            limits: self.limits,
            depth: self.depth,
        })
    }
    fn next_value_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<T::Value, A::Error> {
        self.inner.next_value_seed(Seed {
            inner: seed,
            limits: self.limits,
            depth: self.depth,
        })
    }
    fn size_hint(&self) -> Option<usize> {
        Some(0)
    }
}
impl<'a, 'de, A: EnumAccess<'de>> EnumAccess<'de> for Access<'a, A> {
    type Error = A::Error;
    type Variant = Access<'a, A::Variant>;
    fn variant_seed<T: DeserializeSeed<'de>>(
        self,
        seed: T,
    ) -> Result<(T::Value, Self::Variant), A::Error> {
        let (value, variant) = self.inner.variant_seed(Seed {
            inner: seed,
            limits: self.limits,
            depth: self.depth,
        })?;
        Ok((
            value,
            Access {
                inner: variant,
                limits: self.limits,
                depth: self.depth,
            },
        ))
    }
}
impl<'de, A: VariantAccess<'de>> VariantAccess<'de> for Access<'_, A> {
    type Error = A::Error;
    fn unit_variant(self) -> Result<(), A::Error> {
        self.inner.unit_variant()
    }
    fn newtype_variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value, A::Error> {
        self.inner.newtype_variant_seed(Seed {
            inner: seed,
            limits: self.limits,
            depth: self.depth,
        })
    }
    fn tuple_variant<V: Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value, A::Error> {
        self.inner.tuple_variant(
            len,
            BoundedVisitor {
                inner: visitor,
                limits: self.limits,
                depth: self.depth,
            },
        )
    }
    fn struct_variant<V: Visitor<'de>>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, A::Error> {
        self.inner.struct_variant(
            fields,
            BoundedVisitor {
                inner: visitor,
                limits: self.limits,
                depth: self.depth,
            },
        )
    }
}

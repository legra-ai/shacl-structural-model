//! Budgeted deserializer forwarding.
use super::{Bounded, BoundedVisitor};
use serde::{Deserializer, de::Visitor};

impl<'de, D: Deserializer<'de>> Deserializer<'de> for Bounded<'_, D> {
    type Error = D::Error;
    fn deserialize_any<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_any(BoundedVisitor {
            inner: visitor,
            limits: self.limits,
            depth: self.depth + 1,
        })
    }
    fn deserialize_bool<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_bool(BoundedVisitor {
            inner: visitor,
            limits: self.limits,
            depth: self.depth + 1,
        })
    }
    fn deserialize_i8<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_i8(BoundedVisitor {
            inner: visitor,
            limits: self.limits,
            depth: self.depth + 1,
        })
    }
    fn deserialize_i16<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_i16(BoundedVisitor {
            inner: visitor,
            limits: self.limits,
            depth: self.depth + 1,
        })
    }
    fn deserialize_i32<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_i32(BoundedVisitor {
            inner: visitor,
            limits: self.limits,
            depth: self.depth + 1,
        })
    }
    fn deserialize_i64<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_i64(BoundedVisitor {
            inner: visitor,
            limits: self.limits,
            depth: self.depth + 1,
        })
    }
    fn deserialize_i128<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_i128(BoundedVisitor {
            inner: visitor,
            limits: self.limits,
            depth: self.depth + 1,
        })
    }
    fn deserialize_u8<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_u8(BoundedVisitor {
            inner: visitor,
            limits: self.limits,
            depth: self.depth + 1,
        })
    }
    fn deserialize_u16<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_u16(BoundedVisitor {
            inner: visitor,
            limits: self.limits,
            depth: self.depth + 1,
        })
    }
    fn deserialize_u32<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_u32(BoundedVisitor {
            inner: visitor,
            limits: self.limits,
            depth: self.depth + 1,
        })
    }
    fn deserialize_u64<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_u64(BoundedVisitor {
            inner: visitor,
            limits: self.limits,
            depth: self.depth + 1,
        })
    }
    fn deserialize_u128<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_u128(BoundedVisitor {
            inner: visitor,
            limits: self.limits,
            depth: self.depth + 1,
        })
    }
    fn deserialize_f32<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_f32(BoundedVisitor {
            inner: visitor,
            limits: self.limits,
            depth: self.depth + 1,
        })
    }
    fn deserialize_f64<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_f64(BoundedVisitor {
            inner: visitor,
            limits: self.limits,
            depth: self.depth + 1,
        })
    }
    fn deserialize_char<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_char(BoundedVisitor {
            inner: visitor,
            limits: self.limits,
            depth: self.depth + 1,
        })
    }
    fn deserialize_str<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_str(BoundedVisitor {
            inner: visitor,
            limits: self.limits,
            depth: self.depth + 1,
        })
    }
    fn deserialize_string<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_string(BoundedVisitor {
            inner: visitor,
            limits: self.limits,
            depth: self.depth + 1,
        })
    }
    fn deserialize_bytes<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_bytes(BoundedVisitor {
            inner: visitor,
            limits: self.limits,
            depth: self.depth + 1,
        })
    }
    fn deserialize_byte_buf<V: Visitor<'de>>(
        mut self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_byte_buf(BoundedVisitor {
            inner: visitor,
            limits: self.limits,
            depth: self.depth + 1,
        })
    }
    fn deserialize_option<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_option(BoundedVisitor {
            inner: visitor,
            limits: self.limits,
            depth: self.depth + 1,
        })
    }
    fn deserialize_unit<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_unit(BoundedVisitor {
            inner: visitor,
            limits: self.limits,
            depth: self.depth + 1,
        })
    }
    fn deserialize_seq<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_seq(BoundedVisitor {
            inner: visitor,
            limits: self.limits,
            depth: self.depth + 1,
        })
    }
    fn deserialize_map<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_map(BoundedVisitor {
            inner: visitor,
            limits: self.limits,
            depth: self.depth + 1,
        })
    }
    fn deserialize_identifier<V: Visitor<'de>>(
        mut self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_identifier(BoundedVisitor {
            inner: visitor,
            limits: self.limits,
            depth: self.depth + 1,
        })
    }
    fn deserialize_ignored_any<V: Visitor<'de>>(
        mut self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_ignored_any(BoundedVisitor {
            inner: visitor,
            limits: self.limits,
            depth: self.depth + 1,
        })
    }
    fn deserialize_unit_struct<V: Visitor<'de>>(
        mut self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_unit_struct(
            name,
            BoundedVisitor {
                inner: visitor,
                limits: self.limits,
                depth: self.depth + 1,
            },
        )
    }
    fn deserialize_newtype_struct<V: Visitor<'de>>(
        mut self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_newtype_struct(
            name,
            BoundedVisitor {
                inner: visitor,
                limits: self.limits,
                depth: self.depth + 1,
            },
        )
    }
    fn deserialize_tuple<V: Visitor<'de>>(
        mut self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_tuple(
            len,
            BoundedVisitor {
                inner: visitor,
                limits: self.limits,
                depth: self.depth + 1,
            },
        )
    }
    fn deserialize_tuple_struct<V: Visitor<'de>>(
        mut self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_tuple_struct(
            name,
            len,
            BoundedVisitor {
                inner: visitor,
                limits: self.limits,
                depth: self.depth + 1,
            },
        )
    }
    fn deserialize_struct<V: Visitor<'de>>(
        mut self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_struct(
            name,
            fields,
            BoundedVisitor {
                inner: visitor,
                limits: self.limits,
                depth: self.depth + 1,
            },
        )
    }
    fn deserialize_enum<V: Visitor<'de>>(
        mut self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.enter()?;
        self.inner.deserialize_enum(
            name,
            variants,
            BoundedVisitor {
                inner: visitor,
                limits: self.limits,
                depth: self.depth + 1,
            },
        )
    }
    fn is_human_readable(&self) -> bool {
        self.inner.is_human_readable()
    }
}

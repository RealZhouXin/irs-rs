use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
pub struct BinarySerializeError(String);

impl std::error::Error for BinarySerializeError {}

impl fmt::Display for BinarySerializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Serialization error: {}", self.0)
    }
}

impl serde::ser::Error for BinarySerializeError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        BinarySerializeError(msg.to_string())
    }
}

impl serde::de::Error for BinarySerializeError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        BinarySerializeError(msg.to_string())
    }
}

pub fn serialize<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, BinarySerializeError> {
    let mut ser = BinarySerializer::new();
    value.serialize(&mut ser)?;
    Ok(ser.into_bytes())
}
pub fn deserialize<T>(bytes: &[u8]) -> Result<T, BinarySerializeError>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let mut de = BinaryDeserializer::new(bytes);
    T::deserialize(&mut de)
}

// 序列化器
pub struct BinarySerializer {
    output: Vec<u8>,
}

// 反序列化器
pub struct BinaryDeserializer<'a> {
    input: &'a [u8],
    pos: usize,
}

impl BinarySerializer {
    pub fn new() -> Self {
        Self { output: Vec::new() }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.output
    }

    fn write_u8(&mut self, value: u8) {
        self.output.push(value);
    }

    fn write_u32(&mut self, value: u32) {
        self.output.extend_from_slice(&value.to_le_bytes());
    }

    fn write_string(&mut self, s: &str) {
        self.write_u32(s.len() as u32);
        self.output.extend_from_slice(s.as_bytes());
    }
}

impl<'a> serde::Serializer for &'a mut BinarySerializer {
    type Ok = ();
    type Error = BinarySerializeError;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8(0)?;
        Ok(())
    }
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        if let Some(len) = len {
            self.write_u32(len as u32);
        } else {
            return Err(BinarySerializeError(String::from("序列长度必须已知")));
        }
        Ok(self)
    }
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_u8(1)?;
        value.serialize(self)
    }
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.write_u8(if v { 1 } else { 0 });
        Ok(())
    }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.write_u8(v);
        Ok(())
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.write_u8(v as u8);
        Ok(())
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(v);
        Ok(())
    }
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.write_u8(v as u8);
        Ok(())
    }
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.write_string(v);
        Ok(())
    }
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        // self.serialize_u8(4)?;
        Ok(self)
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_u8(1)?;
        Ok(self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.serialize_u32(len.unwrap_or(0) as u32)?;
        Ok(self)
    }
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(variant_index)?;
        Ok(())
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_u32(variant_index)?;
        Ok(self)
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.serialize_u32(variant_index)?;
        Ok(self)
    }
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_u32(variant_index)?;
        value.serialize(self)
    }
}

impl<'a> serde::ser::SerializeSeq for &'a mut BinarySerializer {
    type Ok = ();
    type Error = BinarySerializeError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        // 将元素用相同的序列化器序列化
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> serde::ser::SerializeTuple for &'a mut BinarySerializer {
    type Ok = ();
    type Error = BinarySerializeError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> serde::ser::SerializeTupleStruct for &'a mut BinarySerializer {
    type Ok = ();
    type Error = BinarySerializeError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> serde::ser::SerializeTupleVariant for &'a mut BinarySerializer {
    type Ok = ();
    type Error = BinarySerializeError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> serde::ser::SerializeMap for &'a mut BinarySerializer {
    type Ok = ();
    type Error = BinarySerializeError;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Self::Error> {
        key.serialize(&mut **self)
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        value.serialize(&mut **self)
    }

    fn serialize_entry<K: ?Sized + Serialize, V: ?Sized + Serialize>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error> {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> serde::ser::SerializeStruct for &'a mut BinarySerializer {
    type Ok = ();
    type Error = BinarySerializeError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> serde::ser::SerializeStructVariant for &'a mut BinarySerializer {
    type Ok = ();
    type Error = BinarySerializeError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'de> BinaryDeserializer<'de> {
    pub fn new(input: &'de [u8]) -> Self {
        Self { input, pos: 0 }
    }

    fn ensure_available(&self, n: usize) -> Result<(), BinarySerializeError> {
        if self.pos + n > self.input.len() {
            Err(BinarySerializeError(format!(
                "Unexpected EOF: need {} bytes, have {} at pos {}",
                n,
                self.input.len().saturating_sub(self.pos),
                self.pos
            )))
        } else {
            Ok(())
        }
    }

    fn read_u8(&mut self) -> Result<u8, BinarySerializeError> {
        self.ensure_available(1)?;
        let b = self.input[self.pos];
        self.pos += 1;
        Ok(b)
    }

    fn read_exact<const N: usize>(&mut self) -> Result<[u8; N], BinarySerializeError> {
        self.ensure_available(N)?;
        let mut buf = [0u8; N];
        buf.copy_from_slice(&self.input[self.pos..self.pos + N]);
        self.pos += N;
        Ok(buf)
    }

    fn read_u16(&mut self) -> Result<u16, BinarySerializeError> {
        Ok(u16::from_le_bytes(self.read_exact::<2>()?))
    }
    fn read_u32(&mut self) -> Result<u32, BinarySerializeError> {
        Ok(u32::from_le_bytes(self.read_exact::<4>()?))
    }
    fn read_u64(&mut self) -> Result<u64, BinarySerializeError> {
        Ok(u64::from_le_bytes(self.read_exact::<8>()?))
    }
    fn read_u128(&mut self) -> Result<u128, BinarySerializeError> {
        Ok(u128::from_le_bytes(self.read_exact::<16>()?))
    }
    fn read_i8(&mut self) -> Result<i8, BinarySerializeError> {
        Ok(self.read_u8()? as i8)
    }
    fn read_i16(&mut self) -> Result<i16, BinarySerializeError> {
        Ok(i16::from_le_bytes(self.read_exact::<2>()?))
    }
    fn read_i32(&mut self) -> Result<i32, BinarySerializeError> {
        Ok(i32::from_le_bytes(self.read_exact::<4>()?))
    }
    fn read_i64(&mut self) -> Result<i64, BinarySerializeError> {
        Ok(i64::from_le_bytes(self.read_exact::<8>()?))
    }
    fn read_i128(&mut self) -> Result<i128, BinarySerializeError> {
        Ok(i128::from_le_bytes(self.read_exact::<16>()?))
    }
    fn read_f32(&mut self) -> Result<f32, BinarySerializeError> {
        Ok(f32::from_le_bytes(self.read_exact::<4>()?))
    }
    fn read_f64(&mut self) -> Result<f64, BinarySerializeError> {
        Ok(f64::from_le_bytes(self.read_exact::<8>()?))
    }

    fn read_string(&mut self) -> Result<String, BinarySerializeError> {
        let len = self.read_u32()? as usize;
        self.ensure_available(len)?;
        let s = std::str::from_utf8(&self.input[self.pos..self.pos + len])
            .map_err(|e| BinarySerializeError(format!("Invalid UTF-8: {}", e)))?;
        self.pos += len;
        Ok(s.to_string())
    }

    fn read_bytes(&mut self, len: usize) -> Result<&'de [u8], BinarySerializeError> {
        // 返回切片引用需要在输入生命周期内，不改变所有权
        // 为简单起见，返回一个切片到原输入（但注意 pos 递增）
        self.ensure_available(len)?;
        let start = self.pos;
        self.pos += len;
        Ok(&self.input[start..start + len])
    }
}

impl<'de, 'a> serde::Deserializer<'de> for &'a mut BinaryDeserializer<'de> {
    type Error = BinarySerializeError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(BinarySerializeError("deserialize_any not supported".into()))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let b = self.read_u8()?;
        visitor.visit_bool(b != 0)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u8(self.read_u8()?)
    }
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u16(self.read_u16()?)
    }
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u32(self.read_u32()?)
    }
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u64(self.read_u64()?)
    }
    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u128(self.read_u128()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i8(self.read_i8()?)
    }
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i16(self.read_i16()?)
    }
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i32(self.read_i32()?)
    }
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i64(self.read_i64()?)
    }
    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i128(self.read_i128()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f32(self.read_f32()?)
    }
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f64(self.read_f64()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let b = self.read_u8()?;
        visitor.visit_char(b as char)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let s = self.read_string()?;
        visitor.visit_string(s)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // 约定：先读一个 u32 长度，再读这么多字节
        let len = self.read_u32()? as usize;
        let b = self.read_bytes(len)?;
        visitor.visit_borrowed_bytes(b)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // 对应 serialize_some 写入 1 + value；建议 serialize_none 写入 0
        let tag = self.read_u8()?;
        match tag {
            0 => visitor.visit_none(),
            1 => visitor.visit_some(&mut *self),
            _ => Err(BinarySerializeError(format!("Invalid option tag {}", tag))),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(&mut *self)
    }

    // 序列：前置 u32 长度
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let len = self.read_u32()? as usize;
        struct SeqAccessImpl<'a, 'de> {
            de: &'a mut BinaryDeserializer<'de>,
            remaining: usize,
        }
        impl<'de, 'a> serde::de::SeqAccess<'de> for SeqAccessImpl<'a, 'de> {
            type Error = BinarySerializeError;
            fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
            where
                T: serde::de::DeserializeSeed<'de>,
            {
                if self.remaining == 0 {
                    return Ok(None);
                }
                self.remaining -= 1;
                let val = seed.deserialize(&mut *self.de)?;
                Ok(Some(val))
            }

            fn size_hint(&self) -> Option<usize> {
                Some(self.remaining)
            }
        }

        visitor.visit_seq(SeqAccessImpl {
            de: self,
            remaining: len,
        })
    }

    // 固定长度数组/元组按元素顺序读取
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        struct TupleAccessImpl<'a, 'de> {
            de: &'a mut BinaryDeserializer<'de>,
            remaining: usize,
        }
        impl<'de, 'a> serde::de::SeqAccess<'de> for TupleAccessImpl<'a, 'de> {
            type Error = BinarySerializeError;
            fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
            where
                T: serde::de::DeserializeSeed<'de>,
            {
                if self.remaining == 0 {
                    return Ok(None);
                }
                self.remaining -= 1;
                let val = seed.deserialize(&mut *self.de)?;
                Ok(Some(val))
            }
        }
        // 让 visitor 驱动具体元素个数（serde 会用声明的元组长度消费）
        visitor.visit_seq(TupleAccessImpl {
            de: self,
            remaining: _len,
        })
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // 建议：如果序列化时写了长度，这里先读长度；你当前没有写长度，所以只能“未知长度”的访问。
        // 为避免无限读取，这里要求前置 u32 长度；如果确实没有，请按需更改逻辑。
        let len = self.read_u32()? as usize;

        struct MapAccessImpl<'a, 'de> {
            de: &'a mut BinaryDeserializer<'de>,
            remaining: usize,
        }
        impl<'de, 'a> serde::de::MapAccess<'de> for MapAccessImpl<'a, 'de> {
            type Error = BinarySerializeError;

            fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
            where
                K: serde::de::DeserializeSeed<'de>,
            {
                if self.remaining == 0 {
                    return Ok(None);
                }
                let key = seed.deserialize(&mut *self.de)?;
                Ok(Some(key))
            }

            fn next_value_seed<VV>(&mut self, seed: VV) -> Result<VV::Value, Self::Error>
            where
                VV: serde::de::DeserializeSeed<'de>,
            {
                // 读完一个 value，减少 remaining
                let v = seed.deserialize(&mut *self.de)?;
                self.remaining -= 1;
                Ok(v)
            }

            fn size_hint(&self) -> Option<usize> {
                Some(self.remaining)
            }
        }

        visitor.visit_map(MapAccessImpl {
            de: self,
            remaining: len,
        })
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // 按字段顺序直接消费即可
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        use serde::de::{DeserializeSeed, EnumAccess, IntoDeserializer, VariantAccess, Visitor};

        // 读取序列化时写入的变体索引（u32，小端）
        let variant_index = self.read_u32()?;

        // 构造 EnumAccess，把索引交给 visitor，由 visitor 决定具体变体
        struct EA<'a, 'de> {
            de: &'a mut BinaryDeserializer<'de>,
            index: u32,
        }
        struct VA<'a, 'de> {
            de: &'a mut BinaryDeserializer<'de>,
        }

        impl<'de, 'a> EnumAccess<'de> for EA<'a, 'de> {
            type Error = BinarySerializeError;
            type Variant = VA<'a, 'de>;

            fn variant_seed<VS>(self, seed: VS) -> Result<(VS::Value, Self::Variant), Self::Error>
            where
                VS: DeserializeSeed<'de>,
            {
                // 把索引转成反序列化器，交给 seed 解析成具体变体标识
                let v = seed.deserialize(self.index.into_deserializer())?;
                Ok((v, VA { de: self.de }))
            }
        }

        impl<'de, 'a> VariantAccess<'de> for VA<'a, 'de> {
            type Error = BinarySerializeError;

            fn unit_variant(self) -> Result<(), Self::Error> {
                // 单元变体：索引后无负载
                Ok(())
            }

            fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
            where
                T: DeserializeSeed<'de>,
            {
                // 新类型变体：索引后直接是值
                seed.deserialize(self.de)
            }

            fn tuple_variant<VT>(self, len: usize, visitor: VT) -> Result<VT::Value, Self::Error>
            where
                VT: Visitor<'de>,
            {
                // 元组变体：不写长度，由编译期 len 驱动
                serde::de::Deserializer::deserialize_tuple(self.de, len, visitor)
            }

            fn struct_variant<VT>(
                self,
                fields: &'static [&'static str],
                visitor: VT,
            ) -> Result<VT::Value, Self::Error>
            where
                VT: Visitor<'de>,
            {
                // 结构体变体：不写长度，按字段数量顺序读取
                serde::de::Deserializer::deserialize_tuple(self.de, fields.len(), visitor)
            }
        }

        visitor.visit_enum(EA {
            de: self,
            index: variant_index,
        })
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // 如果键是字符串：按字符串读取
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

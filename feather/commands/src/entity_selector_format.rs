use std::fmt;
use std::fmt::Display;
use std::ops::{AddAssign, MulAssign, Neg};

use serde::de::{DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor};
use serde::{de, ser, Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, SelectorError>;

#[derive(Clone, Debug, PartialEq)]
pub struct SelectorError(pub String);

impl ser::Error for SelectorError {
    fn custom<T: Display>(msg: T) -> Self {
        Self(msg.to_string())
    }
}

impl de::Error for SelectorError {
    fn custom<T: Display>(msg: T) -> Self {
        Self(msg.to_string())
    }
}

impl Display for SelectorError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for SelectorError {}

pub struct EntitySelectorSerializer {
    output: String,
}

#[allow(dead_code)]
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = EntitySelectorSerializer {
        output: String::new(),
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut EntitySelectorSerializer {
    type Ok = ();

    type Error = SelectorError;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.serialize_str(match v {
            true => "true",
            false => "false",
        })
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        let quote = v.contains(&[' ', '"', '\\'][..]);
        if quote {
            self.output += "\"";
        }
        self.output += &v.replace('"', "\\\"");
        if quote {
            self.output += "\"";
        }
        Ok(())
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<()> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_unit(self) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        variant.serialize(&mut *self)?;
        self.output += "=";
        value.serialize(&mut *self)?;
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.output += "[";
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.output += "{";
        variant.serialize(&mut *self)?;
        self.output += "=[";
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.output += "{";
        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.output += "{";
        variant.serialize(&mut *self)?;
        self.output += "={";
        Ok(self)
    }
}

impl<'a> ser::SerializeSeq for &'a mut EntitySelectorSerializer {
    type Ok = ();
    type Error = SelectorError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "]";
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut EntitySelectorSerializer {
    type Ok = ();
    type Error = SelectorError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "]";
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut EntitySelectorSerializer {
    type Ok = ();
    type Error = SelectorError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "]";
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut EntitySelectorSerializer {
    type Ok = ();
    type Error = SelectorError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "]}";
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut EntitySelectorSerializer {
    type Ok = ();
    type Error = SelectorError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('{') {
            self.output += ",";
        }
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.output += "=";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "}";
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut EntitySelectorSerializer {
    type Ok = ();
    type Error = SelectorError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('{') {
            self.output += ",";
        }
        key.serialize(&mut **self)?;
        self.output += "=";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "}";
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut EntitySelectorSerializer {
    type Ok = ();
    type Error = SelectorError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('{') {
            self.output += ",";
        }
        key.serialize(&mut **self)?;
        self.output += "=";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "}}";
        Ok(())
    }
}

pub struct EntitySelectorDeserializer<'de> {
    input: &'de str,
}

impl<'de> EntitySelectorDeserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        EntitySelectorDeserializer { input }
    }
}

pub fn from_str<'de, T>(s: &'de str) -> Result<T>
where
    T: Deserialize<'de>,
{
    let mut deserializer = EntitySelectorDeserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(SelectorError("Trailing characters".to_string()))
    }
}

impl<'de> EntitySelectorDeserializer<'de> {
    fn peek_char(&mut self) -> Result<char> {
        self.input
            .chars()
            .next()
            .ok_or_else(|| SelectorError("Unexpected EOF".to_string()))
    }

    fn next_char(&mut self) -> Result<char> {
        let ch = self.peek_char()?;
        self.input = &self.input[ch.len_utf8()..];
        Ok(ch)
    }

    fn parse_unsigned<T>(&mut self) -> Result<T>
    where
        T: AddAssign<T> + MulAssign<T> + From<u8>,
    {
        let mut int = match self.next_char()? {
            ch @ '0'..='9' => T::from(ch as u8 - b'0'),
            _ => {
                return Err(SelectorError("Expected integer".to_string()));
            }
        };
        loop {
            match self.input.chars().next() {
                Some(ch @ '0'..='9') => {
                    self.input = &self.input[1..];
                    int *= T::from(10);
                    int += T::from(ch as u8 - b'0');
                }
                _ => {
                    return Ok(int);
                }
            }
        }
    }

    fn parse_signed<T>(&mut self) -> Result<T>
    where
        T: Neg<Output = T> + AddAssign<T> + MulAssign<T> + From<i8>,
    {
        let neg = self.peek_char() == Ok('-');
        if neg {
            self.next_char()?;
        }
        let mut int = match self.next_char()? {
            ch @ '0'..='9' => T::from((ch as u8 - b'0') as i8),
            _ => {
                return Err(SelectorError("Expected integer".to_string()));
            }
        };
        loop {
            match self.input.chars().next() {
                Some(ch @ '0'..='9') => {
                    self.input = &self.input[1..];
                    int *= T::from(10);
                    int += T::from((ch as u8 - b'0') as i8);
                }
                _ => {
                    return Ok(if neg { -int } else { int });
                }
            }
        }
    }

    fn parse_string(&mut self) -> Result<&'de str> {
        if self.peek_char()? != '"' {
            if let Some(len) = self.input.find(&[',', ' ', ']', '}', '='][..]) {
                let s = &self.input[..len];
                self.input = &self.input[len..];
                return Ok(s);
            }
        } else {
            self.next_char()?;
            let mut escaping = false;
            for (i, ch) in self.input.chars().enumerate() {
                match ch {
                    '\\' => escaping = true,
                    '"' => {
                        if !escaping {
                            let res = Ok(&self.input[..i]);
                            self.input = &self.input[i + 1..];
                            return res;
                        }
                    }
                    _ => continue,
                }
            }
        }
        Err(SelectorError("Unexpected EOF".to_string()))
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut EntitySelectorDeserializer<'de> {
    type Error = SelectorError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.peek_char()? {
            'n' => self.deserialize_unit(visitor),
            '0'..='9' => self.deserialize_u64(visitor),
            '-' => self.deserialize_i64(visitor),
            '[' => self.deserialize_seq(visitor),
            '{' => self.deserialize_map(visitor),
            't' | 'f' => self.deserialize_bool(visitor),
            _ => self.deserialize_str(visitor),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.parse_string()? {
            "true" | "\"true\"" => visitor.visit_bool(true),
            "false" | "\"false\"" => visitor.visit_bool(false),
            something_else => visitor.visit_str(something_else),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.parse_signed()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.parse_signed()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.parse_signed()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.parse_signed()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.parse_unsigned()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.parse_unsigned()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.parse_unsigned()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.parse_unsigned()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        for (i, ch) in self.input.chars().enumerate() {
            match ch {
                '0'..='9' | '.' => continue,
                _ => {
                    let res = visitor.visit_f32(
                        self.input[..i]
                            .parse()
                            .map_err(|_| SelectorError("Invalid number".to_string()))?,
                    );
                    self.input = &self.input[i..];
                    return res;
                }
            }
        }
        Err(SelectorError("Unexpected EOF".to_string()))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        for (i, ch) in self.input.chars().enumerate() {
            match ch {
                '0'..='9' | '.' => continue,
                _ => {
                    let res = visitor.visit_f64(
                        self.input[..i]
                            .parse()
                            .map_err(|_| SelectorError("Invalid number".to_string()))?,
                    );
                    self.input = &self.input[i..];
                    return res;
                }
            }
        }
        Err(SelectorError("Unexpected EOF".to_string()))
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.parse_string()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.next_char()? == '[' {
            let value = visitor.visit_seq(CommaSeparated::new(&mut self))?;
            if self.next_char()? == ']' {
                Ok(value)
            } else {
                Err(SelectorError("Expected array end".to_string()))
            }
        } else {
            Err(SelectorError("Expected array".to_string()))
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.next_char()? == '{' {
            let value = visitor.visit_map(CommaSeparated::new(&mut self))?;
            if self.next_char()? == '}' {
                Ok(value)
            } else {
                Err(SelectorError("Expected map end".to_string()))
            }
        } else {
            Err(SelectorError("Expected map".to_string()))
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(Enum::new(self))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct CommaSeparated<'a, 'de: 'a> {
    de: &'a mut EntitySelectorDeserializer<'de>,
    first: bool,
}

impl<'a, 'de> CommaSeparated<'a, 'de> {
    fn new(de: &'a mut EntitySelectorDeserializer<'de>) -> Self {
        CommaSeparated { de, first: true }
    }
}

impl<'de, 'a> SeqAccess<'de> for CommaSeparated<'a, 'de> {
    type Error = SelectorError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.de.peek_char()? == ']' {
            return Ok(None);
        }
        if !self.first && self.de.next_char()? != ',' {
            return Err(SelectorError("Expected comma".to_string()));
        }
        self.first = false;
        seed.deserialize(&mut *self.de).map(Some)
    }
}

impl<'de, 'a> MapAccess<'de> for CommaSeparated<'a, 'de> {
    type Error = SelectorError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.de.peek_char()? == '}' {
            return Ok(None);
        }
        if !self.first && self.de.next_char()? != ',' {
            return Err(SelectorError("Expected comma".to_string()));
        }
        self.first = false;
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        if self.de.next_char()? != '=' {
            return Err(SelectorError("Expected =".to_string()));
        }
        seed.deserialize(&mut *self.de)
    }
}

struct Enum<'a, 'de: 'a> {
    de: &'a mut EntitySelectorDeserializer<'de>,
}

impl<'a, 'de> Enum<'a, 'de> {
    fn new(de: &'a mut EntitySelectorDeserializer<'de>) -> Self {
        Enum { de }
    }
}

impl<'de, 'a> EnumAccess<'de> for Enum<'a, 'de> {
    type Error = SelectorError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        let val = seed.deserialize(&mut *self.de)?;
        Ok((val, self))
    }
}

impl<'de, 'a> VariantAccess<'de> for Enum<'a, 'de> {
    type Error = SelectorError;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        if self.de.next_char()? != '=' {
            Err(SelectorError("Expected =".to_string()))
        } else {
            seed.deserialize(self.de)
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.de.next_char()? != '=' {
            Err(SelectorError("Expected =".to_string()))
        } else {
            de::Deserializer::deserialize_seq(self.de, visitor)
        }
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.de.next_char()? != '=' {
            Err(SelectorError("Expected =".to_string()))
        } else {
            de::Deserializer::deserialize_map(self.de, visitor)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arguments::{
        AdvancementPredicate, BoolPredicate, EntitySelectorPredicate, EntitySelectorSorting,
    };
    use std::collections::HashMap;

    #[test]
    fn test_serialize_struct() {
        #[derive(Serialize)]
        struct Test {
            x: f64,
            dx: f64,
            sort: EntitySelectorSorting,
        }

        let test = Test {
            x: 5.0,
            dx: 2.5,
            sort: EntitySelectorSorting::Nearest,
        };
        let expected = r#"{x=5,dx=2.5,sort=nearest}"#;
        assert_eq!(to_string(&test).unwrap(), expected);
    }

    #[test]
    fn test_serialize_enum() {
        #[derive(Serialize)]
        enum E {
            Newtype(u32),
            Tuple(u32, u32),
            Struct { a: u32 },
        }

        let n = E::Newtype(1);
        let expected = r#"Newtype=1"#;
        assert_eq!(to_string(&n).unwrap(), expected);

        let t = E::Tuple(1, 2);
        let expected = r#"{Tuple=[1,2]}"#;
        assert_eq!(to_string(&t).unwrap(), expected);

        let s = E::Struct { a: 1 };
        let expected = r#"{Struct={a=1}}"#;
        assert_eq!(to_string(&s).unwrap(), expected);

        let args = vec![
            EntitySelectorPredicate::Dx(1.5),
            EntitySelectorPredicate::Distance((1.5..=5.9).into()),
            EntitySelectorPredicate::Predicate(BoolPredicate(false, "my_predicate".to_string())),
            EntitySelectorPredicate::Level((0..=6).into()),
            EntitySelectorPredicate::Advancements(HashMap::from([
                ("test/1".to_string(), AdvancementPredicate::Value(true)),
                (
                    "test/2".to_string(),
                    AdvancementPredicate::Criteria(HashMap::from([("criteria".to_string(), true)])),
                ),
            ])),
        ];

        let expected = r#"[dx=1.5,distance=1.5..5.9,predicate=!my_predicate,level=0..6,advancements={test/1=true,test/2={criteria=true}}]"#;
        let expected2 = r#"[dx=1.5,distance=1.5..5.9,predicate=!my_predicate,level=0..6,advancements={test/2={criteria=true},test/1=true}]"#;
        assert!(to_string(&args).unwrap() == expected || to_string(&args).unwrap() == expected2);
    }

    #[test]
    fn test_deserialize_struct() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct Test {
            x: f64,
            dx: f64,
            sort: EntitySelectorSorting,
        }

        let j = r#"{x=5,dx=2.5,sort=nearest}"#;
        let expected = Test {
            x: 5.0,
            dx: 2.5,
            sort: EntitySelectorSorting::Nearest,
        };

        assert_eq!(expected, from_str(j).unwrap());
    }

    #[test]
    fn test_deserialize_enum() {
        #[derive(Deserialize, PartialEq, Debug)]
        enum E {
            Newtype(u32),
            Tuple(u32, u32),
            Struct { a: u32 },
        }

        let j = r#"Newtype=1"#;
        let expected = E::Newtype(1);
        assert_eq!(expected, from_str(j).unwrap());

        let j = r#"Tuple=[1,2]"#;
        let expected = E::Tuple(1, 2);
        assert_eq!(expected, from_str(j).unwrap());

        let j = r#""Struct"={a=1}"#;
        let expected = E::Struct { a: 1 };
        assert_eq!(expected, from_str(j).unwrap());

        let j = r#"[dx=1.5,distance=1.5..5.9,predicate=!my_predicate,level=0..6,advancements={test/1=true,test/2={criteria=true}}]"#;
        let expected: Vec<EntitySelectorPredicate> = vec![
            EntitySelectorPredicate::Dx(1.5),
            EntitySelectorPredicate::Distance((1.5..=5.9).into()),
            EntitySelectorPredicate::Predicate(BoolPredicate(false, "my_predicate".to_string())),
            EntitySelectorPredicate::Level((0..=6).into()),
            EntitySelectorPredicate::Advancements(HashMap::from([
                ("test/1".to_string(), AdvancementPredicate::Value(true)),
                (
                    "test/2".to_string(),
                    AdvancementPredicate::Criteria(HashMap::from([("criteria".to_string(), true)])),
                ),
            ])),
        ];

        assert_eq!(
            expected,
            from_str::<Vec<EntitySelectorPredicate>>(j).unwrap()
        );
    }
}

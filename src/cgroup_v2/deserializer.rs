use serde::de::{self, DeserializeSeed, MapAccess, Visitor};
use serde::Deserialize;

use std::ops::*;

use super::error::CGroupError;

type Error = CGroupError;
type Result<T> = std::result::Result<T, Error>;

trait IsIdentifier {
    fn is_identifier(&self) -> bool;
}

impl IsIdentifier for char {
    fn is_identifier(&self) -> bool {
        self.is_ascii_alphanumeric() || *self == '_'
    }
}

pub struct Deserializer<'de> {
    input: &'de str,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        Deserializer { input }
    }
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::new("Parse error: Trailing characters"))
    }
}

impl<'de> Deserializer<'de> {
    fn is_eof(&self) -> bool {
        self.input.len() == 0
    }

    fn peek_char(&self) -> Result<char> {
        self.input
            .chars()
            .next()
            .ok_or(Error::new("Parse error: EOF"))
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
            _ => return Err(Error::new("Parse error: Expected integer")),
        };
        loop {
            match self.peek_char() {
                Ok(ch @ '0'..='9') => {
                    self.next_char()?;
                    int *= T::from(10);
                    int += T::from(ch as u8 - b'0');
                }
                _ => {
                    return Ok(int);
                }
            }
        }
    }

    fn parse_string(&mut self) -> Result<&'de str> {
        if !self.peek_char()?.is_identifier() {
            return Err(Error::new("Parse error: Expected string"));
        }
        let res = &self.input[..];
        let mut len = 0;
        loop {
            match self.peek_char() {
                Ok(ch) if ch.is_identifier() => {
                    len += 1;
                    self.next_char()?;
                }
                _ => {
                    return Ok(&res[..len]);
                }
            }
        }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = CGroupError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        panic!("Deserialization of arbitrary type is not supported");
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
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

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
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

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
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
    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
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

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = visitor.visit_map(Accessor::new(self))?;
        Ok(value)
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
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
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
        // ignoring unknown field by read to end of line
        loop {
            let next = self.peek_char();
            if let Err(_) = next {
                break;
            }
            if let Ok('\n') = next {
                break;
            }
            self.next_char().unwrap();
        }
        visitor.visit_str("Ignored")
    }
}

struct Accessor<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    first: bool,
}

impl<'a, 'de> Accessor<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Self { de, first: true }
    }
}

impl<'de, 'a> MapAccess<'de> for Accessor<'a, 'de> {
    type Error = CGroupError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.de.is_eof() {
            return Ok(None);
        }
        if !self.first && self.de.next_char()? != '\n' {
            return Err(CGroupError::new(
                "Parse error: Expect newline as the delimiter",
            ));
        }
        self.first = false;
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        if self.de.next_char()? != ' ' {
            return Err(CGroupError::new(
                "Parse error: Expect whitespace as the delimiter",
            ));
        }
        seed.deserialize(&mut *self.de)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[test]
    fn test() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct CpuStat {
            usage_usec: u64,
            user_usec: u64,
            system_usec: u64,
        }

        let input = r#"usage_usec 9304127
user_usec 7523033
system_usec 1781093"#;
        let expected = CpuStat {
            usage_usec: 9304127,
            user_usec: 7523033,
            system_usec: 1781093,
        };
        assert_eq!(expected, from_str(input).unwrap());

        let input = r#"usage_usec 9304127
user_usec 7523033
system_usec 1781093
nr_periods 0
nr_throttled 0
throttled_usec 0"#;
        let expected = CpuStat {
            usage_usec: 9304127,
            user_usec: 7523033,
            system_usec: 1781093,
        };
        assert_eq!(expected, from_str(input).unwrap());
    }
}

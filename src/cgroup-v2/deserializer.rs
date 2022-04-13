use serde::Deserialize;

use super::cpu;
use super::error::CGroupError;

pub struct Deserializer<'de> {
    input: &'de str,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        Deserializer { input }
    }
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T, CGroupError>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(CGroupError::new("Parse error"))
    }
}

impl<'de> Deserializer<'de> {
    fn is_eof(&self) -> bool {
        self.input.length() == 0
    }

    fn peek_char(&self) -> Result<char> {
        self.input.chars().next().ok_or(Error::Eof)
    }

    fn next_char(&mut self) -> Result<char> {
        let ch = self.peek_char()?;
        self.input = &self.input[ch.len_utf8()..];
        Ok(ch)
    }

    fn parse_unsigned(&mut self) -> Result<T>
    where
        T: AddAssign<T> + MulAssign<T> + From<u8>,
    {
        let mut int = match self.next_char()? {
            ch @ '0'..='9' => T::from(ch as u8 - b'0'),
            _ => {
                return Err(Error::ExpectedInteger);
            }
        };
        loop {
            match self.peek_char() {
                Some(ch @ '0'..='9') => {
                    self.next_char();
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
        if !self.next_char()?.is_alphabetic() {
            return Err(Error::ExpectedString);
        }
        let res = &self.input[..];
        let len = 1;
        loop {
            match self.next_char() {
            Some(ch @ ch.is_alphabetic()) => {
                len += 1;
            }
            _ => {
                return Ok(res[..len]);
            },
        }
        }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = CGroupError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.parse_unsigned()?)
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
    
    fn deserialize_map<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
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
        deserialize_map(visitor)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }
}
struct Accessor<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    first: bool,
}

impl<'a, 'de> Accessor<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Self {
            de,
            first: true,
        }
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
            return Err(CGroupError::new("Parse error"));
        }
        self.first = false;
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        if self.de.next_char()? != ' ' {
            return Err(CGroupError::new("Parse error"));
        }
        seed.deserialize(&mut *self.de)
    }
}

mod tests{
#[test]
fn test() {
    #[derive(Deserialize, PartialEq, Debug)]
    enum Test {
        CpuStat {
            usage_usec: u64;
            user_usec: u64;
            system_usec: u64;
        }
    }

    let input = r#"usage_usec 9304127
user_usec 7523033
system_usec 1781093
nr_periods 0
nr_throttled 0
throttled_usec 0"#;
    let expected = Test::CpuStat { usage: 100, user: 60, system: 40};
    assert_eq!(expected, from_str(input));
}
}


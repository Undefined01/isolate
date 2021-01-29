use std::convert::{TryFrom, TryInto};
use std::num::{ParseIntError, TryFromIntError};
use std::time::Duration;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct KiB(u64);

impl KiB {
    pub fn from_byte(num: u64) -> Self {
        Self(num / 1024)
    }

    #[allow(non_snake_case)]
    pub fn from_KiB(num: u64) -> Self {
        Self(num)
    }

    pub fn try_from_str_byte(s: &str) -> Result<Self, ParseIntError> {
        let res = s.parse::<u64>()?;
        Ok(Self::from_byte(res))
    }

    pub fn byte(&self) -> u64 {
        self.0 * 1024
    }

    pub fn str_byte(&self) -> String {
        self.byte().to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd)]
pub struct ms(u64);

impl ms {
    pub fn from_millis(millis: u64) -> Self {
        Self(millis)
    }

    pub fn try_from_str_ns(s: &str) -> Result<Self, ParseIntError> {
        let res = s.parse::<u64>()?;
        Ok(ms(res / 1000000))
    }

    pub fn as_secs(&self) -> u64 {
        self.0 / 1000
    }
    
    pub fn as_millis(&self) -> u64 {
        self.0
    }
}

impl TryFrom<Duration> for ms {
    type Error = TryFromIntError;
    fn try_from(d: Duration) -> Result<Self, Self::Error> {
        d.as_millis().try_into().map(|x| Self(x))
    }
}

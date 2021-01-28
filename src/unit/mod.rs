use std::convert::{TryFrom, TryInto};
use std::num::{ParseIntError, TryFromIntError};
use std::time::Duration;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct KiB(pub u64);

impl KiB {
    pub fn try_from_str_byte(s: &str) -> Result<Self, ParseIntError> {
        let res = s.parse::<u64>()?;
        Ok(KiB(res / 1024))
    }

    pub fn str_byte(&self) -> String {
        (self.0 * 1024).to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd)]
pub struct ms(pub u64);

impl ms {
    pub fn try_from_str_ns(s: &str) -> Result<Self, ParseIntError> {
        let res = s.parse::<u64>()?;
        Ok(ms(res / 1000000))
    }
}

impl TryFrom<Duration> for ms {
    type Error = TryFromIntError;
    fn try_from(d: Duration) -> Result<Self, Self::Error> {
        d.as_millis().try_into().map(|x| Self(x))
    }
}

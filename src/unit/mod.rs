use std::convert::{TryFrom, TryInto};
use std::num::TryFromIntError;
use std::time::Duration;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Space(u64);

impl Space {
    pub fn from_byte(num: u64) -> Self {
        Self(num)
    }

    pub fn from_kib(num: u64) -> Self {
        Self(num * 1024)
    }

    pub fn from_mib(num: u64) -> Self {
        Self(num * 1024 * 1024)
    }

    pub fn as_bytes(&self) -> u64 {
        self.0
    }

    pub fn as_kib(&self) -> u64 {
        self.0 / 1024
    }

    pub fn as_mib(&self) -> u64 {
        self.0 / 1024 / 1024
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Time(u64);

impl Time {
    /// 从秒(s)构造
    pub fn from_secs(secs: u64) -> Self {
        Self(secs * 1000 * 1000)
    }

    /// 从毫秒(ms)构造
    pub fn from_millis(millis: u64) -> Self {
        Self(millis * 1000)
    }

    /// 从微秒(us)构造
    pub fn from_micros(micros: u64) -> Self {
        Self(micros)
    }

    pub fn as_secs(&self) -> u64 {
        self.0 / 1000 / 1000
    }

    pub fn as_millis(&self) -> u64 {
        self.0 / 1000
    }

    pub fn as_micros(&self) -> u64 {
        self.0
    }
}

impl TryFrom<Duration> for Time {
    type Error = TryFromIntError;
    fn try_from(d: Duration) -> Result<Self, Self::Error> {
        d.as_millis().try_into().map(|x| Self(x))
    }
}

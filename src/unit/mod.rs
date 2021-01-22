use std::num::ParseIntError;

pub struct KiB(u64);

impl KiB {
    pub fn try_from_str_byte(s: &str) -> Result<Self, ParseIntError> {
        let res = s.parse::<u64>()?;
        Ok(KiB(res / 1024))
    }
}

#[allow(non_camel_case_types)]
pub struct ms(u64);

impl ms {
    pub fn try_from_str_ns(s: &str) -> Result<Self, ParseIntError> {
        let res = s.parse::<u64>()?;
        Ok(ms(res / 1000000))
    }
}

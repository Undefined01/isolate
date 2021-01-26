use std::num::ParseIntError;

#[derive(Debug)]
pub struct KiB(pub u64);

impl KiB {
    pub fn try_from_str_byte(s: &str) -> Result<Self, ParseIntError> {
        let res = s.parse::<u64>()?;
        Ok(KiB(res / 1024))
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct ms(pub u64);

impl ms {
    pub fn try_from_str_ns(s: &str) -> Result<Self, ParseIntError> {
        let res = s.parse::<u64>()?;
        Ok(ms(res / 1000000))
    }
}

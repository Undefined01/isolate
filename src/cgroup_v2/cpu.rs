use super::deserializer;
use super::error::CGroupError;
use crate::unit::Time;

#[derive(Debug)]
pub struct CpuStat {
    usage: Time,
    user: Time,
    system: Time,
}

impl CpuStat {
    pub fn from_str(input: &str) -> Result<Self, CGroupError> {
        #[derive(Deserialize, PartialEq)]
        struct CpuStatFromVfs {
            usage_usec: u64,
            user_usec: u64,
            system_usec: u64,
        }

        let cpu_stat = deserializer::from_str(s)?;
        Ok(Self {
            usage: Time::from_micros(cpu_stat.usage_usec),
            user: Time::from_micros(cpu_stat.user_usec),
            system: Time::from_micros(cpu_stat.system_usec),
        })
    }
}

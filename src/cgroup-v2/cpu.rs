#[derive(Debug)]
pub struct CpuStat {
    usage: crate::unit::Time;
    user: crate::unit::Time;
    system: crate::unit::Time;
}

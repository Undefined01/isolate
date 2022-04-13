use crate::sys;
use crate::unit::*;
use nix::errno::Errno;
use nix::libc;

#[derive(Debug)]
pub struct ResourceLimit {
    pub stack_size: Space,
    pub mem_limit: Option<Space>,
    pub cpu_limit: Option<Time>,
    pub proc_limit: Option<u64>,
}

impl ResourceLimit {
    pub fn setrlimit(&self) -> Result<(), Errno> {
        let mut rlim: libc::rlimit;
        macro_rules! set {
            ($type: ident, $val: expr) => {
                debug!("Setting resource limit {} to {}", stringify!($type), $val);
                rlim = libc::rlimit {
                    rlim_cur: $val,
                    rlim_max: $val,
                };
                sys::setrlimit(libc::$type, &rlim)?;
            };
        }
        set!(RLIMIT_STACK, self.stack_size.as_bytes());
        set!(RLIMIT_CORE, 0);
        // set!(RLIMIT_FSIZE, 10);
        if let Some(cpu_limit) = &self.cpu_limit {
            // 计时有一定误差，多容忍 1 秒
            set!(RLIMIT_CPU, cpu_limit.as_secs() + 1);
        }
        if let Some(mem_limit) = &self.mem_limit {
            set!(RLIMIT_DATA, mem_limit.as_bytes());
        }
        if let Some(proc_limit) = &self.proc_limit {
            set!(RLIMIT_NPROC, *proc_limit);
        }
        Ok(())
    }
}

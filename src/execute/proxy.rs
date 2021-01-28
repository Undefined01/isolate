use crate::config::Config;
use nix::unistd::{getpid, setpgid};

pub fn entry(config: &Config) -> isize {
    let pid = getpid();

    // 创建进程组
    setpgid(pid, pid).expect("Failed to create process group");

    // 进入 control group
    config.cg.apply(pid).expect("Failed to enter control group");
    config.payload.exec().expect("Failed to execute payload");
    0
}

use crate::config::Config;
use nix::unistd::{getpid, setgid, setpgid, setuid, Gid, Uid};

pub fn entry(config: &Config) -> isize {
    let pid = getpid();

    // 创建进程组
    setpgid(pid, pid).expect("Failed to create process group");

    // 进入 control group
    config.cg.apply(pid).expect("Failed to enter control group");

    config
        .rlim
        .setrlimit()
        .expect("Failed to install resource limits");

    setgid(Gid::from_raw(65534)).expect("Failed to switch to nobody");
    setuid(Uid::from_raw(65534)).expect("Failed to switch to nobody");

    config.payload.exec().expect("Failed to execute payload");
    0
}

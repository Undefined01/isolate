use crate::config::Config;
use nix::unistd::getpid;

pub fn entry(config: &Config) -> isize {
    config.cg.apply(getpid()).expect("Failed to enter control group");
    config.payload.exec().expect("Failed to execute payload");
    0
}

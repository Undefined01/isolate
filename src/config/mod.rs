use crate::cgroup::Cgroup;
use crate::unit::{ms, KiB};

use nix::unistd::execvpe;
use std::ffi::CString;

#[derive(Debug)]
pub struct Config {
    pub cg: Cgroup,
    pub rlim: ResourceLimit,
    pub payload: Payload,
}

#[derive(Debug)]
pub struct ResourceLimit {
    pub stack_size: KiB,
    pub mem_limit: Option<KiB>,
    pub cpu_limit: Option<ms>,
}

#[derive(Debug)]
pub struct Payload {
    path: String,
    argv: Vec<String>,
    envp: Vec<String>,
}

impl Payload {
    pub fn new(path: String, argv: Vec<String>, envp: Vec<String>) -> Self {
        Self { path, argv, envp }
    }

    pub fn exec(&self) -> Result<(), nix::Error> {
        fn to_cstring(s: &String) -> CString {
            CString::new(s.as_bytes()).unwrap()
        }
        let path = to_cstring(&self.path);
        let argv = self.argv.iter().map(to_cstring).collect::<Vec<_>>();
        let envp = self.envp.iter().map(to_cstring).collect::<Vec<_>>();
        Err(execvpe(&path, &argv, &envp).unwrap_err())
    }
}

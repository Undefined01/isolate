use crate::cgroup::CGroup;
use crate::resourcelimit::ResourceLimit;

use nix::unistd::execvpe;
use std::ffi::CString;

#[derive(Debug)]
pub struct Config {
    pub cg: CGroup,
    pub rlim: ResourceLimit,
    pub payload: Payload,
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

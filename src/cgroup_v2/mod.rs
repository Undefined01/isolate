use nix::unistd::Pid;
use nix::sys::statfs::{CGROUP2_SUPER_MAGIC, statfs};
use std::fs;
use std::io;
use std::path;

mod deserializer;
mod error;

use error::CGroupError;

use crate::unit;

#[derive(Debug)]
pub struct CGroup {
    hierarchy: String,
}

impl CGroup {
    pub fn new<T: Into<String>>(hierarchy: T) -> Result<Self, CGroupError> {
        let hierarchy = hierarchy.into();
        Self::ensureCGroupV2(&hierarchy)?;
        Self {
            hierarchy
        }
    }

    pub fn ensureCGroupV2(path: &str) -> Result<(), CGroupError> {
        let info = statfs(path)?;
        if info.filesystem_type() != CGROUP2_SUPER_MAGIC {
            Err(CGroupError::new("\"{}\" is not mounted as cgroup v2"))
        } else {
            Ok(())
        }
    }
}

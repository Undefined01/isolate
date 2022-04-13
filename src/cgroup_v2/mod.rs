use nix::sys::statfs::{statfs, CGROUP2_SUPER_MAGIC};
use nix::unistd::Pid;
use std::fs;
use std::io;
use std::path;

mod cpu;
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
        Ok(Self { hierarchy })
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

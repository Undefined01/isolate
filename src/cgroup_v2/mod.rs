//! Control Group V2
//!
//! Based on cgroup2, a linux kernel feature to track and limit resources of process groups,
//! this module requires linux kernel >= 4.15.
//! See [kernel document](https://www.kernel.org/doc/Documentation/cgroup-v2.txt) for implement details.

use nix::sys::statfs::{statfs, CGROUP2_SUPER_MAGIC};
use nix::unistd::Pid;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

use crate::unit::Space;

mod cpu;
mod deserializer;
mod error;

use cpu::*;
use error::CGroupError;

#[derive(Debug)]
pub struct CGroup {
    hierarchy: String,
}

impl CGroup {
    pub fn from_path<T: Into<String>>(hierarchy: T) -> Result<Self, CGroupError> {
        let hierarchy = hierarchy.into();
        Self::ensure_cgroup_v2(&hierarchy)?;
        Ok(Self { hierarchy })
    }

    pub fn create<T: AsRef<str>>(root: T, subhierarchy: T) -> Result<Self, CGroupError> {
        Self::ensure_cgroup_v2(root.as_ref())?;
        let hierarchy = format!("{}/{}", root.as_ref(), subhierarchy.as_ref());
        if !Path::new(&hierarchy).exists() {
            fs::create_dir(&hierarchy)?;
        }
        Self::from_path(&hierarchy)
    }

    fn ensure_cgroup_v2(path: &str) -> Result<(), CGroupError> {
        let info = statfs(path)?;
        if info.filesystem_type() != CGROUP2_SUPER_MAGIC {
            Err(CGroupError::new("\"{}\" is not mounted as cgroup v2"))
        } else {
            Ok(())
        }
    }

    fn read(&self, file: &str) -> Result<String, io::Error> {
        let path = format!("{}/{}", self.hierarchy, file);
        fs::read_to_string(&path)
    }
    fn write(&self, file: &str, buf: &str) -> Result<(), io::Error> {
        let path = format!("{}/{}", self.hierarchy, file);
        fs::write(&path, buf.as_bytes())
    }
    fn append(&self, file: &str, buf: &str) -> Result<(), io::Error> {
        let path = format!("{}/{}", self.hierarchy, file);
        let mut f = fs::File::options().append(true).open(&path)?;
        f.write_all(buf.as_bytes())
    }

    pub fn add_proc(&self, pid: &Pid) -> Result<(), CGroupError> {
        self.append("cgroup.procs", &pid.to_string())?;
        Ok(())
    }

    pub fn get_proc(&self) -> Result<Vec<Pid>, CGroupError> {
        Ok(self
            .read("cgroup.procs")?
            .split('\n')
            .map(|x| Pid::from_raw(x.parse::<i32>().unwrap()))
            .collect())
    }

    pub fn cpu_stat(&self) -> Result<CpuStat, CGroupError> {
        CpuStat::from_str(&self.read("cpu.stat")?)
    }

    pub fn memory_current(&self) -> Result<Space, CGroupError> {
        Ok(Space::from_byte(
            self.read("memory.current")?.parse::<u64>().unwrap(),
        ))
    }

    pub fn set_memory_limit(&self, max: Space) -> Result<(), CGroupError> {
        self.write("memory.max", &max.as_bytes().to_string())?;
        Ok(())
    }
}

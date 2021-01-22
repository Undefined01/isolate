use std::fs;
use std::io;
use std::path;

use crate::unit;

#[derive(Debug)]
pub struct Cgroup {
    root: String,
    group: String,
}

impl Cgroup {
    pub fn new<T: Into<String>>(root: T, group: T) -> Self {
        Self {
            root: root.into(),
            group: group.into(),
        }
    }

    pub fn read(&self, subsystem: &str, attr: &str) -> Result<String, io::Error> {
        let file = format!("{}/{}/{}/{}", self.root, subsystem, self.group, attr);
        let res = fs::read_to_string(&file);
        debug!("file {}: {:?}", file, res);
        res
    }
    pub fn write(&self, subsystem: &str, attr: &str, buf: &str) -> Result<(), io::Error> {
        let file = format!("{}/{}/{}/{}", self.root, subsystem, self.group, attr);
        let res = fs::write(&file, buf.as_bytes());
        debug!("file {}: {:?}", file, res);
        res
    }

    pub fn init(&self, subsystems: &Vec<&str>) -> Result<(), io::Error> {
        for subsystem in subsystems.iter() {
            let p = format!("{}/{}/{}", self.root, subsystem, self.group);
            if path::Path::new(&p).exists() {
                debug!("{} already exists", p);
            } else {
                debug!("Creating {}", p);
                fs::create_dir(&p)?
            }
        }
        Ok(())
    }

    pub fn reset_cpu_usage(&self) -> Result<(), ()> {
        self.write("cpuacct", "cpuacct.usage", "0")
            .map_err(|e| info!("Fail to reset cpu usage: {:?}", e))
    }
    pub fn cpu_usage(&self) -> Result<unit::ms, ()> {
        let res = self
            .read("cpuacct", "cpuacct.usage")
            .map_err(|e| info!("Fail to read cpu usage: {:?}", e))?;
        unit::ms::try_from_str_ns(&res).map_err(|e| info!("Fail to read cpu usage: {:?}", e))
    }

    pub fn reset_mem_usage(&self) -> Result<(), ()> {
        self.write("memory", "memory.max_usage_in_bytes", "0")
            .map_err(|e| info!("Fail to reset memory usage: {:?}", e))
    }
    pub fn mem_usage(&self) -> Result<unit::KiB, ()> {
        let res = self
            .read("cpuacct", "cpuacct.usage")
            .map_err(|e| info!("Fail to read cpu usage: {:?}", e))?;
        unit::KiB::try_from_str_byte(&res).map_err(|e| info!("Fail to read cpu usage: {:?}", e))
    }
}

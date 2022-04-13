use nix::mount::{mount, MsFlags};

#[derive(Debug)]
pub struct Jail {
    pub workdir: String,
    pub mountpoints: Vec<MountPoint>,
}

impl Jail {
    pub fn apply(&self) -> Result<(), nix::Error> {
        for mountpoint in self.mountpoints.iter() {
            debug!(
                "Mouning {} to {} with flags {:x}",
                mountpoint.source.as_deref().unwrap_or("none"),
                mountpoint.target,
                mountpoint.flags.bits()
            );
            let target = format!("{}/{}", self.workdir, mountpoint.target);
            if !std::path::Path::new(&target).exists() {
                std::fs::create_dir(&target).map_err(|e| {
                    info!("Failed to create dir: {:?}", e);
                    nix::Error::last()
                })?
            }
            mount(
                mountpoint.source.as_deref(),
                target.as_str(),
                mountpoint.fstype.as_deref(),
                mountpoint.flags,
                mountpoint.data.as_deref(),
            )?;
            if mountpoint.flags.contains(MsFlags::MS_RDONLY) {
                mount(
                    <Option<&str>>::None,
                    target.as_str(),
                    mountpoint.fstype.as_deref(),
                    mountpoint.flags | MsFlags::MS_REMOUNT,
                    mountpoint.data.as_deref(),
                )?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct MountPoint {
    source: Option<String>,
    target: String,
    fstype: Option<String>,
    flags: MsFlags,
    data: Option<String>,
}

impl MountPoint {
    pub fn new(
        source: Option<String>,
        target: String,
        fstype: Option<String>,
        flags: MsFlags,
        data: Option<String>,
    ) -> Self {
        Self {
            source,
            target,
            fstype,
            flags,
            data,
        }
    }
}

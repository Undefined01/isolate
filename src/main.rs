use env_logger;
use isolate;

fn main() {
    env_logger::init();

    let mp = {
        use isolate::MountPoint;
        use nix::mount::MsFlags;
        vec![
            MountPoint::new(
                Some("/bin".into()),
                "bin".into(),
                None,
                MsFlags::MS_BIND | MsFlags::MS_RDONLY | MsFlags::MS_NOSUID | MsFlags::MS_NODEV,
                None,
            ),
            MountPoint::new(
                Some("/lib".into()),
                "lib".into(),
                Some("none".into()),
                MsFlags::MS_BIND
                    | MsFlags::MS_REC
                    | MsFlags::MS_RDONLY
                    | MsFlags::MS_NOSUID
                    | MsFlags::MS_NODEV,
                None,
            ),
            MountPoint::new(
                Some("/lib64".into()),
                "lib64".into(),
                Some("none".into()),
                MsFlags::MS_BIND
                    | MsFlags::MS_REC
                    | MsFlags::MS_RDONLY
                    | MsFlags::MS_NOSUID
                    | MsFlags::MS_NODEV,
                None,
            ),
        ]
    };
    isolate::Jail {
        workdir: "/var/local/lib/isolate/0/root".into(),
        mountpoints: mp,
    }
    .apply()
    .expect("Failed to mount jail");

    // let cg = isolate::CGroup::new("/sys/fs/cgroup", "box1");
    // cg.init().expect("Failed to create control groups");
    // let rlim = isolate::ResourceLimit {
    //     stack_size: isolate::unit::KiB::from_KiB(8 * 1024),
    //     cpu_limit: Some(isolate::unit::ms::from_millis(10000)),
    //     mem_limit: Some(isolate::unit::KiB::from_KiB(100000)),
    //     proc_limit: Some(10),
    // };
    // let payload = isolate::Payload::new(
    //     "bash".into(),
    //     vec!["bash".into(), "-c".into(), "testprogram/a.out 10000".into()],
    //     vec!["PATH=/bin:/usr/bin".into()],
    // );
    // let config = isolate::Config { cg, rlim, payload };

    // isolate::run(&config).expect("Failed to execute");
}

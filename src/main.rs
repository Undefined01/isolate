use env_logger;
use isolate;

fn main() {
    env_logger::init();

    let cg = isolate::Cgroup::new("/sys/fs/cgroup", "box1");
    cg.init().expect("Failed to create control groups");
    let rlim = isolate::ResourceLimit {
        stack_size: isolate::unit::KiB(8 * 1024),
    };
    let payload = isolate::Payload::new(
        "echo".into(),
        vec!["echo".into(), "hello!".into()],
        vec!["PATH=/bin:/usr/bin".into()],
    );
    let config = isolate::Config { cg, rlim, payload };

    isolate::run(&config).expect("Failed to execute");
}

use env_logger;
use isolate;

fn main() {
    env_logger::init();

    let cg = isolate::Cgroup::new("/sys/fs/cgroup", "box1");
    cg.init(&vec!["cpuacct", "memory"])
        .expect("Failed to create control groups");

    println!("Hello, world!");
}
